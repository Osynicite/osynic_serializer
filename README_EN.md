<!-- markdownlint-disable MD033 MD041 MD045 -->
<p align="center" dir="auto">
    <img style="height:240px;width:240px" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="Logo逃走啦~"/>
</p>

<p align="center">
  <h1 align="center">OsynicSerializer 🎵</h1>
  <p align="center">A high-performance osu! beatmap serialization tool developed based on osynic_osudb</p>
</p>

<p align="center">
  <a href="https://www.rust-lang.org/" target="_blank"><img src="https://img.shields.io/badge/Rust-1.85%2B-blue"/></a>
  <a href="https://crates.io/crates/osynic_serializer" target="_blank"><img src="https://img.shields.io/crates/v/osynic_serializer"/></a>
  <a href="https://docs.rs/osynic_serializer" target="_blank"><img src="https://img.shields.io/docsrs/osynic_serializer/0.1.0"/></a>
  <a href="https://github.com/osynicite/osynic_serializer" target="_blank"><img src="https://img.shields.io/badge/License-MIT-green.svg"/></a>

</p>

<p align="center">
  <hr />

[中文版本](README.md) | [English Version](README_EN.md)

[osynic_serializer](https://github.com/osynicite/osynic_serializer) is a high-performance osu! beatmap serialization tool developed based on [osynic_osudb](https://github.com/osynicite/osynic_osudb), supporting FOLDER and OSUDB serialization algorithms.

![osynic_serializer.gif](https://s2.loli.net/2025/03/10/cwsgFnTEa76xiWQ.gif)

It is recommended to use [osynic_downloader](https://github.com/osynicite/osynic_downloader) to achieve fast serialization of osu! beatmaps and multi-device beatmap synchronization.

![osynic_downloader.gif](https://s2.loli.net/2025/03/10/hasqOmgctyG4TWd.gif)

## ✨ Features

- **Dual-mode serialization**: Supports direct serialization of the Songs directory or the osu!.db file
- **Multiple output formats**: Supports serialization to sets or songs in two JSON formats
- **Efficient and fast**: Extremely fast, serialization of 5000+ beatmaps takes only 0.6s
- **Incremental filtering**: Supports incremental filtering, comparing serialization results with existing JSON files, and outputting only locally missing beatmaps

## 📦 Installation

### Precompiled version

```bash
cargo install osynic_serializer
```

### Source code compilation

```bash
git clone https://github.com/osynicite/osynic_serializer
cd osynic_serializer
cargo build --release
```

## 🚀 Quick Start

### Basic usage

```bash
# Directly serialize the Songs folder to sets.json
osynic-sl -t sets -a FOLDER -o ./songs

# Directly serialize the osu!.db file to songs.json
osynic-sl -t songs -p D:\\ProgramUnsigned\\Games\\OSU -o ./songs

# Diff serialize the Songs folder, output diffSets.json compared to the locally missing beatmaps
osynic-sl -t sets -a FOLDER -d json/diffSets.json -o ./songs

# Diff serialize the osu!.db file, output diffSongs.json compared to the locally missing beatmaps
osynic-sl -t songs -p D:\\ProgramUnsigned\\Games\\OSU -d json/diffSongs.json -o ./songs
```

## ⚙️ Parameter Details

| Parameter   | Abbreviation | Default Value                       | Description                                                                           |
| ----------- | ------------ | ----------------------------------- | ------------------------------------------------------------------------------------- |
| --algorithm | -a           | OSUDB                               | osu! beatmap serialization algorithm                                                  |
| --json-type | -t           | songs                               | JSON output format                                                                    |
| --path      | -p           | osu! default installation directory | osu! installation directory ( or a folder containing the Songs folder or osu!.db)     |
| --diff      | -d           | -                                   | Incremental filtering file address (the json-type of the file needs to be consistent) |
| --output    | -o           | songs                               | JSON storage directory (automatically created)                                        |
| --help      | -h           | -                                   | Display help information                                                              |

## Supported Serialization Algorithms

1. **FOLDER**: Based on the Songs folder serialization, extract metadata from the beatmap folder name and .osu file, which may be inaccurate when the folder name is incomplete
2. **OSUDB**: Based on the osu!.db file serialization, the most accurate serialization method

### Output JSON example

**sets.json** (sets mode):

```json
{
    "beatmapset_ids": ["114514", "1919810", "1538879"]
}
```

**songs.json** (songs mode) (Osynic):

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

## 📌Notes

1. **osu! Installation Directory**: Please ensure that the Songs folder or osu!.db file exists in the osu! installation directory. If the osu! installation directory is not in the default location, please use the `--path` parameter to specify it
2. **Incremental Filtering**: The incremental filtering function requires a pre-existing JSON file for comparison serialization results, only outputting beatmaps that are locally missing

## 🆗 Use as a lib

First, add the dependency to your `Cargo.toml`:

```toml
[dependencies]
osynic_serializer = "0.1.0"
```

The top-level methods are all located in the `osynic_serializer::commands` module, the required types are all located in the `osynic_serializer::types` module, and the `osynic_serializer::functions` module is a slightly lower-level function.

```rust
// Pass in osu_dir, serialize the Songs folder, and get Vec<SongWithMapper>
use osynic_serializer::commands::serialize_by_folder;
// Pass in osu_dir, serialize the osu!.db file, and get Vec<SongWithMapper>
use osynic_serializer::commands::serialize_by_osudb;
// Compare new Beatmapsets with existing Beatmapsets, and get the differential Beatmapsets
use osynic_serializer::commands::diff_sets;
// Compare new SongsWithMapper with existing SongsWithMapper, and get the differential SongsWithMapper
use osynic_serializer::commands::diff_songs;
```

In addition, [osynic_osudb](https://github.com/osynicite/osynic_osudb) has also been re-exported and can be referenced directly through this library:

```rust
use osynic_serializer::osynic_osudb::*;
```

## 🤝 Contribution Guidelines

This library was written in almost an afternoon, so there are definitely many places that need to be improved. It just simply took out the serialization function in `osynic_core`, and many places are still very rigid, but I am currently too lazy to optimize it, and I will wait for a better day to do it

So, if there is any problem with the code, or if you have any suggestions, please feel free to submit a PR or Issue, and I will deal with it as soon as possible~

If you want to contribute code, please follow these rules:

- Follow the official Rust coding specifications
- New features must be accompanied by test cases
- Run `cargo fmt` and `cargo clippy` before submitting

## 📜 License

This project is open-sourced under the [MIT License](LICENSE), please respect the original author's copyright. When using osu! related resources, please abide by the [osu! Community Guidelines](https://osu.ppy.sh/wiki/zh/Legal).
