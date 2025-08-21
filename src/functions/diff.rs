use crate::types::{Song, SongWithMapper};

// 函数一 比较新旧两个的Vec<Song>的差异，输出新的Vec<Song>里面多出来的部分，按照song_id来比较，尽可能减少内存占用

pub fn diff_new_songs_by_song_id(songs_old: &[Song], songs_new: &[Song]) -> Vec<Song> {
    let song_id_list_old: Vec<u32> = songs_old.iter().map(|song| song.song_id).collect();
    let mut diff_songs: Vec<Song> = Vec::new();
    for song in songs_new {
        if !song_id_list_old.contains(&song.song_id) {
            diff_songs.push(song.clone());
        }
    }
    diff_songs
}

pub fn diff_new_songs_by_song_id_with_mapper(
    songs_old: &[SongWithMapper],
    songs_new: &[SongWithMapper],
) -> Vec<SongWithMapper> {
    let song_id_list_old: Vec<u32> = songs_old.iter().map(|song| song.song_id).collect();
    let mut diff_songs: Vec<SongWithMapper> = Vec::new();
    for song in songs_new {
        if !song_id_list_old.contains(&song.song_id) {
            diff_songs.push(song.clone());
        }
    }
    diff_songs
}

// 函数三 比较新旧两个的Vec<u32>的差异，输出新的Vec<u32>里面多出来的部分，尽可能减少内存占用
pub fn diff_new_sets_by_sets(sets_old: &[u32], sets_new: &[u32]) -> Vec<u32> {
    let mut diff_sets: Vec<u32> = Vec::new();
    for sets_item in sets_new {
        if !sets_old.contains(sets_item) {
            diff_sets.push(*sets_item);
        }
    }
    diff_sets
}

// test module

#[cfg(test)]

mod tests {
    use super::*;
    use crate::types::Song;

    #[test]
    fn test_diff_songs_by_song_id() {
        let song1 = Song {
            song_id: 1,
            artist_name: "artist1".to_string(),
            song_name: "song1".to_string(),
            no_video: false,
        };
        let song2 = Song {
            song_id: 2,
            artist_name: "artist2".to_string(),
            song_name: "song2".to_string(),
            no_video: false,
        };
        let song3 = Song {
            song_id: 3,
            artist_name: "artist3".to_string(),
            song_name: "song3".to_string(),
            no_video: false,
        };
        let song4 = Song {
            song_id: 4,
            artist_name: "artist4".to_string(),
            song_name: "song4".to_string(),
            no_video: false,
        };
        let song5 = Song {
            song_id: 5,
            artist_name: "artist5".to_string(),
            song_name: "song5".to_string(),
            no_video: false,
        };
        let songs_old = vec![song1.clone(), song3.clone(), song5];
        let songs_new = vec![song1, song2, song3, song4];
        let diff_songs = diff_new_songs_by_song_id(&songs_old, &songs_new);
        println!("{:?}", diff_songs);
        assert_eq!(diff_songs.len(), 2);
        assert_eq!(diff_songs[0].song_id, 2);
        assert_eq!(diff_songs[1].song_id, 4);
    }
    // ---- core::algorithms::diff::tests::test_diff_songs_by_song_id stdout ----
    // [Song { song_id: 2, artist_name: "artist2", song_name: "song2", no_video: false }, Song { song_id: 4, artist_name: "artist4", song_name: "song4", no_video: false }]
    // successes:
    // core::algorithms::diff::tests::test_diff_songs_by_song_id
    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 16 filtered out; finished in 0.00s
}
