use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::OnceLock,
};

#[cfg(windows)]
static FFMPEG_EXE: &[u8] = include_bytes!("../../assets/ffmpeg.exe");

pub fn path() -> &'static PathBuf {
    static PATH: OnceLock<PathBuf> = OnceLock::new();
    PATH.get_or_init(|| {
        #[cfg(windows)]
        {
            let dir = dirs::cache_dir()
                .unwrap_or_else(std::env::temp_dir)
                .join("lirel");
            let exe = dir.join("ffmpeg.exe");
            let up_to_date = std::fs::metadata(&exe)
                .map(|m| m.len() == FFMPEG_EXE.len() as u64)
                .unwrap_or(false);
            if !up_to_date {
                let _ = std::fs::create_dir_all(&dir);
                let tmp = dir.join("ffmpeg.exe.tmp");
                if std::fs::write(&tmp, FFMPEG_EXE).is_ok() {
                    let _ = std::fs::rename(&tmp, &exe);
                }
            }
            exe
        }
        #[cfg(not(windows))]
        {
            PathBuf::from("ffmpeg")
        }
    })
}

pub fn command() -> Command {
    #[allow(unused_mut)]
    let mut cmd = Command::new(path());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000);
    }
    cmd
}

pub fn probe_duration(input: &Path) -> Option<f64> {
    let out = command()
        .args(["-hide_banner", "-i"])
        .arg(input)
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&out.stderr);
    let t = text.split("Duration: ").nth(1)?.split(',').next()?;
    let mut parts = t.trim().split(':');
    let h: f64 = parts.next()?.parse().ok()?;
    let m: f64 = parts.next()?.parse().ok()?;
    let s: f64 = parts.next()?.parse().ok()?;
    Some(h * 3600.0 + m * 60.0 + s)
}
