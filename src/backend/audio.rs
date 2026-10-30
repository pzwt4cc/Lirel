use super::ffmpeg;
use std::{
    fs,
    io::{BufRead, BufReader, Read},
    path::{Path, PathBuf},
    process::Stdio,
    thread,
};

#[derive(Clone, Copy, Debug)]
pub enum AudioFormat {
    Mp3,
    Wav,
    Flac,
    Ogg,
    Aac,
    Opus,
}

impl AudioFormat {
    pub fn from_index(i: usize) -> Self {
        match i {
            0 => Self::Mp3,
            1 => Self::Wav,
            2 => Self::Flac,
            3 => Self::Ogg,
            4 => Self::Aac,
            _ => Self::Opus,
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Self::Mp3 => "mp3",
            Self::Wav => "wav",
            Self::Flac => "flac",
            Self::Ogg => "ogg",
            Self::Aac => "m4a",
            Self::Opus => "opus",
        }
    }

    fn codec_args(self) -> &'static [&'static str] {
        match self {
            Self::Mp3 => &["-c:a", "libmp3lame", "-q:a", "2"],
            Self::Wav => &["-c:a", "pcm_s16le"],
            Self::Flac => &["-c:a", "flac"],
            Self::Ogg => &["-c:a", "libvorbis", "-q:a", "5"],
            Self::Aac => &["-c:a", "aac", "-b:a", "192k"],
            Self::Opus => &["-c:a", "libopus", "-b:a", "128k"],
        }
    }
}

#[derive(Debug)]
pub enum ConvertError {
    FfmpegMissing,
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

pub fn convert(
    input: &Path,
    out_dir: &Path,
    format: AudioFormat,
    mut on_progress: impl FnMut(f32),
) -> Result<PathBuf, ConvertError> {
    fs::create_dir_all(out_dir).map_err(|e| ConvertError::Failed(e.to_string()))?;

    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let output = unique_output(out_dir, stem, format.extension());
    let duration = ffmpeg::probe_duration(input);

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
        .arg("-vn")
        .args(format.codec_args())
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

    for line in BufReader::new(stdout).lines().map_while(Result::ok) {
        if let (Some(value), Some(total)) = (line.strip_prefix("out_time_us="), duration) {
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

    if status.success() {
        on_progress(1.0);
        Ok(output)
    } else {
        let _ = fs::remove_file(&output);
        Err(ConvertError::Failed(err_text.trim().to_string()))
    }
}
