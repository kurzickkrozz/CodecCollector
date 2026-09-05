from collections import defaultdict
from pathlib import Path
import subprocess
import sys
import os
import csv

VIDEO_EXTENSIONS = {".mp4", ".mkv", ".avi", ".mov", ".webm", ".m4v", ".flv", ".wmv"}

def get_ffprobe_path():
    if getattr(sys, 'frozen', False) and hasattr(sys, '_MEIPASS'):
        return os.path.join(sys._MEIPASS, "ffprobe.exe")
    
    if not subprocess.run(["where", "ffprobe"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode:
        return "ffprobe"
    
    manual_path = Path("C:/ffmpeg/bin/ffprobe.exe")
    if manual_path.exists():
        return str(manual_path)
        
    return None

FFPROBE_PATH = get_ffprobe_path()

def get_video_codec(file_path):
    if not FFPROBE_PATH:
        print("\n[Error] Could not locate 'ffprobe.exe'.")
        sys.exit(1)

    path_str = str(Path(file_path).resolve())
    cmd = [
        FFPROBE_PATH,
        "-v", "error",
        "-select_streams", "v:0",
        "-show_entries", "stream=codec_name",
        "-of", "default=noprint_wrappers=1:nokey=1",
        path_str
    ]
    try:
        result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, check=True)
        codec = result.stdout.strip()
        return codec if codec else "unknown"
    except subprocess.SubprocessError:
        return "unknown"

def organize_videos(root_directory):
    if not FFPROBE_PATH:
        print("[Error] FFprobe is required but could not be found.")
        return

    codec_dict = defaultdict(list)
    root_path = Path(root_directory)

    if not root_path.exists():
        print(f"\nError: Directory '{root_directory}' does not exist or is unreachable.")
        return

    print(f"\nScanning network share at '{root_path.resolve()}'...\n")

    for file_path in root_path.rglob("*"):
        if file_path.is_file() and file_path.suffix.lower() in VIDEO_EXTENSIONS:
            codec = get_video_codec(file_path)
            codec_dict[codec].append(file_path.name)

    print("Program made and compiled by TunnelRat (aka Krozz)\n")

    for codec in sorted(codec_dict.keys()):
        print(f"Codec: {codec.upper()}")
        print("-" * 40)
        for file_name in sorted(codec_dict[codec]):
            print(f"  {file_name}")
        print()

    total_files = sum(len(files) for files in codec_dict.values())
    if total_files > 0:
        print("Codec Statistics")
        print("-" * 40)
        print(f"  Grand Total: {total_files}")
        for codec in sorted(codec_dict.keys()):
            count = len(codec_dict[codec])
            percentage = (count / total_files) * 100
            print(f"  {codec.upper()}: {percentage:.1f}% ({count})")

        # Export to Desktop CSV
        desktop_path = Path.home() / "Desktop"
        csv_filename = "codec-report.csv"
        csv_file_path = desktop_path / csv_filename
        
        try:
            with open(csv_file_path, mode="w", newline="", encoding="utf-8") as csv_file:
                writer = csv.writer(csv_file)
                writer.writerow(["File Name", "Codec"])
                for codec in sorted(codec_dict.keys()):
                    for file_name in sorted(codec_dict[codec]):
                        writer.writerow([file_name, codec.upper()])
            print(f'\n[Success] CSV report saved to your Desktop named "{csv_filename}"')
        except Exception as e:
            print(f"\n[Warning] Could not save CSV to Desktop: {e}")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        target_dir = sys.argv[1]
    else:
        target_dir = input("Enter the directory path to scan: ").strip()
        target_dir = target_dir.strip('"\'')
        
    organize_videos(target_dir)
    
    input("\nPress Enter to exit...")