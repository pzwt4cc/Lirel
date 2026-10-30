# Lirel

A small free converter for photos and videos. Pick what you want to convert, choose files, press Convert.

Written in Rust with a Slint GUI. Made for myself, shared in case it's useful to someone else.

## What it can do

**Photos**
- Any format to any: JPG, PNG, WEBP, AVIF, JXL, GIF, BMP, TIFF, ICO
- Quality and size (100% / 75% / 50% / 25%) settings
- Video frame to JPG / PNG / WEBP

**Video**
- Video to audio
- MP4 / MKV / WEBM / MOV / AVI
- Video to GIF or animated WEBP
- Compress video (H.264, H.265, VP9)

**Audio**
- MP3, WAV, FLAC, OGG, AAC, OPUS

**Settings**
- Russian / English
- System, light or dark theme
- Accent color
- Output folder
- Optionally open the folder when the conversion is done

## Download

**Windows:** grab `Lirel-windows-x64.exe` from [Releases](../../releases) and run it. ffmpeg is built in, nothing else to install.

**Linux:** build it yourself (below). You need `ffmpeg` installed and in your PATH.

## Build

Linux:

```
git clone https://github.com/<your-name>/Lirel
cd Lirel
cargo build --release
```

Windows: put a static `ffmpeg.exe` (full GPL build, for example from
[BtbN](https://github.com/BtbN/FFmpeg-Builds/releases)) into `assets/` first, then run `cargo build --release`.
It gets embedded into the exe. The GitHub Actions workflow in `.github/workflows` does this automatically.

The binary will be in `target/release/`.

## License

Add one before publishing (MIT is a fine default). Note that the bundled Windows ffmpeg is a GPL build.
