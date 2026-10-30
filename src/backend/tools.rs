//! Описание всех конверторов: форматы, настройки и аргументы ffmpeg.
//! Чтобы добавить новый конвертор: вариант в `Tool`, запись в `ALL`
//! и ветки в `title`, `tile`, `formats`, `options`, `input_exts`, `plan`.

/// Текст на двух языках: (русский, английский).
pub type L = (&'static str, &'static str);

pub struct Opt {
    pub title: L,
    pub choices: &'static [L],
    pub default: usize,
}

pub struct Plan {
    pub ext: &'static str,
    /// Аргументы между входным и выходным файлом.
    pub args: Vec<String>,
    /// Если результат ограничен по длительности (секунды).
    pub limit: Option<f64>,
    /// Результат — один кадр: если ffmpeg ни одного не выдал, это ошибка.
    pub needs_frame: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tool {
    VideoAudio = 0,
    Photo = 1,
    Video = 2,
    Audio = 3,
    Gif = 4,
    Compress = 5,
    VideoPhoto = 6,
}

// ---------- расширения входных файлов ----------
const VIDEO_EXT: &[&str] = &[
    "mp4", "mkv", "avi", "mov", "webm", "flv", "wmv", "m4v", "mpg", "mpeg", "ts", "3gp",
];
const AUDIO_EXT: &[&str] = &["mp3", "wav", "flac", "ogg", "m4a", "opus", "aac", "wma", "aiff"];
const ALL_MEDIA_EXT: &[&str] = &[
    "mp4", "mkv", "avi", "mov", "webm", "flv", "wmv", "m4v", "mpg", "mpeg", "ts", "3gp", "mp3",
    "wav", "flac", "ogg", "m4a", "opus", "aac", "wma", "aiff",
];
const PHOTO_EXT: &[&str] = &[
    "jpg", "jpeg", "png", "webp", "avif", "jxl", "gif", "bmp", "tif", "tiff", "ico", "tga", "ppm",
    "pgm", "pcx", "svg", "psd", "heic", "heif",
];

// ---------- форматы ----------
const AUDIO_FORMATS: &[&str] = &["MP3", "WAV", "FLAC", "OGG", "AAC", "OPUS"];
const PHOTO_FORMATS: &[&str] = &["JPG", "PNG", "WEBP", "AVIF", "JXL", "GIF", "BMP", "TIFF", "ICO"];
const FRAME_FORMATS: &[&str] = &["JPG", "PNG", "WEBP"];
const VIDEO_FORMATS: &[&str] = &["MP4", "MKV", "WEBM", "MOV", "AVI"];
const GIF_FORMATS: &[&str] = &["GIF", "WEBP"];
const COMPRESS_FORMATS: &[&str] = &["MP4 (H.264)", "MP4 (H.265)", "WEBM"];

// ---------- настройки ----------
const KBPS: [u32; 5] = [96, 128, 192, 256, 320];

const O_BITRATE: Opt = Opt {
    title: ("Битрейт", "Bitrate"),
    choices: &[
        ("96 кбит/с", "96 kbps"),
        ("128 кбит/с", "128 kbps"),
        ("192 кбит/с", "192 kbps"),
        ("256 кбит/с", "256 kbps"),
        ("320 кбит/с", "320 kbps"),
    ],
    default: 2,
};
const O_RATE: Opt = Opt {
    title: ("Частота дискретизации", "Sample rate"),
    choices: &[("Как в оригинале", "Original"), ("44.1 кГц", "44.1 kHz"), ("48 кГц", "48 kHz")],
    default: 0,
};
const O_CHANNELS: Opt = Opt {
    title: ("Каналы", "Channels"),
    choices: &[("Как в оригинале", "Original"), ("Моно", "Mono"), ("Стерео", "Stereo")],
    default: 0,
};
const O_PHOTO_QUALITY: Opt = Opt {
    title: ("Качество", "Quality"),
    choices: &[
        ("Максимальное", "Maximum"),
        ("Высокое", "High"),
        ("Среднее", "Medium"),
        ("Низкое", "Low"),
    ],
    default: 1,
};
const O_PHOTO_SIZE: Opt = Opt {
    title: ("Размер", "Size"),
    choices: &[
        ("Как в оригинале", "Original"),
        ("75%", "75%"),
        ("50%", "50%"),
        ("25%", "25%"),
    ],
    default: 0,
};
const O_VIDEO_QUALITY: Opt = Opt {
    title: ("Качество", "Quality"),
    choices: &[("Высокое", "High"), ("Среднее", "Medium"), ("Низкое", "Low")],
    default: 0,
};
const O_RESOLUTION: Opt = Opt {
    title: ("Разрешение", "Resolution"),
    choices: &[
        ("Как в оригинале", "Original"),
        ("1080p", "1080p"),
        ("720p", "720p"),
        ("480p", "480p"),
    ],
    default: 0,
};
const O_SOUND: Opt = Opt {
    title: ("Звук", "Sound"),
    choices: &[("Оставить", "Keep"), ("Убрать", "Remove")],
    default: 0,
};
const O_GIF_FPS: Opt = Opt {
    title: ("Кадров в секунду", "Frames per second"),
    choices: &[("10", "10"), ("15", "15"), ("24", "24")],
    default: 1,
};
const O_GIF_WIDTH: Opt = Opt {
    title: ("Ширина", "Width"),
    choices: &[
        ("320 px", "320 px"),
        ("480 px", "480 px"),
        ("640 px", "640 px"),
        ("Как в оригинале", "Original"),
    ],
    default: 1,
};
const O_GIF_LEN: Opt = Opt {
    title: ("Длительность", "Duration"),
    choices: &[
        ("Весь ролик", "Whole video"),
        ("Первые 5 с", "First 5 s"),
        ("Первые 10 с", "First 10 s"),
        ("Первые 30 с", "First 30 s"),
    ],
    default: 1,
};
const O_COMPRESS_LEVEL: Opt = Opt {
    title: ("Степень сжатия", "Compression"),
    choices: &[
        ("Лёгкое", "Light"),
        ("Среднее", "Medium"),
        ("Сильное", "Strong"),
        ("Максимальное", "Maximum"),
    ],
    default: 1,
};

const O_FRAME_AT: Opt = Opt {
    title: ("Момент", "Moment"),
    choices: &[
        ("Начало", "Start"),
        ("1 с", "1 s"),
        ("5 с", "5 s"),
        ("10 с", "10 s"),
        ("30 с", "30 s"),
        ("1 мин", "1 min"),
    ],
    default: 1,
};

const VIDEO_AUDIO_OPTS: &[Opt] = &[O_BITRATE];
const AUDIO_OPTS: &[Opt] = &[O_BITRATE, O_RATE, O_CHANNELS];
const PHOTO_OPTS: &[Opt] = &[O_PHOTO_QUALITY, O_PHOTO_SIZE];
const VIDEO_OPTS: &[Opt] = &[O_VIDEO_QUALITY, O_RESOLUTION, O_SOUND];
const GIF_OPTS: &[Opt] = &[O_GIF_FPS, O_GIF_WIDTH, O_GIF_LEN];
const COMPRESS_OPTS: &[Opt] = &[O_COMPRESS_LEVEL, O_RESOLUTION, O_SOUND];
const FRAME_OPTS: &[Opt] = &[O_FRAME_AT, O_PHOTO_QUALITY];

/// Разделы на главном экране.
pub const CATEGORIES: [L; 3] = [("Фото", "Photos"), ("Видео", "Video"), ("Аудио", "Audio")];

fn s(v: &[&str]) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}

fn pick(o: &[usize], i: usize, default: usize) -> usize {
    o.get(i).copied().unwrap_or(default)
}

impl Tool {
    pub const ALL: [Tool; 7] = [
        Tool::VideoAudio,
        Tool::Photo,
        Tool::Video,
        Tool::Audio,
        Tool::Gif,
        Tool::Compress,
        Tool::VideoPhoto,
    ];

    pub fn from_index(i: i32) -> Tool {
        Self::ALL.get(i.max(0) as usize).copied().unwrap_or(Tool::VideoAudio)
    }

    /// Индекс раздела в `CATEGORIES`.
    pub fn category(self) -> usize {
        match self {
            Tool::Photo | Tool::VideoPhoto => 0,
            Tool::VideoAudio | Tool::Video | Tool::Gif | Tool::Compress => 1,
            Tool::Audio => 2,
        }
    }

    pub fn title(self) -> L {
        match self {
            Tool::VideoAudio => ("Видео в аудио", "Video to audio"),
            Tool::Photo => ("Фото в любой формат", "Photo to any format"),
            Tool::Video => ("Видео", "Video"),
            Tool::Audio => ("Аудио", "Audio"),
            Tool::Gif => ("Видео в GIF", "Video to GIF"),
            Tool::Compress => ("Сжатие видео", "Compress video"),
            Tool::VideoPhoto => ("Кадр из видео", "Video frame"),
        }
    }

    /// Подпись на плитке: «из» и «во что».
    pub fn tile(self) -> (&'static str, &'static str) {
        match self {
            Tool::VideoAudio => ("MP4", "MP3"),
            Tool::Photo => ("ANY", "ANY"),
            Tool::Video => ("MP4", "MKV"),
            Tool::Audio => ("WAV", "FLAC"),
            Tool::Gif => ("MP4", "GIF"),
            Tool::Compress => ("MP4", "MP4 ↓"),
            Tool::VideoPhoto => ("MP4", "JPG"),
        }
    }

    pub fn formats(self) -> &'static [&'static str] {
        match self {
            Tool::VideoAudio | Tool::Audio => AUDIO_FORMATS,
            Tool::Photo => PHOTO_FORMATS,
            Tool::Video => VIDEO_FORMATS,
            Tool::Gif => GIF_FORMATS,
            Tool::Compress => COMPRESS_FORMATS,
            Tool::VideoPhoto => FRAME_FORMATS,
        }
    }

    pub fn options(self) -> &'static [Opt] {
        match self {
            Tool::VideoAudio => VIDEO_AUDIO_OPTS,
            Tool::Audio => AUDIO_OPTS,
            Tool::Photo => PHOTO_OPTS,
            Tool::Video => VIDEO_OPTS,
            Tool::Gif => GIF_OPTS,
            Tool::Compress => COMPRESS_OPTS,
            Tool::VideoPhoto => FRAME_OPTS,
        }
    }

    pub fn input_exts(self) -> &'static [&'static str] {
        match self {
            Tool::VideoAudio => ALL_MEDIA_EXT,
            Tool::Audio => AUDIO_EXT,
            Tool::Photo => PHOTO_EXT,
            Tool::Video | Tool::Gif | Tool::Compress | Tool::VideoPhoto => VIDEO_EXT,
        }
    }

    /// `fmt` — индекс формата, `o` — выбранные пункты по каждой группе настроек.
    pub fn plan(self, fmt: usize, o: &[usize]) -> Plan {
        match self {
            Tool::VideoAudio => audio_plan(fmt, pick(o, 0, 2), 0, 0),
            Tool::Audio => audio_plan(fmt, pick(o, 0, 2), pick(o, 1, 0), pick(o, 2, 0)),
            Tool::Photo => photo_plan(fmt, pick(o, 0, 1), pick(o, 1, 0)),
            Tool::Video => video_plan(fmt, pick(o, 0, 0), pick(o, 1, 0), pick(o, 2, 0) == 1),
            Tool::Gif => gif_plan(fmt, pick(o, 0, 1), pick(o, 1, 1), pick(o, 2, 1)),
            Tool::Compress => compress_plan(fmt, pick(o, 0, 1), pick(o, 1, 0), pick(o, 2, 0) == 1),
            Tool::VideoPhoto => frame_plan(fmt, pick(o, 0, 1), pick(o, 1, 1)),
        }
    }
}

fn audio_plan(fmt: usize, bitrate: usize, rate: usize, channels: usize) -> Plan {
    let k = format!("{}k", KBPS[bitrate.min(KBPS.len() - 1)]);
    let k = k.as_str();
    let (ext, codec): (&'static str, Vec<String>) = match fmt {
        0 => ("mp3", s(&["-c:a", "libmp3lame", "-b:a", k])),
        1 => ("wav", s(&["-c:a", "pcm_s16le"])),
        2 => ("flac", s(&["-c:a", "flac"])),
        3 => ("ogg", s(&["-c:a", "libvorbis", "-b:a", k])),
        4 => ("m4a", s(&["-c:a", "aac", "-b:a", k])),
        _ => ("opus", s(&["-c:a", "libopus", "-b:a", k])),
    };
    let mut args = s(&["-vn"]);
    args.extend(codec);
    // libopus сам работает только с 48 кГц и кратными, поэтому частоту для него не трогаем
    if ext != "opus" {
        match rate {
            1 => args.extend(s(&["-ar", "44100"])),
            2 => args.extend(s(&["-ar", "48000"])),
            _ => {}
        }
    }
    match channels {
        1 => args.extend(s(&["-ac", "1"])),
        2 => args.extend(s(&["-ac", "2"])),
        _ => {}
    }
    Plan { ext, args, limit: None, needs_frame: false }
}

/// Кодек и расширение для картинки. `kind` — название формата из списков выше.
fn image_codec(kind: &str, q: usize) -> (&'static str, Vec<String>) {
    let q = q.min(3);
    match kind {
        "JPG" => {
            let v = ["2", "4", "8", "15"][q];
            ("jpg", s(&["-c:v", "mjpeg", "-pix_fmt", "yuvj420p", "-q:v", v]))
        }
        "PNG" => ("png", s(&["-c:v", "png"])),
        "WEBP" => {
            let v = ["95", "85", "70", "50"][q];
            ("webp", s(&["-c:v", "libwebp", "-quality", v]))
        }
        "AVIF" => {
            let v = ["18", "26", "34", "42"][q];
            (
                "avif",
                s(&["-c:v", "libaom-av1", "-crf", v, "-b:v", "0", "-cpu-used", "6", "-pix_fmt", "yuv420p"]),
            )
        }
        "JXL" => {
            let v = ["0.5", "1.0", "2.0", "3.5"][q];
            ("jxl", s(&["-c:v", "libjxl", "-distance", v]))
        }
        "GIF" => ("gif", s(&["-c:v", "gif"])),
        "BMP" => ("bmp", s(&["-c:v", "bmp"])),
        "TIFF" => ("tiff", s(&["-c:v", "tiff"])),
        _ => ("ico", s(&["-c:v", "png"])),
    }
}

fn photo_plan(fmt: usize, quality: usize, size: usize) -> Plan {
    let kind = PHOTO_FORMATS[fmt.min(PHOTO_FORMATS.len() - 1)];
    let mut filters: Vec<String> = Vec::new();

    if size > 0 {
        let factor = [1.0, 0.75, 0.5, 0.25][size.min(3)];
        filters.push(format!("scale=trunc(iw*{factor}/2)*2:trunc(ih*{factor}/2)*2"));
    }
    if kind == "ICO" {
        // иконки бывают не больше 256×256, PNG внутри ICO должен быть RGBA
        filters.push("scale=min(256\\,iw):min(256\\,ih):force_original_aspect_ratio=decrease,format=rgba".into());
    }
    if kind == "JPG" {
        // JPEG без прозрачности: подкладываем белый фон вместо чёрного
        filters.push("split=2[bg][fg];[bg]drawbox=c=white:t=fill[bgw];[bgw][fg]overlay=format=auto".into());
    }

    let mut args = s(&["-frames:v", "1"]);
    if !filters.is_empty() {
        args.extend(["-vf".to_string(), filters.join(",")]);
    }
    let (ext, codec) = image_codec(kind, quality);
    args.extend(codec);
    Plan { ext, args, limit: None, needs_frame: false }
}

fn frame_plan(fmt: usize, moment: usize, quality: usize) -> Plan {
    let kind = FRAME_FORMATS[fmt.min(FRAME_FORMATS.len() - 1)];
    let at = ["0", "1", "5", "10", "30", "60"][moment.min(5)];
    let mut args = s(&["-ss", at, "-frames:v", "1"]);
    let (ext, codec) = image_codec(kind, quality);
    args.extend(codec);
    Plan { ext, args, limit: None, needs_frame: true }
}

fn height_filter(res: usize) -> Option<String> {
    let h = [0, 1080, 720, 480][res.min(3)];
    // `\,` — экранированная запятая внутри выражения фильтра
    (h > 0).then(|| format!("scale=-2:min({h}\\,ih)"))
}

fn video_plan(fmt: usize, quality: usize, res: usize, mute: bool) -> Plan {
    let q = quality.min(2);
    let mut args = Vec::new();
    if let Some(vf) = height_filter(res) {
        args.extend(["-vf".to_string(), vf]);
    }
    let (ext, codec): (&'static str, Vec<String>) = match fmt {
        0 | 1 | 3 => {
            let crf = ["20", "24", "29"][q];
            let mut c = s(&["-c:v", "libx264", "-preset", "medium", "-crf", crf, "-pix_fmt", "yuv420p"]);
            let ext = match fmt {
                0 => {
                    c.extend(s(&["-movflags", "+faststart"]));
                    "mp4"
                }
                1 => "mkv",
                _ => "mov",
            };
            if !mute {
                c.extend(s(&["-c:a", "aac", "-b:a", "160k"]));
            }
            (ext, c)
        }
        2 => {
            let crf = ["30", "36", "42"][q];
            let mut c = s(&["-c:v", "libvpx-vp9", "-crf", crf, "-b:v", "0", "-row-mt", "1", "-cpu-used", "4"]);
            if !mute {
                c.extend(s(&["-c:a", "libopus", "-b:a", "128k"]));
            }
            ("webm", c)
        }
        _ => {
            let v = ["3", "5", "8"][q];
            let mut c = s(&["-c:v", "mpeg4", "-q:v", v]);
            if !mute {
                c.extend(s(&["-c:a", "libmp3lame", "-b:a", "192k"]));
            }
            ("avi", c)
        }
    };
    args.extend(codec);
    if mute {
        args.push("-an".into());
    }
    Plan { ext, args, limit: None, needs_frame: false }
}

fn gif_plan(fmt: usize, fps: usize, width: usize, len: usize) -> Plan {
    let fps = [10, 15, 24][fps.min(2)];
    let mut filters = format!("fps={fps}");
    if let Some(w) = [Some(320), Some(480), Some(640), None][width.min(3)] {
        // `min(w, iw)` — не растягиваем маленькие ролики
        filters.push_str(&format!(",scale=min({w}\\,iw):-2:flags=lanczos"));
    }
    let mut args = s(&["-an"]);
    let ext = if fmt == 1 {
        args.extend(["-vf".to_string(), filters]);
        args.extend(s(&["-c:v", "libwebp", "-loop", "0", "-quality", "75"]));
        "webp"
    } else {
        filters.push_str(",split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse");
        args.extend(["-vf".to_string(), filters]);
        args.extend(s(&["-loop", "0"]));
        "gif"
    };
    let limit = [None, Some(5.0), Some(10.0), Some(30.0)][len.min(3)];
    if let Some(l) = limit {
        args.extend(["-t".to_string(), format!("{l}")]);
    }
    Plan { ext, args, limit, needs_frame: false }
}

fn compress_plan(fmt: usize, level: usize, res: usize, mute: bool) -> Plan {
    let l = level.min(3);
    let mut args = Vec::new();
    if let Some(vf) = height_filter(res) {
        args.extend(["-vf".to_string(), vf]);
    }
    let (ext, codec): (&'static str, Vec<String>) = match fmt {
        0 => {
            let crf = ["23", "27", "31", "35"][l];
            let mut c = s(&["-c:v", "libx264", "-preset", "medium", "-crf", crf, "-pix_fmt", "yuv420p", "-movflags", "+faststart"]);
            if !mute {
                c.extend(s(&["-c:a", "aac", "-b:a", "96k"]));
            }
            ("mp4", c)
        }
        1 => {
            let crf = ["26", "30", "34", "38"][l];
            let mut c = s(&["-c:v", "libx265", "-preset", "medium", "-crf", crf, "-tag:v", "hvc1", "-pix_fmt", "yuv420p", "-movflags", "+faststart"]);
            if !mute {
                c.extend(s(&["-c:a", "aac", "-b:a", "96k"]));
            }
            ("mp4", c)
        }
        _ => {
            let crf = ["32", "37", "42", "47"][l];
            let mut c = s(&["-c:v", "libvpx-vp9", "-crf", crf, "-b:v", "0", "-row-mt", "1", "-cpu-used", "4"]);
            if !mute {
                c.extend(s(&["-c:a", "libopus", "-b:a", "64k"]));
            }
            ("webm", c)
        }
    };
    args.extend(codec);
    if mute {
        args.push("-an".into());
    }
    Plan { ext, args, limit: None, needs_frame: false }
}