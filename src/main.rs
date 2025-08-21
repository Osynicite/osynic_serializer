#![cfg(feature = "tui")]

use clap::{Parser, ValueEnum};
use osynic_serializer::commands::{
    diff_sets, diff_songs, serialize_by_folder, serialize_by_osu_db,
};
use osynic_serializer::functions::check::{
    check_osu_dir, check_sets_type, check_songs_type, get_osu_dir,
};
use osynic_serializer::functions::parse::parse_song_id_list_with_mapper;
use osynic_serializer::functions::storage::marked_save_to;
use osynic_serializer::types::{Beatmapsets, SongWithMapper, SongsWithMapper};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, clap::ValueEnum)]
enum Algorithm {
    Osudb,
    Folder,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum JsonType {
    Songs,
    Sets,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long, default_value = "osudb")]
    algorithm: Algorithm,

    #[arg(short = 't', long, default_value = "songs")]
    json_type: JsonType,

    #[arg(short, long)]
    path: Option<PathBuf>,

    #[arg(short, long)]
    diff: Option<PathBuf>,

    #[arg(short, long, default_value = "songs")]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    let osu_dir = get_osu_directory(args.path)?;
    validate_diff_file(&args.diff, &args.json_type)?;

    let songs = match args.algorithm {
        Algorithm::Osudb => serialize_by_osu_db(osu_dir.to_str().unwrap_or_default()),
        Algorithm::Folder => serialize_by_folder(osu_dir.to_str().unwrap_or_default()),
    }?;

    let is_diff = args.diff.is_some();
    let json_type = args.json_type;
    match json_type {
        JsonType::Sets => {
            let sets = Beatmapsets {
                beatmapset_ids: parse_song_id_list_with_mapper(&songs.songs),
            };
            let result_data = process_diff_sets(sets, args.diff)?;
            println!(
                "Total beatmapsets after diff: {}",
                result_data.beatmapset_ids.len()
            );
            save_sets_data(is_diff, &args.output, &result_data, args.algorithm)?;
        }
        JsonType::Songs => {
            let result_data = process_diff_songs(songs, args.diff)?;
            println!("Total songs after diff: {}", result_data.songs.len());
            save_songs_data(is_diff, &args.output, &result_data, args.algorithm)?;
        }
    }

    Ok(())
}

fn get_osu_directory(user_path: Option<PathBuf>) -> Result<PathBuf, Box<dyn std::error::Error>> {
    user_path
        .or_else(|| check_osu_dir().then(|| PathBuf::from(get_osu_dir())))
        .ok_or("osu! path not found".into())
}

fn validate_diff_file(
    diff_path: &Option<PathBuf>,
    json_type: &JsonType,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = diff_path {
        let content = std::fs::read_to_string(path)?;
        let is_valid = match json_type {
            JsonType::Songs => check_songs_type(&content),
            JsonType::Sets => check_sets_type(&content),
        };

        if !is_valid {
            return Err("Invalid diff file".into());
        }
    }
    Ok(())
}

fn process_diff_sets(
    base_data: Beatmapsets,
    diff_path: Option<PathBuf>,
) -> Result<Beatmapsets, Box<dyn std::error::Error>> {
    match diff_path {
        Some(path) => {
            let diff_content = std::fs::read_to_string(path)?;
            let diff_data: Beatmapsets = serde_json::from_str(&diff_content)?;
            Ok(diff_sets(&base_data, &diff_data))
        }
        None => Ok(base_data),
    }
}

fn process_diff_songs(
    base_data: SongsWithMapper,
    diff_path: Option<PathBuf>,
) -> Result<SongsWithMapper, Box<dyn std::error::Error>> {
    match diff_path {
        Some(path) => {
            let diff_content = std::fs::read_to_string(path)?;
            let diff_data: Vec<SongWithMapper> = serde_json::from_str(&diff_content)?;
            let diff_data = SongsWithMapper { songs: diff_data };
            Ok(diff_songs(&base_data, &diff_data))
        }
        None => Ok(base_data),
    }
}

fn save_sets_data(
    is_diff: bool,
    output_dir: &Path,
    data: &Beatmapsets,
    algorithm: Algorithm,
) -> Result<(), Box<dyn std::error::Error>> {
    let diff_mark = if is_diff { "diff_" } else { "" };
    let filename = format!(
        "{}{}_{}.json",
        diff_mark,
        JsonType::Sets
            .to_possible_value()
            .unwrap_or_default()
            .get_name(),
        match algorithm {
            Algorithm::Osudb => "dm",
            Algorithm::Folder => "m",
        }
    );
    let json = serde_json::to_string_pretty(data)?;
    marked_save_to(output_dir.to_str().unwrap_or_default(), &filename, &json)?;
    Ok(())
}

fn save_songs_data(
    is_diff: bool,
    output_dir: &Path,
    data: &SongsWithMapper,
    algorithm: Algorithm,
) -> Result<(), Box<dyn std::error::Error>> {
    let diff_mark = if is_diff { "diff_" } else { "" };
    let filename = format!(
        "{}{}_{}.json",
        diff_mark,
        JsonType::Songs
            .to_possible_value()
            .unwrap_or_default()
            .get_name(),
        match algorithm {
            Algorithm::Osudb => "dm",
            Algorithm::Folder => "m",
        }
    );
    let json = serde_json::to_string_pretty(&data.songs)?;
    marked_save_to(output_dir.to_str().unwrap_or_default(), &filename, &json)?;
    Ok(())
}
