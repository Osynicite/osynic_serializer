use crate::error::Result;
use crate::functions::check::{check_osu_db, check_songs_dir};
use crate::functions::diff::{diff_new_sets_by_sets, diff_new_songs_by_song_id_with_mapper};
use crate::functions::serialize::{
    open_osu_db, serialize_osu_db_with_mapper, serialize_song_folder_with_mapper,
};
use crate::types::{Beatmapsets, SongsWithMapper};

pub fn serialize_by_folder(osu_dir: &str) -> Result<SongsWithMapper> {
    if check_songs_dir(osu_dir) {
        let song_dir = format!("{}\\Songs", osu_dir);
        let songs = serialize_song_folder_with_mapper(&song_dir)?;
        // let json = serde_json::to_string_pretty(&songs)?;
        // marked_save_to(save_path, "songs_m.json", &json)?;
        Ok(SongsWithMapper { songs })
    } else {
        eprintln!("No songs found in the directory: {}", osu_dir);
        Err("No songs found".into())
    }
}

pub fn serialize_by_osu_db(osu_dir: &str) -> Result<SongsWithMapper> {
    if check_osu_db(osu_dir) {
        let osudb_dir = format!("{}\\osu!.db", osu_dir);
        let mut osu_db = open_osu_db(&osudb_dir)?;
        let songs = serialize_osu_db_with_mapper(&mut osu_db)?;
        // let json = serde_json::to_string_pretty(&songs)?;
        // marked_save_to(save_path, "songs_dm.json", &json)?;
        Ok(SongsWithMapper { songs })
    } else {
        eprintln!("No osu!db found in the directory : {}", osu_dir);
        Err("No osu!db found".into())
    }
}

pub fn diff_sets(sets_old: &Beatmapsets, sets_new: &Beatmapsets) -> Beatmapsets {
    let diff_beatmapset_ids =
        diff_new_sets_by_sets(&sets_old.beatmapset_ids, &sets_new.beatmapset_ids);
    Beatmapsets {
        beatmapset_ids: diff_beatmapset_ids,
    }
}

pub fn diff_songs(songs_old: &SongsWithMapper, songs_new: &SongsWithMapper) -> SongsWithMapper {
    let diff_songs = diff_new_songs_by_song_id_with_mapper(&songs_old.songs, &songs_new.songs);
    SongsWithMapper { songs: diff_songs }
}
