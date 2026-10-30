use super::{ffmpeg, tools::Plan};
use std::{
    fs,
    io::{BufRead, BufReader, Read},
    path::{Path, PathBuf},
    process::Stdio,
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

#[derive(Debug)]
pub enum ConvertError {
    FfmpegMissing,
    Cancelled,
    Failed(String),
}

fn unique_output(dir: &Path, stem: &str, ext: &str) -> PathBuf {
    let mut path = dir.join(format!("{stem}.{ext}"));
    let mut n = 1;
    while path.exists() {
        path = dir.join(format!("{stem} ({n}).{ext}"));
        n += 1;
    }
    path
}

pub fn run(
    input: &Path,
    out_dir: &Path,
    plan: &Plan,
    cancel: &AtomicBool,
    mut on_progress: impl FnMut(f32),
) -> Result<PathBuf, ConvertError> {
    fs::create_dir_all(out_dir).map_err(|e| ConvertError::Failed(e.to_string()))?;

    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let output = unique_output(out_dir, stem, plan.ext);
    let total = ffmpeg::probe_duration(input).map(|d| plan.limit.map_or(d, |l| d.min(l)));

    let mut child = ffmpeg::command()
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            "-nostdin",
            "-y",
            "-progress",
            "pipe:1",
            "-nostats",
            "-i",
        ])
        .arg(input)
        .args(&plan.args)
        .arg(&output)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                ConvertError::FfmpegMissing
            } else {
                ConvertError::Failed(e.to_string())
            }
        })?;

    let stdout = child.stdout.take().expect("stdout piped");
    let mut stderr = child.stderr.take().expect("stderr piped");

    let stderr_thread = thread::spawn(move || {
        let mut text = String::new();
        let _ = stderr.read_to_string(&mut text);
        text
    });

    let mut frames = 0u64;
    for line in BufReader::new(stdout).lines().map_while(Result::ok) {
        if cancel.load(Ordering::Relaxed) {
            let _ = child.kill();
            break;
        }
        if let Some(v) = line.strip_prefix("frame=") {
            frames = v.trim().parse().unwrap_or(frames);
        }
        if let (Some(value), Some(total)) = (line.strip_prefix("out_time_us="), total) {
            if let Ok(us) = value.trim().parse::<f64>() {
                if total > 0.0 {
                    on_progress(((us / 1_000_000.0) / total).clamp(0.0, 0.99) as f32);
                }
            }
        }
    }

    let status = child
        .wait()
        .map_err(|e| ConvertError::Failed(e.to_string()))?;
    let err_text = stderr_thread.join().unwrap_or_default();

    if cancel.load(Ordering::Relaxed) {
        let _ = fs::remove_file(&output);
        return Err(ConvertError::Cancelled);
    }
    let produced = fs::metadata(&output).map(|m| m.len() > 0).unwrap_or(false)
        && (!plan.needs_frame || frames > 0);
    if status.success() && produced {
        on_progress(1.0);
        Ok(output)
    } else if status.success() {
        let _ = fs::remove_file(&output);
        Err(ConvertError::Failed(
            "nothing was produced (time beyond video length?)".into(),
        ))
    } else {
        let _ = fs::remove_file(&output);
        Err(ConvertError::Failed(err_text.trim().to_string()))
    }
}
