use rayon::prelude::*;
use std::collections::BTreeMap;
use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;
use walkdir::WalkDir;

const VIDEO_EXTENSIONS: &[&str] = &[".mp4", ".mkv", ".avi", ".mov", ".webm", ".m4v", ".flv", ".wmv"];

fn get_codec_alias(raw: &str) -> String {
    let upper = raw.to_uppercase();
    match upper.as_str() {
        "MPEG1" | "MPEG1VIDEO" | "MPEG-1" => "H261".to_string(),
        "MPEG2" | "MPEG2VIDEO" | "MPEG-2" => "H262".to_string(),
        "MPEG-4" | "MPEG4" | "H263P" | "H263I" => "H263".to_string(),
        "AVC" | "H.264" | "H-264" => "H264".to_string(),
        "H.265" | "H265" | "H-265" => "HEVC".to_string(),
        "THEORA" => "VP3".to_string(),
        _ => upper,
    }
}

fn get_codec_info(codec: &str) -> &'static str {
    match codec {
        "AV1" => "Est. in 2018. Developed by AOMedia. Highly efficient for 4K, 8K, and HDR.",
        "H261" => "Est. in 1988. Known as 'MPEG-1'. Legacy format.",
        "H262" => "Est. in 1995. Known as 'MPEG-2'. Early DVD format.",
        "H263" => "Est. in 1995. Known as 'MPEG-4 Part 2'. Early VTC format.",
        "H264" => "Est. in 2003. Known as 'MPEG-4 Part 10' or 'AVC'. Most compatible.",
        "H266" => "Est. in 2020. Known as 'VVC'. Highest efficiency for 8K and 360* VR.",
        "HEVC" => "Est. in 2013. Known as 'H265'. Made to cut down size for 4K media.",
        "VP3" => "Est. in 2000. Known as 'Theora'. On2 Technologies' 'TrueMotion'.",
        "VP8" => "Est. in 2010. Google's standard for WebM & WebRTC web streaming.",
        "VP9" => "Est. in 2013. Heavily used for YouTube. Highly efficient for 4K.",
        "VC1" => "Est. in 2006. Used for HD-DVDs to compete against Blu-ray.",
        _ => "Custom or unlisted format.",
    }
}

fn get_ffprobe_path() -> String {
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(dir) = exe_path.parent() {
            let local = dir.join("ffprobe.exe");
            if local.exists() {
                return local.to_string_lossy().into_owned();
            }
        }
    }
    "ffprobe".to_string()
}

fn get_video_codec(path: &Path) -> String {
    let ffprobe = get_ffprobe_path();
    let path_str = path.to_string_lossy();

    let output = Command::new(&ffprobe)
        .args(&[
            "-v", "error",
            "-select_streams", "v:0",
            "-show_entries", "stream=codec_name",
            "-of", "default=noprint_wrappers=1:nokey=1",
            &path_str,
        ])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let stdout_str = String::from_utf8_lossy(&out.stdout);
            if let Some(first_line) = stdout_str.lines().next() {
                let codec = first_line.trim().to_uppercase();
                if codec.is_empty() {
                    "UNKNOWN".to_string()
                } else {
                    get_codec_alias(&codec)
                }
            } else {
                "UNKNOWN".to_string()
            }
        }
        _ => "UNKNOWN".to_string(),
    }
}

fn main() {
    let start_time = Instant::now();

    let args: Vec<String> = env::args().collect();
    let target_dir = if args.len() > 1 {
        args[1].clone()
    } else {
        print!("Enter the directory path to scan: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().trim_matches('"').to_string()
    };

    let root = Path::new(&target_dir);
    if !root.exists() {
        println!("Error: Directory does not exist.");
        return;
    }

    println!("\nScanning target at '{}' (Parallel Mode)...\n", root.display());

    let paths: Vec<PathBuf> = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .filter(|path| {
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext_dot = format!(".{}", ext.to_lowercase());
                    return VIDEO_EXTENSIONS.contains(&ext_dot.as_str());
                }
            }
            false
        })
        .collect();

    let total_files = paths.len();

    let results: Vec<(String, String)> = paths
        .par_iter()
        .map(|path| {
            let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
            let codec = get_video_codec(path);
            (codec, file_name)
        })
        .collect();

    let mut codec_dict: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (codec, file_name) in results {
        codec_dict.entry(codec).or_default().push(file_name);
    }

    println!("Script made by TunnelRat (aka Krozz)\n");
    for (codec, files) in &codec_dict {
        println!("Codec: {}", codec);
        println!("{}", "-".repeat(40));
        for file in files {
            println!("  {}", file);
        }
        println!();
    }

    if total_files > 0 {
        println!("Codec Statistics");
        println!("{}", "-".repeat(40));
        println!("  Grand Total: {}", total_files);
        for (codec, files) in &codec_dict {
            let count = files.len();
            let percentage = (count as f64 / total_files as f64) * 100.0;
            println!("  {}: {:.1}% ({})", codec, percentage, count);
        }
        println!();

        println!("Codec Information");
        println!("{}", "-".repeat(40));
        for codec in codec_dict.keys() {
            let info = get_codec_info(codec);
            println!("  {}: {}", codec, info);
        }
        println!();

        if let Some(desktop) = dirs::desktop_dir() {
            let csv_path = desktop.join("codec-report.csv");
            if let Ok(mut wtr) = csv::Writer::from_path(csv_path) {
                let _ = wtr.write_record(&["File Name", "Codec"]);
                for (codec, files) in &codec_dict {
                    for file in files {
                        let _ = wtr.write_record(&[file, codec]);
                    }
                }
                println!("[Success] CSV report saved to your Desktop named codec-report.csv");
            }
        }
    }

    let elapsed = start_time.elapsed();
    let total_ms = elapsed.as_millis();
    let days = total_ms / (1000 * 60 * 60 * 24);
    let hours = (total_ms / (1000 * 60 * 60)) % 24;
    let minutes = (total_ms / (1000 * 60)) % 60;
    let seconds = (total_ms / 1000) % 60;
    let millis = total_ms % 1000;

    println!("\nExecution Time: {} Days, {} Hours, {} Minutes, {} Seconds, {} milliseconds", days, hours, minutes, seconds, millis);

    println!("\nPress Enter to exit...");
    let _ = io::stdin().read_line(&mut String::new());
}