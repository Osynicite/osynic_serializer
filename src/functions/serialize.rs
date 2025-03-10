use osynic_osudb::entity::osu::osudb::OsuDB;
use super::parse::{parse_beatmap_to_song,parse_beatmap_to_song_with_mapper};
// use super::mark::add_timestamp_and_os_and_hostname_to_filename;
use super::walker::{walk_file_name_with_extension_first, walk_folder_name};
use super::storage::marked_save_to;
use crate::types::{Song,SongWithMapper};
use crate::error::Result;

pub fn serialize_song_folder_raw(folder_name: &str) -> Result<(u32, String, String, bool)> {
    let parts: Vec<&str> = folder_name.splitn(2, ' ').collect();
    let song_id_str = parts[0];

    if !song_id_str.chars().all(|c| c.is_digit(10)) {
        return Err("Invalid song_id".into());
    }

    if parts.len() < 2 {
        return Err("Invalid folder name format".into());
    }

    let song_id: u32 = song_id_str.parse()?;

    let rest = parts[1];
    let parts: Vec<&str> = rest.rsplitn(2, " - ").collect();

    if parts.len() < 2 {
        return Ok((song_id, "manbo".to_string(), "manbo".to_string(), false));
    }

    let artist_name = parts[1].to_string();

    let song_parts: Vec<&str> = parts[0].rsplitn(2, " [no video]").collect();
    let song_name = song_parts[song_parts.len() - 1].to_string();
    let no_video = song_parts.len() == 2;

    Ok((song_id, artist_name, song_name, no_video))
}


pub fn serialize_song_folder(folder_name: &str) -> Result<Song> {
    let (song_id, artist_name, song_name, no_video) = serialize_song_folder_raw(folder_name)?;
    Ok(Song{song_id, artist_name, song_name, no_video})
}

pub fn serialize_mapper_raw(folder_name: &str) -> Result<String> {
    let parts: Vec<&str> = folder_name.rsplitn(2, '(').collect();
    let rest = parts[0];
    let rparts: Vec<&str> = rest.split( ')').collect();
    let mapper_name = rparts[0].to_string();
    Ok(mapper_name)
}

    
pub fn serialize_osu_db(osu_db: &mut OsuDB) -> Result<Vec<Song>> {
    let mut songs = Vec::new();

    let mut beatmapset_id_set = std::collections::HashSet::new();
    for beatmap in osu_db.beatmaps.iter_mut() {
        if beatmapset_id_set.contains(&beatmap.beatmapset_id) {
            continue;
        }
        beatmapset_id_set.insert(beatmap.beatmapset_id);
        let song = parse_beatmap_to_song(beatmap);
        songs.push(song);
    }

    Ok(songs)
}

pub fn serialize_osu_db_with_mapper(osu_db: &mut OsuDB) -> Result<Vec<SongWithMapper>> {
    let mut songs = Vec::new();

    let mut beatmapset_id_set = std::collections::HashSet::new();
    for beatmap in osu_db.beatmaps.iter_mut() {
        if beatmapset_id_set.contains(&beatmap.beatmapset_id) {
            continue;
        }
        beatmapset_id_set.insert(beatmap.beatmapset_id);
        let song = parse_beatmap_to_song_with_mapper(beatmap);
        songs.push(song);
    }

    Ok(songs)
}

pub fn serialize_song_folder_with_mapper(songs_dir: &str) -> Result<Vec<SongWithMapper>> {
    let mut songs = Vec::new();

    for folder_name in walk_folder_name(songs_dir)? {
        if let Ok((song_id, artist_name, song_name, no_video)) = serialize_song_folder_raw(&folder_name) {
            let sub_name = format!("{}/{}", songs_dir, folder_name);
            let inner_name = walk_file_name_with_extension_first(&sub_name, ".osu")?; 
            let mapper_name = serialize_mapper_raw(&inner_name)?;
            songs.push(SongWithMapper {
                song_id,
                artist_name,
                song_name,
                no_video,
                mapper_name,
            });
        }
    }

    Ok(songs)
}

// 函数一、 从文件夹中读取地图,不包含mapper_name，并保存到路径是当前文件夹下的/songs文件夹下，命名为songs_{}.json，其中{}为当前时间戳
pub fn serialize_by_folder_name(songs_dir: &str) -> Result<()> {
    let mut songs = Vec::new();

    for folder_name in walk_folder_name(songs_dir)? {
        if let Ok((song_id, artist_name, song_name, no_video)) = serialize_song_folder_raw(&folder_name) {
            songs.push(Song {
                song_id,
                artist_name,
                song_name,
                no_video,
            });
        }
    }

    let json = serde_json::to_string_pretty(&songs)?;
    // let file_name = add_timestamp_and_os_and_hostname_to_filename("songs.json");
    // let path = "songs";
    // create_path_if_not_exists(&path)?;
    // save_file_in_if_not_exists(path, &file_name, &json)?;
    marked_save_to("songs", "songs.json", &json)?;

    Ok(())
}

// 函数二、 从文件夹中读取地图,包含mapper_name，并保存到当前文件夹下的/songs文件夹下，命名为songs_m_{}.json，其中{}为当前时间戳
// 其中，mapper_name是从子文件夹中读取的一个.osu文件的文件名经过serialize_mapper_raw函数处理后的结果,注意！很多个.osu文件只取第一个
pub fn serialize_by_folder_name_with_mapper(songs_dir: &str,save_path:&str) -> Result<()> {
    let mut songs = Vec::new();

    for folder_name in walk_folder_name(songs_dir)? {
        if let Ok((song_id, artist_name, song_name, no_video)) = serialize_song_folder_raw(&folder_name) {
            let sub_name = format!("{}/{}", songs_dir, folder_name);
            let inner_name = walk_file_name_with_extension_first(&sub_name, ".osu")?; 
            let mapper_name = serialize_mapper_raw(&inner_name)?;
            songs.push(SongWithMapper {
                song_id,
                artist_name,
                song_name,
                no_video,
                mapper_name,
            });
        }
    }

    let json = serde_json::to_string_pretty(&songs)?;
    // let file_name = add_timestamp_and_os_and_hostname_to_filename("songs_m.json");
    // create_path_if_not_exists(&save_path)?;
    // save_file_in_if_not_exists(save_path, &file_name, &json)?;
    marked_save_to(save_path, "songs_m.json", &json)?;
    Ok(())
}

// 函数三、从OsuDB中读取地图，并保存到当前文件夹下的/songs文件夹下，命名为songs_d_{}.json，其中{}为当前时间戳
pub fn serialize_by_osu_db(osu_db: &mut OsuDB) -> Result<()> {
    let songs = serialize_osu_db(osu_db)?;
    let json = serde_json::to_string_pretty(&songs)?;
    // let file_name = add_timestamp_and_os_and_hostname_to_filename("songs_d.json");
    // let path = "songs";
    // create_path_if_not_exists(&path)?;
    // save_file_in_if_not_exists(path, &file_name, &json)?;
    marked_save_to("songs", "songs_d.json", &json)?;
    Ok(())
}

// 函数四、从OsuDB中读取地图,包含mapper_name，并保存到当前文件夹下的/songs文件夹下，命名为songs_dm_{}.json，其中{}为当前时间戳
pub fn serialize_by_osu_db_with_mapper(osu_db: &mut OsuDB,save_path:&str) -> Result<()> {
    let songs = serialize_osu_db_with_mapper(osu_db)?;
    let json = serde_json::to_string_pretty(&songs)?;
    // let file_name = add_timestamp_and_os_and_hostname_to_filename("songs_dm.json");
    // create_path_if_not_exists(&save_path)?;
    // save_file_in_if_not_exists(save_path, &file_name, &json)?;
    marked_save_to(save_path, "songs_dm.json", &json)?;
    Ok(())
}

pub fn open_osu_db(osu_db: &str) -> Result<OsuDB> {
    Ok(OsuDB::from_file(osu_db)?)
}


// 函数五
// pub fn serialize_by_folder_name_with_mapper(songs_dir: &str) -> Result<Vec<SongWithMapper>> {
//     let mut songs = Vec::new();

//     for folder_name in walk_folder_name(songs_dir)? {
//         if let Ok((song_id, artist_name, song_name, no_video)) = serialize_song_folder_raw(&folder_name) {
//             let sub_name = format!("{}/{}", songs_dir, folder_name);
//             let inner_name = walk_file_name_with_extension_first(&sub_name, ".osu")?; 
//             let mapper_name = serialize_mapper_raw(&inner_name)?;
//             songs.push(SongWithMapper {
//                 song_id,
//                 artist_name,
//                 song_name,
//                 no_video,
//                 mapper_name,
//             });
//         }
//     }

//     Ok(songs)
//     // let diff_songs = diff_new_songs_by_song_id_with_mapper(&songs, &songs);

//     // let json = serde_json::to_string_pretty(&songs)?;
//     // marked_save_to(save_path, "diff_songs_m.json", &json)?;
//     // let file_name = add_timestamp_and_os_and_hostname_to_filename("diff_songs_m.json");
//     // create_path_if_not_exists(&save_path)?;
//     // save_file_in_if_not_exists(save_path, &file_name, &json)?;
//     // Ok(())
//     // Ok(diff_songs)
// }



// test module

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_serialize_song_folder_raw() {
        let folder_name = "1985060 hitorie - Nichijou to Chikyuu no Gakubuchi (wowaka x Hatsune Miku Edit)";
        let result = serialize_song_folder_raw(folder_name).unwrap();
        println!("{:?}", result);
    }
    // ---- core::algorithms::serialize::tests::test_serialize_song_folder_raw stdout ----
    // (1985060, "hitorie", "Nichijou to Chikyuu no Gakubuchi (wowaka x Hatsune Miku Edit)", false)
    // successes:
    // core::algorithms::serialize::tests::test_serialize_song_folder_raw
    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 47 filtered out; finished in 0.00s

    #[test]
    fn test_serialize_song_folder() {
        let folder_name = "1985060 hitorie - Nichijou to Chikyuu no Gakubuchi (wowaka x Hatsune Miku Edit)";
        let result = serialize_song_folder(folder_name).unwrap();
        println!("{:?}", result);
    }
    // ---- core::algorithms::serialize::tests::test_serialize_song_folder stdout ----
    // Song { song_id: 1985060, artist_name: "hitorie", song_name: "Nichijou to Chikyuu no Gakubuchi (wowaka x Hatsune Miku Edit)", no_video: false }
    // successes:
    // core::algorithms::serialize::tests::test_serialize_song_folder
    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

    #[test]
    fn test_serialize_mapper_raw() {
        let folder_name = "hitorie - Nichijou to Chikyuu no Gakubuchi (wowaka x Hatsune Miku Edit) (flake) [take care.].osu";
        let result = serialize_mapper_raw(folder_name).unwrap();
        println!("{:?}", result);
    }
    // ---- core::algorithms::serialize::tests::test_serialize_mapper_raw stdout ----
    // "flake"
    // successes:
    // core::algorithms::serialize::tests::test_serialize_mapper_raw
    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 21 filtered out; finished in 0.00s


    #[test]
    fn test_serialize_osu_db() {
        let mut osu_db = OsuDB::from_file("osu!.db").unwrap();
        let songs = serialize_osu_db(&mut osu_db).unwrap();
        println!("{:?}", songs);
    }

    #[test]
    fn test_serialize_osu_db_with_mapper() {
        let mut osu_db = OsuDB::from_file("osu!.db").unwrap();
        let songs = serialize_osu_db_with_mapper(&mut osu_db).unwrap();
        println!("{:?}", songs);
    }
}