<!-- markdownlint-disable MD033 MD041 MD045 MD023 MD036 MD022 MD032 MD031 -->
<div align="center">
  <img style="height:200px;width:200px" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="OsynicSerializer Logo"/>
  
  # OsynicSerializer 🎵
  
  **High-Performance osu! Beatmap Serialization Tool**
  
  Professional beatmap data processing tool built with Rust and osynic_osudb
  
  [![Rust Version](https://img.shields.io/badge/Rust-1.85%2B-blue?style=flat-square&logo=rust)](https://www.rust-lang.org/)
  [![Crates.io](https://img.shields.io/crates/v/osynic_serializer?style=flat-square&logo=rust)](https://crates.io/crates/osynic_serializer)
  [![Documentation](https://img.shields.io/docsrs/osynic_serializer?style=flat-square&logo=docs.rs)](https://docs.rs/osynic_serializer)
  [![License](https://img.shields.io/badge/License-MIT-green.svg?style=flat-square)](LICENSE)
  [![Build Status](https://img.shields.io/github/actions/workflow/status/osynicite/osynic_serializer/ci.yml?branch=master&style=flat-square&logo=github)](https://github.com/osynicite/osynic_serializer/actions)
  
  [中文版本](README.md) • [English Version](README_EN.md)
  
</div>

---

## 🎯 Overview

[osynic_serializer](https://github.com/osynicite/osynic_serializer) is a high-performance osu! beatmap serialization tool developed based on [osynic_osudb](https://github.com/osynicite/osynic_osudb). It supports both FOLDER and OSUDB serialization algorithms for fast extraction and processing of osu! beatmap data.

<div align="center">
  <img src="https://s2.loli.net/2025/03/10/cwsgFnTEa76xiWQ.gif" alt="OsynicSerializer Demo" style="max-width: 100%; border-radius: 8px;" />
</div>

### 🔗 Ecosystem

Recommended for use with [osynic_downloader](https://github.com/osynicite/osynic_downloader) to achieve a complete beatmap management and synchronization solution.

<div align="center">
  <img src="https://s2.loli.net/2025/03/10/hasqOmgctyG4TWd.gif" alt="OsynicDownloader Demo" style="max-width: 100%; border-radius: 8px;" />
</div>

## ✨ Core Features

### 🚀 **High-Performance Processing**
- ⚡ **Lightning-Fast Serialization**: Process 5000+ beatmaps in just 0.6 seconds
- 🔄 **Dual-Mode Support**: Process Songs directory or osu!.db file directly
- 📊 **Multiple Output Formats**: Support for both sets and songs JSON formats

### 🎯 **Smart Differential Processing**
- 🔍 **Incremental Sync**: Compare local vs remote beatmaps, output only differences
- 📈 **Detailed Statistics**: Comprehensive processing result statistics
- 🛡️ **Data Validation**: Automatic validation of input file format correctness

### 🛠️ **Developer-Friendly**
- 📚 **Library & CLI**: Use as both command-line tool and Rust library
- 🔧 **Flexible Configuration**: Rich command-line parameters and configuration options
- 📖 **Complete Documentation**: Detailed API documentation and usage examples

## 📦 Installation Guide

### Method 1: Precompiled Binary (Recommended)

Install directly using Cargo:

```bash
cargo install osynic_serializer
```

### Method 2: Build from Source

```bash
# Clone the repository
git clone https://github.com/osynicite/osynic_serializer
cd osynic_serializer

# Build release version
cargo build --release

# Optional: Install locally
cargo install --path .
```

### System Requirements

- **Rust**: 1.85.0 or higher
- **Operating System**: Windows / macOS / Linux
- **Memory**: 2GB+ available RAM recommended (for processing large collections)

## 🚀 Quick Start

### Basic Usage

#### 1. Songs Folder Serialization
```bash
# Serialize to songs.json format
osynic-sl -t songs -a FOLDER -o ./output

# Serialize to sets.json format  
osynic-sl -t sets -a FOLDER -o ./output
```

#### 2. osu!.db File Serialization
```bash
# Use specified osu! installation path
osynic-sl -t songs -p "D:\Games\osu!" -o ./output

# Use default osu! installation path (auto-detection)
osynic-sl -t songs -o ./output
```

#### 3. Differential Processing (Incremental Sync)
```bash
# Compare and output missing beatmap sets
osynic-sl -t sets -d "./remote/diffSets.json" -o ./output

# Compare and output missing individual beatmaps
osynic-sl -t songs -d "./remote/diffSongs.json" -o ./output
```

### Real-World Use Cases

#### Scenario 1: Export All Beatmap Data
```bash
# Export complete beatmap data for backup
osynic-sl -t songs -o ./backup
```

#### Scenario 2: Multi-Device Synchronization
```bash
# Device A: Export local beatmap list
osynic-sl -t sets -o ./sync

# Device B: Compare and download missing beatmaps
osynic-sl -t sets -d "./sync/sets_dm.json" -o ./to_download
```

## ⚙️ Command Line Parameters

| Parameter | Short | Default | Description |
|-----------|-------|---------|-------------|
| `--algorithm` | `-a` | `OSUDB` | Serialization algorithm selection |
| `--json-type` | `-t` | `songs` | Output JSON format type |
| `--path` | `-p` | *Auto-detect* | osu! installation directory path |
| `--diff` | `-d` | - | Differential comparison file path |
| `--output` | `-o` | `songs` | Output directory path |
| `--help` | `-h` | - | Display help information |

### Parameter Details

#### 🔧 `--algorithm` / `-a`
Choose the beatmap data extraction method:

- **`OSUDB`** (Recommended): Read from osu!.db file, most accurate data
- **`FOLDER`**: Scan from Songs folder, suitable when db file is unavailable

#### 📄 `--json-type` / `-t`
Specify the output JSON data format:

- **`songs`**: Detailed format with complete beatmap information
- **`sets`**: Compact format with only beatmapset IDs

#### 📁 `--path` / `-p`
Specify osu! installation directory, the program will automatically look for:
- `Songs/` folder (FOLDER mode)
- `osu!.db` file (OSUDB mode)

If not specified, the program will attempt auto-detection of common installation locations.

#### 🔄 `--diff` / `-d`
Provide comparison file for incremental processing:
- File format must match the `--json-type` parameter
- Output will be beatmap data missing locally

## 📊 Output Format Explanation

### Sets Format (`-t sets`)

Suitable for beatmapset management and batch downloading:

```json
{
  "beatmapset_ids": [
    "114514",
    "1919810", 
    "1538879"
  ]
}
```

### Songs Format (`-t songs`)

Contains detailed beatmap metadata:

```json
[
  {
    "song_id": 1985060,
    "artist_name": "ヒトリエ",
    "mapper_name": "flake",
    "song_name": "日常と地球の額縁 (wowaka x 初音ミク Edit)",
    "no_video": false
  },
  {
    "song_id": 1997071,
    "artist_name": "ナブナ",
    "mapper_name": "Ryuusei Aika", 
    "song_name": "始発とカフカ",
    "no_video": false
  }
]
```

## 🎯 Advanced Usage

### Batch Script Example

Create `sync.bat` for automated beatmap synchronization:

```batch
@echo off
echo Exporting local beatmap data...
osynic-sl -t sets -o ./export

echo Comparing with remote data...
osynic-sl -t sets -d "./remote/sets.json" -o ./missing

echo Sync complete! Check ./missing directory
pause
```

### PowerShell Script Example

```powershell
# Auto-detect and process multiple osu! installations
$osuPaths = @(
    "C:\Users\$env:USERNAME\AppData\Local\osu!",
    "D:\Games\osu!",
    "E:\osu!"
)

foreach ($path in $osuPaths) {
    if (Test-Path $path) {
        Write-Host "Processing path: $path"
        osynic-sl -t songs -p $path -o "./backup/$(Split-Path $path -Leaf)"
    }
}
```

## 📚 Using as a Library

OsynicSerializer is not only a command-line tool but also a complete Rust library.

### Adding Dependencies

Add to your `Cargo.toml`:

```toml
[dependencies]
osynic_serializer = { version = "0.1.2", default-features = false, features = ["cli"] }
```

### Core API

#### Main Command Functions

```rust
use osynic_serializer::commands::{
    serialize_by_folder,    // Serialize based on Songs folder
    serialize_by_osu_db,    // Serialize based on osu!.db file  
    diff_sets,              // Beatmapset differential comparison
    diff_songs,             // Beatmap differential comparison
};
```

#### Type Definitions

```rust
use osynic_serializer::types::{
    SongWithMapper,         // Individual beatmap information
    SongsWithMapper,        // Beatmap collection
    Beatmapsets,           // Beatmapset ID collection
};
```

### Usage Examples

#### Basic Serialization

```rust
use osynic_serializer::commands::{serialize_by_folder, serialize_by_osu_db};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Method 1: Serialize from Songs folder
    let songs_from_folder = serialize_by_folder("/path/to/osu/Songs")?;
    println!("Found {} beatmaps", songs_from_folder.songs.len());
    
    // Method 2: Serialize from osu!.db
    let songs_from_db = serialize_by_osu_db("/path/to/osu")?;
    println!("Database contains {} beatmaps", songs_from_db.songs.len());
    
    Ok(())
}
```

#### Differential Processing

```rust
use osynic_serializer::commands::{diff_songs, diff_sets};
use osynic_serializer::types::{SongsWithMapper, Beatmapsets};

fn sync_beatmaps() -> Result<(), Box<dyn std::error::Error>> {
    // Get local beatmap data
    let local_songs = serialize_by_osu_db("/path/to/osu")?;
    
    // Load remote beatmap data
    let remote_data = std::fs::read_to_string("remote_songs.json")?;
    let remote_songs: SongsWithMapper = serde_json::from_str(&remote_data)?;
    
    // Calculate difference
    let missing_songs = diff_songs(&remote_songs, &local_songs);
    println!("Need to download {} beatmaps", missing_songs.songs.len());
    
    Ok(())
}
```

#### Advanced Usage: Custom Processing Workflow

```rust
use osynic_serializer::functions::{
    check::{check_osu_dir, get_osu_dir},
    parse::parse_song_id_list_with_mapper,
    storage::marked_save_to,
};

fn custom_workflow() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Check osu! installation
    if !check_osu_dir() {
        eprintln!("osu! installation not found");
        return Ok(());
    }
    
    let osu_path = get_osu_dir();
    println!("Detected osu! installation: {}", osu_path);
    
    // 2. Serialize data
    let songs = serialize_by_osu_db(&osu_path)?;
    
    // 3. Extract beatmapset IDs
    let beatmapset_ids = parse_song_id_list_with_mapper(&songs.songs);
    
    // 4. Save results
    let json_data = serde_json::to_string_pretty(&beatmapset_ids)?;
    marked_save_to("./output", "beatmapsets.json", &json_data)?;
    
    Ok(())
}
```

### Re-exported Dependencies

You can directly use the underlying osu!.db parsing library:

```rust
// Directly use osynic_osudb functionality
use osynic_serializer::osynic_osudb::*;
```

## ❓ Frequently Asked Questions

### 🔧 Installation & Configuration

<details>
<summary><strong>Q: What to do when installation shows Rust version too low?</strong></summary>

```bash
# Update Rust to the latest version
rustup update stable

# Check version (requires 1.85.0+)
rustc --version
```
</details>

<details>
<summary><strong>Q: What to do when the program can't find osu! installation directory?</strong></summary>

Manually specify the osu! installation path:
```bash
osynic-sl -p "C:\Users\YourName\AppData\Local\osu!" -t songs -o ./output
```

Or specify any directory containing the `Songs` folder.
</details>

### 🚀 Usage Related

<details>
<summary><strong>Q: What's the difference between FOLDER and OSUDB modes?</strong></summary>

- **OSUDB Mode** (Recommended):
  - ✅ Most accurate data with complete metadata
  - ✅ Fast processing speed
  - ❌ Requires osu!.db file to exist

- **FOLDER Mode**:
  - ✅ No dependency on database file
  - ✅ Suitable for backup or corrupted installations
  - ❌ May have inaccurate data due to irregular folder naming
</details>

<details>
<summary><strong>Q: What does differential processing mean?</strong></summary>

Differential processing finds differences between two beatmap collections:
- Input: Local beatmaps + Remote beatmap list
- Output: Only beatmaps missing locally
- Purpose: Achieve incremental sync, avoid duplicate downloads
</details>

### 🐛 Troubleshooting

<details>
<summary><strong>Q: What to do when errors occur during serialization?</strong></summary>

1. **Permission Issues**: Ensure read permissions for osu! directory
2. **File Corruption**: Try using FOLDER mode to bypass corrupted .db files
3. **Memory Shortage**: Close other programs to free up memory
4. **Path Issues**: Use absolute paths instead of relative paths
</details>

<details>
<summary><strong>Q: Output JSON file is empty or incomplete data?</strong></summary>

Possible causes:
- osu! database file corrupted or outdated
- Songs folder empty or incorrect path
- Insufficient permissions to read files

Solutions:
```bash
# Try rebuilding database (press F5 in osu!)
# Or use verbose output to see error information
osynic-sl -t songs --verbose
```
</details>

## 🤝 Contributing

We welcome all forms of contributions! Whether it's code, documentation, bug reports, or feature suggestions.

### 🔧 Development Environment Setup

```bash
# 1. Fork and clone repository
git clone https://github.com/your-username/osynic_serializer
cd osynic_serializer

# 2. Install development dependencies
cargo build

# 3. Run tests
cargo test

# 4. Check code formatting
cargo fmt --check
cargo clippy -- -D warnings
```

### 📝 Submission Guidelines

- **Code Style**: Follow official Rust coding standards
- **Test Coverage**: New features require corresponding tests
- **Documentation Updates**: API changes need synchronized documentation updates
- **Commit Format**: Use clear commit messages

### 🐛 Bug Reports

When submitting bug reports, please include:
- Operating system and version
- Rust version
- Complete error messages
- Reproduction steps
- Related configuration files

### 💡 Feature Suggestions

We especially welcome improvements in the following areas:
- Performance optimization
- New output format support
- Better error handling
- Cross-platform compatibility improvements

## 🌟 Acknowledgments

Special thanks to the following projects and contributors:

- [osynic_osudb](https://github.com/osynicite/osynic_osudb) - Core osu!.db parsing library
- [osynic_downloader](https://github.com/osynicite/osynic_downloader) - Companion beatmap downloading tool
- osu! Community - Providing rich resources and support

And all developers who contributed code, reported issues, and made suggestions for this project!

## 📜 License

This project is open-sourced under the [MIT License](LICENSE).

### Terms of Use

- ✅ Commercial use
- ✅ Modification and distribution
- ✅ Private use
- ❗ Must retain copyright notice
- ❗ Must include license

### Related Resources

When using osu! related resources, please comply with:
- [osu! Terms of Service](https://osu.ppy.sh/legal/terms)
- [osu! Community Guidelines](https://osu.ppy.sh/wiki/en/Rules)

---

<div align="center">

**⭐ If this project helps you, please give us a Star!**

Made with ❤️ by [Osynicite](https://github.com/osynicite)

</div>
