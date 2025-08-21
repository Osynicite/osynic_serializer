use crate::types::{Song, SongWithMapper};
use osynic_osudb::entity::osu::beatmap::Beatmap;

// 函数一：将beatmap类型里面的四个字段提取出来填充到song类型里面
// 四个字段的一一对应是这样的：
// beatmapset_id -> song_id
// artist_unicode -> artist_name
// title_unicode -> song_name
// creator -> mapper_name
// disable_video -> no_video

pub fn parse_beatmap_to_song(beatmap: &Beatmap) -> Song {
    Song {
        song_id: beatmap.beatmapset_id as u32,
        artist_name: beatmap.artist_unicode.clone().unwrap_or("".to_string()),
        song_name: beatmap.title_unicode.clone().unwrap_or("".to_string()),
        no_video: beatmap.disable_video,
    }
}

pub fn parse_beatmap_to_song_with_mapper(beatmap: &Beatmap) -> SongWithMapper {
    SongWithMapper {
        song_id: beatmap.beatmapset_id as u32,
        artist_name: beatmap.artist_unicode.clone().unwrap_or("".to_string()),
        song_name: beatmap.title_unicode.clone().unwrap_or("".to_string()),
        no_video: beatmap.disable_video,
        mapper_name: beatmap.creator.clone().unwrap_or("".to_string()),
    }
}

// 函数二、Vec<Beatmap> -> Vec<Song>

pub fn parse_beatmap_list_to_song_list(beatmaps: &[Beatmap]) -> Vec<Song> {
    beatmaps
        .iter()
        .map(parse_beatmap_to_song)
        .collect()
}

// 函数三、返回Song的song_id
pub fn parse_song_id(song: &Song) -> u32 {
    song.song_id
}

// 函数四、传入Vec<Song>，返回song_id的Vec<u32>
pub fn parse_song_id_list(songs: &[Song]) -> Vec<u32> {
    songs.iter().map(parse_song_id).collect()
}

// 函数五，输入Vec<SongWithMapper>，返回song_id的Vec<u32>
pub fn parse_song_id_list_with_mapper(songs: &[SongWithMapper]) -> Vec<u32> {
    songs.iter().map(|song| song.song_id).collect()
}

// test module

#[cfg(test)]

mod tests {
    use super::*;
    use osynic_osudb::entity::osu::beatmap::Beatmap;

    #[test]
    fn test_parse_beatmap_to_song() {
        let beatmap = Beatmap {
            beatmapset_id: 1,
            artist_unicode: Some("artist".to_string()),
            title_unicode: Some("title".to_string()),
            disable_video: false,
            ..Default::default()
        };
        let song = parse_beatmap_to_song(&beatmap);
        assert_eq!(song.song_id, 1);
        assert_eq!(song.artist_name, "artist");
        assert_eq!(song.song_name, "title");
        assert_eq!(song.no_video, false);
    }

    #[test]
    fn test_parse_beatmap_list_to_song_list() {
        let beatmaps = vec![
            Beatmap {
                beatmapset_id: 1,
                artist_unicode: Some("artist1".to_string()),
                title_unicode: Some("title1".to_string()),
                disable_video: false,
                ..Default::default()
            },
            Beatmap {
                beatmapset_id: 2,
                artist_unicode: Some("artist2".to_string()),
                title_unicode: Some("title2".to_string()),
                disable_video: true,
                ..Default::default()
            },
        ];
        let songs = parse_beatmap_list_to_song_list(&beatmaps);
        assert_eq!(songs.len(), 2);
        assert_eq!(songs[0].song_id, 1);
        assert_eq!(songs[0].artist_name, "artist1");
        assert_eq!(songs[0].song_name, "title1");
        assert_eq!(songs[0].no_video, false);
        assert_eq!(songs[1].song_id, 2);
        assert_eq!(songs[1].artist_name, "artist2");
        assert_eq!(songs[1].song_name, "title2");
        assert_eq!(songs[1].no_video, true);
    }

    #[test]
    fn test_parse_song_id() {
        let song = Song {
            song_id: 1,
            artist_name: "artist".to_string(),
            song_name: "title".to_string(),
            no_video: false,
        };
        assert_eq!(parse_song_id(&song), 1);
    }

    #[test]
    fn test_parse_song_id_list() {
        let songs = vec![
            Song {
                song_id: 1,
                artist_name: "artist1".to_string(),
                song_name: "title1".to_string(),
                no_video: false,
            },
            Song {
                song_id: 1001653,
                artist_name: "MIMI feat Hatsune Miku".to_string(),
                song_name: "Mizuoto to Curtain".to_string(),
                no_video: false,
            },
        ];
        let song_ids = parse_song_id_list(&songs);
        println!("{:?}", song_ids);
        assert_eq!(song_ids.len(), 2);
        assert_eq!(song_ids[0], 1);
        assert_eq!(song_ids[1], 1001653);
    }
    // ---- core::algorithms::parse::tests::test_parse_song_id_list stdout ----
    // [1, 1001653]
    // successes:
    // core::algorithms::parse::tests::test_parse_song_id_list
    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s
}
