use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug,Clone,Default)]
pub struct Song {
    pub song_id: u32,
    pub artist_name: String,
    pub song_name: String,
    pub no_video: bool,
}

#[derive(Serialize, Deserialize, Debug,Clone,Default)]
pub struct SongWithMapper {
    pub song_id: u32,
    pub artist_name: String,
    pub mapper_name: String,
    pub song_name: String,
    pub no_video: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongList {
    pub songs: Vec<Song>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Beatmapsets {
    pub beatmapset_ids: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongsWithMapper {
    pub songs: Vec<SongWithMapper>,
}