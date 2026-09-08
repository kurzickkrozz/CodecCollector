# Codec Collector

<p align="center">
  <img src="CodecCollector.ico" alt="Codec Collector Logo" width="180" height="180">
</p>

A high-performance, multithreaded Windows utility written in **Rust** designed for Plex Media Server administrators to scan network shares (SMB/NAS), identify video stream codecs accurately using `ffprobe`, calculate usage statistics, and export clean CSV reports. Originally, v1.0.0 through v1.2.0 were written in Python (and still work), but I've migrated over for **SPEED**. 

## Features

* **Blazing Fast Multithreading:** Powered by `rayon` to concurrently process massive media libraries across all available CPU cores.
* **Accurate Codec Resolution:** Leverages `ffprobe` beneath the hood to reliably parse stream metadata and normalize variant tags (e.g., `AVC`, `H.264`, and `H-264` map cleanly to `H264`).
* **Educational Codec Guide:** Automatically appends historical context, release years, and creator details for every detected format in your library.
* **Network & UNC Share Support:** Seamlessly scans local drives as well as remote network paths (e.g., `\\192.168.1.50\media`).
* **Interactive Prompt:** Simply double-click the `.exe` and paste your target directory when prompted.
* **Terminal Dashboard:** Outputs an itemized list of media files grouped by codec, a **Codec Statistics** percentage breakdown, historical codec information, and total execution time down to milliseconds.
* **Desktop CSV Export:** Automatically generates and saves a report named `codec-report.csv` directly onto the executing user's Desktop.

## Supported Video Formats
`mp4`, `mkv`, `avi`, `mov`, `webm`, `m4v`, `flv`, `wmv`

## Usage

1. Download the latest `CodecCollector.exe` release alongside a compatible `ffprobe.exe` binary in the same directory.
2. Double-click the executable to launch it.
3. Enter or paste your target directory path when prompted:
   ```text
   Enter the directory path to scan:
* Alternatively, you can run it via command-line arguments
  ```text
  .\CodecCollector.exe "\\192.168.1.50\media"

## Building From Source
If you want to modify the source code and compile your own optimized binary with Cargo, use the following steps:
1. Ensure you have the Rust toolchain installed.
2. Clone the repository and navigate into the project directory.
3. Run the release build command to optimize the executable for speed:
   ```text
   cargo build --release
   ```
4. Find your newly packaged .exe inside the dist folder.
