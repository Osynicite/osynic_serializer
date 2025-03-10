pub fn check_osu_db(osu_dir:&str) -> bool {

    let path = format!("{}/osu!.db", osu_dir);
    let path = std::path::Path::new(&path);

    path.exists()
}

// 函数： check_songs_dir，检测当前地址下面是否有Songs文件夹，返回存在状态
pub fn check_songs_dir(osu_dir:&str) -> bool {

    let path = format!("{}/Songs", osu_dir);
    let path = std::path::Path::new(&path);

    path.exists()
}

// 函数：windows系统下，获取用户的用户名，然后判断osu的默认路径是否存在
pub fn check_osu_dir() -> bool {
    let username = whoami::username();
    let path = format!("C:/Users/{}/AppData/Local/osu!", username);
    let path = std::path::Path::new(&path);

    path.exists()
}

pub fn get_osu_dir() -> String {
    let username = whoami::username();
    let path = format!("C:/Users/{}/AppData/Local/osu!", username);
    path
}

pub fn check_sets_type(json: &str) -> bool {
    let json: serde_json::Value = serde_json::from_str(json).unwrap_or_default();
    if json["beatmapset_ids"].is_array() {
        true
    } else {
        false
    }
}

// 判断是不是一个SongWithMapper[]类型的json
pub fn check_songs_type(json: &str) -> bool {
    let json: serde_json::Value = serde_json::from_str(json).unwrap_or_default();
    if json.is_array() {
        let json = match json.as_array() {
            Some(array) => array,
            None => return false,
        };
        if json.len() > 0 {
            if json[0]["song_id"].is_u64() {
                true
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}
