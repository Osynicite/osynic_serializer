pub fn check_osu_db(osu_dir: &str) -> bool {
    let path = format!("{osu_dir}/osu!.db");
    let path = std::path::Path::new(&path);

    path.exists()
}

// 函数： check_songs_dir，检测当前地址下面是否有Songs文件夹，返回存在状态
pub fn check_songs_dir(osu_dir: &str) -> bool {
    let path = format!("{osu_dir}/Songs");
    let path = std::path::Path::new(&path);

    path.exists()
}

// 函数：windows系统下，获取用户的用户名，然后判断osu的默认路径是否存在
pub fn check_osu_dir() -> bool {
    let username = whoami::username();
    let path = format!("C:/Users/{username}/AppData/Local/osu!");
    let path = std::path::Path::new(&path);

    path.exists()
}

pub fn get_osu_dir() -> String {
    let username = whoami::username();
    let path = format!("C:/Users/{username}/AppData/Local/osu!");
    path
}

pub fn check_sets_type(json: &str) -> bool {
    let json: serde_json::Value = serde_json::from_str(json).unwrap_or_default();
    json["beatmapset_ids"].is_array()
}

// 判断是不是一个SongWithMapper[]类型的json
pub fn check_songs_type(json: &str) -> bool {
    let json: serde_json::Value = serde_json::from_str(json).unwrap_or_default();
    if json.is_array() {
        let json = match json.as_array() {
            Some(array) => array,
            None => return false,
        };
        if !json.is_empty() {
            json[0]["song_id"].is_u64()
        } else {
            false
        }
    } else {
        false
    }
}
