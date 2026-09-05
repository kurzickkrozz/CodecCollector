# Codec Collector

<p align="center">
  <img src="CodecCollector.jpg" alt="Codec Collector Logo" width="180" height="180">
</p>

A portable, standalone Windows tool designed for Plex Media Server administrators to scan network shares (SMB/NAS), identify video stream codecs, calculate usage statistics, and export clean CSV reports.

## Features

* **Zero Dependencies:** Bundles `ffprobe` directly into the executable—no need to pre-install Python or FFmpeg on the target machine.
* **Network & UNC Share Support:** Seamlessly scans local drives as well as remote network paths (e.g., `\\192.168.1.50\media`).
* **Interactive Prompt:** Simply double-click the `.exe` and paste your target directory when prompted.
* **Terminal Dashboard:** Outputs an itemized list of media files grouped by codec, complete with a clean **Codec Statistics** breakdown showing overall percentages and a Grand Total.
* **Desktop CSV Export:** Automatically generates and saves a report named `codec-report.csv` directly onto the executing user's Desktop, ready for tools like PowerShell or alternative spreadsheet editors.

## Supported Video Formats
`mp4`, `mkv`, `avi`, `mov`, `webm`, `m4v`, `flv`, `wmv`

## Usage

1. Download the latest `CodecCollector.exe` release.
2. Double-click the executable to launch it.
3. Enter or paste your target directory path when prompted:
   ```text
   Enter the directory path to scan:

## Building From Source
If you want to modify the Python script and compile your own standalone binary with PyInstaller, use the following steps:
* Ensure you have Python and PyInstaller installed.
* Locate your local path to ffprobe.exe (e.g., via WinGet or an FFmpeg installation).
* Run the following PyInstaller compilation command:
  ```text
  pyinstaller --onefile --icon="path\to\CodecCollector.ico" --add-binary "C:\path\to\ffprobe.exe;." CodecCollector.py
* Find your newly packaged .exe inside the dist folder.
