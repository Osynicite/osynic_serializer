<!-- markdownlint-disable MD033 MD041 MD045 -->
<p align="center" dir="auto">
    <img style="height:240px;width:240px" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="Logo逃走啦~"/>
</p>

<p align="center">
  <h1 align="center">OsynicSerializer 🎵</h1>
  <p align="center">Rust编写的osu!谱面序列化工具，基于osynic_osudb开发</p>
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

[osynic_serializer](https://github.com/osynicite/osynic_serializer) 是一款高效的osu!谱面序列化工具，基于[osynic_osudb](https://github.com/osynicite/osynic_osudb)开发，支持FOLDER、OSUDB两种序列化算法。

![osynic_serializer.gif](https://s2.loli.net/2025/03/10/cwsgFnTEa76xiWQ.gif)

推荐搭配[osynic_downloader](https://github.com/osynicite/osynic_downloader)使用，可实现osu!谱面的快速序列化与多设备谱面同步。

![osynic_downloader.gif](https://s2.loli.net/2025/03/10/hasqOmgctyG4TWd.gif)

## ✨ 特性

- **双模式序列化**：支持直接序列化Songs目录或osu!.db文件
- **多输出格式**：支持序列化为sets或songs两种json格式
- **高效迅速**：速度极快，5000+谱面序列化完成仅需0.6s
- **差量过滤**：支持差量过滤，对比序列化结果与已有json文件，仅输出本地缺失谱面

## 📦 安装

### 预编译版本

```bash
cargo install osynic_serializer
```

### 源码编译

```bash
git clone https://github.com/osynicite/osynic_serializer
cd osynic_serializer
cargo build --release
```

## 🚀 快速开始

### 基本使用

```bash
# 直接序列化Songs文件夹到songs.json
osynic-sl -t songs -o ./songs

# 直接序列化osu!.db文件到sets.json
osynic-sl -t sets -p D:\\ProgramUnsigned\\Games\\OSU -o ./songs

# Diff序列化Songs文件夹，输出diffSets.json相较于本地多出来的谱面
osynic-sl -t sets -d json/diffSets.json -o ./songs

# Diff序列化osu!.db文件，输出diffSongs.json相较于本地多出来的谱面
osynic-sl -t songs -p D:\\ProgramUnsigned\\Games\\OSU -d json/diffSongs.json -o ./songs
```

## ⚙️ 参数详解

| 参数        | 简写 | 默认值           | 说明                                                      |
| ----------- | ---- | ---------------- | --------------------------------------------------------- |
| --algorithm | -a   | OSUDB            | osu!谱面序列化算法                                        |
| --json-type | -t   | songs            | json输出格式                                              |
| --path      | -p   | osu!默认安装目录 | osu!安装目录（或者 包含 Songs文件夹 或 osu!.db 的文件夹） |
| --diff      | -d   | -                | 差量过滤文件地址（ 文件的 json-type 需要保持一致）        |
| --output    | -o   | songs            | json存储目录（自动创建）                                  |
| --help      | -h   | -                | 显示帮助信息                                              |

## 支持的序列化算法

1. **FOLDER**：基于Songs文件夹序列化，从谱面文件夹名以及.osu文件中提取元数据，文件夹名不完整时可能存在不准确的情况
2. **OSUDB**：基于osu!.db文件序列化，最准确的序列化方式

### 输出JSON示例

**sets.json**（sets模式）:

```json
{
    "beatmapset_ids": ["114514", "1919810", "1538879"]
}
```

**songs.json**（songs模式）（Osynic）:

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

## 📌 注意事项

1. **osu!安装目录**：请确保osu!安装目录下存在Songs文件夹或osu!.db文件，如果osu!安装目录不在默认位置，请使用`--path`参数指定
2. **差量过滤**：差量过滤功能需要提供一个已有的json文件，用于对比序列化结果，仅输出本地缺失的谱面

## 🆗 作为库来使用

首先在你的`Cargo.toml`中添加依赖：

```toml
[dependencies]
osynic_serializer = "0.1.0"
```

最上层的方法都已位于`osynic_serializer::commands`模块中，所需类型均位于`osynic_serializer::types`模块中，而`osynic_serializer::functions`模块则是略底层的函数。

```rust
// 传入osu_dir,序列化Songs文件夹，得到Vec<SongWithMapper>
use osynic_serializer::commands::serialize_by_folder;
// 传入osu_dir,序列化osu!.db文件，得到Vec<SongWithMapper>
use osynic_serializer::commands::serialize_by_osudb;
// 对比新的Beatmapsets和已有的Beatmapsets，得到差量后的Beatmapsets
use osynic_serializer::commands::diff_sets;
// 对比新的SongsWithMapper和已有的SongsWithMapper，得到差量后的SongsWithMapper
use osynic_serializer::commands::diff_songs;
```

此外， [osynic_osudb](https://github.com/osynicite/osynic_osudb) 也已经重导出，可以直接通过本库引用：

```rust
use osynic_serializer::osynic_osudb::*;
```

## 🤝 贡献指南

这个库是差不多一个下午写完的，所以肯定还有很多地方需要改进，只是简单的把`osynic_core`里面的序列化功能拎出来了，很多地方还很生硬，但我暂时也懒得改了，更待良辰吉日改之

所以，如果代码有任何问题，或者你有任何建议，欢迎提交PR或者Issue，我会尽快处理~

如果你想贡献代码，请遵循以下规则：

- 遵循Rust官方编码规范
- 新增功能需附带测试用例
- 提交前运行`cargo fmt`和`cargo clippy`

## 📜 开源协议

本项目基于 [MIT License](LICENSE) 开源，请尊重原作者的著作权。使用osu!相关资源时请遵守[osu!社区准则](https://osu.ppy.sh/wiki/zh/Legal)。
