<!-- markdownlint-disable MD033 MD041 MD045 MD023 MD036 MD022 MD032 MD031 -->
<div align="center">
  <img style="height:200px;width:200px" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="OsynicSerializer Logo"/>
  
  # OsynicSerializer 🎵
  
  **高性能 osu! 谱面序列化工具**
  
  基于 Rust 和 osynic_osudb 开发的专业谱面数据处理工具
  
  [![Rust Version](https://img.shields.io/badge/Rust-1.85%2B-blue?style=flat-square&logo=rust)](https://www.rust-lang.org/)
  [![Crates.io](https://img.shields.io/crates/v/osynic_serializer?style=flat-square&logo=rust)](https://crates.io/crates/osynic_serializer)
  [![Documentation](https://img.shields.io/docsrs/osynic_serializer?style=flat-square&logo=docs.rs)](https://docs.rs/osynic_serializer)
  [![License](https://img.shields.io/badge/License-MIT-green.svg?style=flat-square)](LICENSE)
  [![Build Status](https://img.shields.io/github/actions/workflow/status/osynicite/osynic_serializer/ci.yml?branch=master&style=flat-square&logo=github)](https://github.com/osynicite/osynic_serializer/actions)
  
  [中文版本](README.md) • [English Version](README_EN.md)
  
</div>

---

## 🎯 概述

[osynic_serializer](https://github.com/osynicite/osynic_serializer) 是一款高效的 osu! 谱面序列化工具，基于 [osynic_osudb](https://github.com/osynicite/osynic_osudb) 开发。支持 FOLDER 和 OSUDB 两种序列化算法，能够快速提取并处理 osu! 谱面数据。

<div align="center">
  <img src="https://s2.loli.net/2025/03/10/cwsgFnTEa76xiWQ.gif" alt="OsynicSerializer Demo" style="max-width: 100%; border-radius: 8px;" />
</div>

### 🔗 生态系统

推荐与 [osynic_downloader](https://github.com/osynicite/osynic_downloader) 配合使用，实现完整的谱面管理和同步解决方案。

<div align="center">
  <img src="https://s2.loli.net/2025/03/10/hasqOmgctyG4TWd.gif" alt="OsynicDownloader Demo" style="max-width: 100%; border-radius: 8px;" />
</div>

## ✨ 核心特性

### 🚀 **高性能处理**
- ⚡ **极速序列化**：5000+ 谱面在 0.6 秒内完成处理
- 🔄 **双模式支持**：直接处理 Songs 目录或 osu!.db 文件
- 📊 **多格式输出**：支持 sets 和 songs 两种 JSON 格式

### 🎯 **智能差量处理**
- 🔍 **增量同步**：对比本地与远程谱面，仅输出差异部分
- 📈 **精确统计**：详细的处理结果统计信息
- 🛡️ **数据验证**：自动验证输入文件格式的正确性

### 🛠️ **开发友好**
- 📚 **库与CLI双用**：既可作为命令行工具，也可作为 Rust 库使用
- 🔧 **灵活配置**：丰富的命令行参数和配置选项
- 📖 **完整文档**：详细的API文档和使用示例

## 📦 安装指南

### 方式一：预编译版本（推荐）

使用 Cargo 直接安装：

```bash
cargo install osynic_serializer
```

### 方式二：源码编译

```bash
# 克隆仓库
git clone https://github.com/osynicite/osynic_serializer
cd osynic_serializer

# 编译发布版本
cargo build --release

# 可选：安装到本地
cargo install --path .
```

### 系统要求

- **Rust**: 1.85.0 或更高版本
- **操作系统**: Windows / macOS / Linux
- **内存**: 建议 2GB 以上可用内存（处理大量谱面时）

## 🚀 快速开始

### 基础用法

#### 1. Songs 文件夹序列化
```bash
# 序列化为 songs.json 格式
osynic-sl -t songs -a FOLDER -o ./output

# 序列化为 sets.json 格式  
osynic-sl -t sets -a FOLDER -o ./output
```

#### 2. osu!.db 文件序列化
```bash
# 使用指定 osu! 安装路径
osynic-sl -t songs -p "D:\Games\osu!" -o ./output

# 使用默认 osu! 安装路径（自动检测）
osynic-sl -t songs -o ./output
```

#### 3. 差量处理（增量同步）
```bash
# 对比并输出本地缺失的谱面集
osynic-sl -t sets -d "./remote/diffSets.json" -o ./output

# 对比并输出本地缺失的单个谱面
osynic-sl -t songs -d "./remote/diffSongs.json" -o ./output
```

### 实际使用场景

#### 场景1：导出所有谱面数据
```bash
# 导出完整的谱面数据用于备份
osynic-sl -t songs -o ./backup
```

#### 场景2：多设备同步
```bash
# 设备A：导出本地谱面列表
osynic-sl -t sets -o ./sync

# 设备B：对比并下载缺失谱面
osynic-sl -t sets -d "./sync/sets_dm.json" -o ./to_download
```

## ⚙️ 命令行参数详解

| 参数 | 简写 | 默认值 | 说明 |
|------|------|--------|------|
| `--algorithm` | `-a` | `OSUDB` | 序列化算法选择 |
| `--json-type` | `-t` | `songs` | 输出 JSON 格式类型 |
| `--path` | `-p` | *自动检测* | osu! 安装目录路径 |
| `--diff` | `-d` | - | 差量对比文件路径 |
| `--output` | `-o` | `songs` | 输出目录路径 |
| `--help` | `-h` | - | 显示帮助信息 |

### 参数详细说明

#### 🔧 `--algorithm` / `-a`
选择谱面数据的提取方式：

- **`OSUDB`** (推荐): 从 osu!.db 文件读取，数据最准确
- **`FOLDER`**: 从 Songs 文件夹扫描，适用于无 db 文件的情况

#### 📄 `--json-type` / `-t`
指定输出的 JSON 数据格式：

- **`songs`**: 包含完整谱面信息的详细格式
- **`sets`**: 仅包含谱面集 ID 的精简格式

#### 📁 `--path` / `-p`
指定 osu! 安装目录，程序会自动寻找：
- `Songs/` 文件夹（FOLDER 模式）
- `osu!.db` 文件（OSUDB 模式）

如不指定，程序会尝试自动检测常见安装位置。

#### 🔄 `--diff` / `-d`
提供对比文件进行增量处理：
- 文件格式必须与 `--json-type` 参数一致
- 输出结果为本地缺失的谱面数据

## 📊 输出格式说明

### Sets 格式 (`-t sets`)

适用于谱面集管理和批量下载：

```json
{
  "beatmapset_ids": [
    "114514",
    "1919810", 
    "1538879"
  ]
}
```

### Songs 格式 (`-t songs`)

包含详细的谱面元数据：

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

## 🎯 高级用法

### 批处理脚本示例

创建 `sync.bat` 用于自动化谱面同步：

```batch
@echo off
echo 正在导出本地谱面数据...
osynic-sl -t sets -o ./export

echo 正在对比远程数据...
osynic-sl -t sets -d "./remote/sets.json" -o ./missing

echo 同步完成！请查看 ./missing 目录
pause
```

### PowerShell 脚本示例

```powershell
# 自动检测并处理多个 osu! 安装
$osuPaths = @(
    "C:\Users\$env:USERNAME\AppData\Local\osu!",
    "D:\Games\osu!",
    "E:\osu!"
)

foreach ($path in $osuPaths) {
    if (Test-Path $path) {
        Write-Host "处理路径: $path"
        osynic-sl -t songs -p $path -o "./backup/$(Split-Path $path -Leaf)"
    }
}
```

## 📚 作为库使用

OsynicSerializer 不仅是命令行工具，也是功能完整的 Rust 库。

### 添加依赖

在 `Cargo.toml` 中添加：

```toml
[dependencies]
osynic_serializer = { version = "0.1.2", default-features = false, features = ["cli"] }
```

### 核心 API

#### 主要命令函数

```rust,no_run
use osynic_serializer::commands::{
    serialize_by_folder,    // 基于 Songs 文件夹序列化
    serialize_by_osu_db,    // 基于 osu!.db 文件序列化  
    diff_sets,              // 谱面集差量对比
    diff_songs,             // 谱面差量对比
};
```

#### 类型定义

```rust,no_run
use osynic_serializer::types::{
    SongWithMapper,         // 单个谱面信息
    SongsWithMapper,        // 谱面集合
    Beatmapsets,           // 谱面集 ID 集合
};
```

### 使用示例

#### 基础序列化

```rust,no_run
use osynic_serializer::commands::{serialize_by_folder, serialize_by_osu_db};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 方式1：从 Songs 文件夹序列化
    let songs_from_folder = serialize_by_folder("/path/to/osu/Songs")?;
    println!("找到 {} 个谱面", songs_from_folder.songs.len());
    
    // 方式2：从 osu!.db 序列化
    let songs_from_db = serialize_by_osu_db("/path/to/osu")?;
    println!("数据库中有 {} 个谱面", songs_from_db.songs.len());
    
    Ok(())
}
```

#### 差量处理

```rust,no_run
use osynic_serializer::commands::{diff_songs, diff_sets};
use osynic_serializer::types::{SongsWithMapper, Beatmapsets};

fn sync_beatmaps() -> Result<(), Box<dyn std::error::Error>> {
    // 获取本地谱面数据
    let local_songs = serialize_by_osu_db("/path/to/osu")?;
    
    // 加载远程谱面数据
    let remote_data = std::fs::read_to_string("remote_songs.json")?;
    let remote_songs: SongsWithMapper = serde_json::from_str(&remote_data)?;
    
    // 计算差量
    let missing_songs = diff_songs(&remote_songs, &local_songs);
    println!("需要下载 {} 个谱面", missing_songs.songs.len());
    
    Ok(())
}
```

#### 高级用法：自定义处理流程

```rust,no_run
use osynic_serializer::functions::{
    check::{check_osu_dir, get_osu_dir},
    parse::parse_song_id_list_with_mapper,
    storage::marked_save_to,
};

fn custom_workflow() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 检查 osu! 安装
    if !check_osu_dir() {
        eprintln!("未找到 osu! 安装");
        return Ok(());
    }
    
    let osu_path = get_osu_dir();
    println!("检测到 osu! 安装: {}", osu_path);
    
    // 2. 序列化数据
    let songs = serialize_by_osu_db(&osu_path)?;
    
    // 3. 提取谱面集 ID
    let beatmapset_ids = parse_song_id_list_with_mapper(&songs.songs);
    
    // 4. 保存结果
    let json_data = serde_json::to_string_pretty(&beatmapset_ids)?;
    marked_save_to("./output", "beatmapsets.json", &json_data)?;
    
    Ok(())
}
```

### 重导出的依赖

可以直接使用底层的 osu!.db 解析库：

```rust,no_run
// 直接使用 osynic_osudb 的功能
use osynic_serializer::osynic_osudb::*;
```

## ❓ 常见问题

### 🔧 安装与配置

<details>
<summary><strong>Q: 安装时提示 Rust 版本过低怎么办？</strong></summary>

```bash
# 更新 Rust 到最新版本
rustup update stable

# 检查版本（需要 1.85.0+）
rustc --version
```
</details>

<details>
<summary><strong>Q: 程序找不到 osu! 安装目录怎么办？</strong></summary>

手动指定 osu! 安装路径：
```bash
osynic-sl -p "C:\Users\YourName\AppData\Local\osu!" -t songs -o ./output
```

或者指定包含 `Songs` 文件夹的任意目录。
</details>

### 🚀 使用相关

<details>
<summary><strong>Q: FOLDER 和 OSUDB 模式有什么区别？</strong></summary>

- **OSUDB 模式**（推荐）：
  - ✅ 数据最准确，包含完整元数据
  - ✅ 处理速度快
  - ❌ 需要 osu!.db 文件存在

- **FOLDER 模式**：
  - ✅ 不依赖数据库文件
  - ✅ 适用于备份或损坏的安装
  - ❌ 可能因文件夹命名不规范导致数据不准确
</details>

<details>
<summary><strong>Q: 差量处理是什么意思？</strong></summary>

差量处理用于找出两个谱面集合之间的差异：
- 输入：本地谱面 + 远程谱面列表
- 输出：仅包含本地缺失的谱面
- 用途：实现增量同步，避免重复下载
</details>

### 🐛 故障排除

<details>
<summary><strong>Q: 序列化过程中出现错误怎么办？</strong></summary>

1. **权限问题**：确保对 osu! 目录有读取权限
2. **文件损坏**：尝试使用 FOLDER 模式绕过损坏的 .db 文件
3. **内存不足**：关闭其他程序释放内存
4. **路径问题**：使用绝对路径而非相对路径
</details>

<details>
<summary><strong>Q: 输出的 JSON 文件为空或数据不全？</strong></summary>

可能原因：
- osu! 数据库文件损坏或过旧
- Songs 文件夹为空或路径错误
- 权限不足无法读取文件

解决方案：
```bash
# 尝试重新构建数据库（在 osu! 中按 F5）
# 或使用详细输出查看错误信息
osynic-sl -t songs --verbose
```
</details>

## 🤝 贡献指南

我们欢迎各种形式的贡献！无论是代码、文档、Bug 报告还是功能建议。

### 🔧 开发环境搭建

```bash
# 1. Fork 并克隆仓库
git clone https://github.com/your-username/osynic_serializer
cd osynic_serializer

# 2. 安装开发依赖
cargo build

# 3. 运行测试
cargo test

# 4. 检查代码格式
cargo fmt --check
cargo clippy -- -D warnings
```

### 📝 提交规范

- **代码风格**：遵循 Rust 官方编码规范
- **测试覆盖**：新功能需要添加相应测试
- **文档更新**：API 变更需要同步更新文档
- **Commit 格式**：使用清晰的提交信息

### 🐛 问题报告

提交 Bug 报告时请包含：
- 操作系统和版本
- Rust 版本
- 完整的错误信息
- 复现步骤
- 相关的配置文件

### 💡 功能建议

我们特别欢迎以下方面的改进：
- 性能优化
- 新的输出格式支持
- 更好的错误处理
- 跨平台兼容性改进

## 🌟 致谢

特别感谢以下项目和贡献者：

- [osynic_osudb](https://github.com/osynicite/osynic_osudb) - 核心的 osu!.db 解析库
- [osynic_downloader](https://github.com/osynicite/osynic_downloader) - 配套的谱面下载工具
- osu! 社区 - 提供了丰富的资源和支持

以及所有为本项目贡献代码、报告问题、提出建议的开发者们！

## 📜 开源协议

本项目基于 [MIT License](LICENSE) 开源。

### 使用条款

- ✅ 商业使用
- ✅ 修改和分发
- ✅ 私人使用
- ❗ 需保留版权声明
- ❗ 需包含许可证

### 相关资源

使用 osu! 相关资源时，请遵守：
- [osu! 使用条款](https://osu.ppy.sh/legal/terms)
- [osu! 社区准则](https://osu.ppy.sh/wiki/zh/Rules)

---

<div align="center">

**⭐ 如果这个项目对你有帮助，请给我们一个 Star！**

Made with ❤️ by [Osynicite](https://github.com/osynicite)

</div>
