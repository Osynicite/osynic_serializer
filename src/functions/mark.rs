// use sys_info;

// 函数一、给文件名尾缀添加当前时间戳

pub fn add_timestamp_to_filename(filename: &str) -> String {
    let timestamp = chrono::Utc::now().format("%y%m%d%H%M%S");
    let extension = std::path::Path::new(filename)
        .extension()
        .unwrap_or_default();
    let filename_without_extension = std::path::Path::new(filename)
        .file_stem()
        .unwrap_or_default();
    format!(
        "{}_{}.{}",
        filename_without_extension.to_string_lossy(),
        timestamp,
        extension.to_string_lossy()
    )
}

// 函数二、给文件夹名尾缀添加当前时间戳
pub fn add_timestamp_to_foldername(foldername: &str) -> String {
    let timestamp = chrono::Utc::now().format("%y%m%d%H%M%S");
    format!("{}_{}", foldername, timestamp)
}

// 函数三、给文件名尾缀添加当前操作系统和设备名
pub fn add_os_and_hostname_to_filename(filename: &str) -> String {
    let os_info = if cfg!(target_os = "windows") {
        format!("{}{}", sys_info::os_type().unwrap_or_default(), sys_info::os_release().unwrap_or_default())
    } else {
        format!("{}{}",sys_info::linux_os_release().unwrap_or_default().name(),sys_info::linux_os_release().unwrap_or_default().version_id.unwrap_or_default())
    };
    let hostname = sys_info::hostname().unwrap_or_default();
    let extension = std::path::Path::new(filename)
        .extension()
        .unwrap_or_default();
    let filename_without_extension = std::path::Path::new(filename)
        .file_stem()
        .unwrap_or_default();
    format!(
        "{}_{}_{}.{}",
        filename_without_extension.to_string_lossy(),
        os_info,
        hostname,
        extension.to_string_lossy()
    )
}

// 函数四、给文件夹名尾缀添加当前操作系统和设备名
pub fn add_os_and_hostname_to_foldername(foldername: &str) -> String {
    let os_info = if cfg!(target_os = "windows") {
        format!("{}{}", sys_info::os_type().unwrap_or_default(), sys_info::os_release().unwrap_or_default())
    } else {
        format!("{}{}",sys_info::linux_os_release().unwrap_or_default().name(),sys_info::linux_os_release().unwrap_or_default().version_id.unwrap_or_default())
    };
    let hostname = sys_info::hostname().unwrap_or_default();
    format!("{}_{}_{}", foldername, os_info, hostname)
}

// 函数五、先给文件名尾缀添加当前时间戳，再给文件名尾缀添加当前操作系统和设备名
pub fn add_timestamp_and_os_and_hostname_to_filename(filename: &str) -> String {
    let timestamp = chrono::Utc::now().format("%y%m%d%H%M%S");
    let os_info = if cfg!(target_os = "windows") {
        format!("{}{}", sys_info::os_type().unwrap_or_default(), sys_info::os_release().unwrap_or_default())
    } else {
        format!("{}{}",sys_info::linux_os_release().unwrap_or_default().name(),sys_info::linux_os_release().unwrap_or_default().version_id.unwrap_or_default())
    };
    let hostname = sys_info::hostname().unwrap_or_default();
    let extension = std::path::Path::new(filename)
        .extension()
        .unwrap_or_default();
    let filename_without_extension = std::path::Path::new(filename)
        .file_stem()
        .unwrap_or_default();
    format!(
        "{}_{}_{}_{}.{}",
        filename_without_extension.to_string_lossy(),
        timestamp,
        os_info,
        hostname,
        extension.to_string_lossy()
    )
}

// 函数六、先给文件夹名尾缀添加当前时间戳，再给文件夹名尾缀添加当前操作系统和设备名
pub fn add_timestamp_and_os_and_hostname_to_foldername(foldername: &str) -> String {
    let timestamp = chrono::Utc::now().format("%y%m%d%H%M%S");
    let os_info = if cfg!(target_os = "windows") {
        format!("{}{}", sys_info::os_type().unwrap_or_default(), sys_info::os_release().unwrap_or_default())
    } else {
        format!("{}{}",sys_info::linux_os_release().unwrap_or_default().name(),sys_info::linux_os_release().unwrap_or_default().version_id.unwrap_or_default())
    };
    let hostname = sys_info::hostname().unwrap_or_default();
    format!("{}_{}_{}_{}", foldername, timestamp, os_info, hostname)
}

// test module

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_timestamp_to_filename() {
        let filename = "songs.json";
        let new_filename = add_timestamp_to_filename(filename);
        println!("{}", new_filename);
        assert!(new_filename.starts_with("songs_"));
        assert!(new_filename.ends_with(".json"));
    }

    #[test]
    fn test_add_timestamp_to_foldername() {
        let foldername = "fetch";
        let new_foldername = add_timestamp_to_foldername(foldername);
        println!("{}", new_foldername);
        assert!(new_foldername.starts_with("fetch_"));
    }

    #[test]
    fn test_add_os_and_hostname_to_filename() {
        let filename = "songs.json";
        let new_filename = add_os_and_hostname_to_filename(filename);
        println!("{}", new_filename);
        assert!(new_filename.starts_with("songs_"));
        assert!(new_filename.ends_with(".json"));
    }
    // ---- core::algorithms::mark::tests::test_add_os_and_hostname_to_filename stdout ----
    // songs_Windows6.2.9200_DESKTOP-KUEMADK.json
    // songs_Ubuntu22.04_DESKTOP-KUEMADK.json
    // songs_NixOS24.05_nixos.json
    // successes:
    // core::algorithms::mark::tests::test_add_os_and_hostname_to_filename
    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 42 filtered out; finished in 0.18s

    #[test]
    fn test_add_os_and_hostname_to_foldername() {
        let foldername = "fetch";
        let new_foldername = add_os_and_hostname_to_foldername(foldername);
        println!("{}", new_foldername);
        assert!(new_foldername.starts_with("fetch_"));
    }

    #[test]
    fn test_add_timestamp_and_os_and_hostname_to_filename() {
        let filename = "songs_m.json";
        let new_filename = add_timestamp_and_os_and_hostname_to_filename(filename);
        println!("{}", new_filename);
        assert!(new_filename.starts_with("songs_m_"));
        assert!(new_filename.ends_with(".json"));
    }
    // ---- core::algorithms::mark::tests::test_add_timestamp_and_os_and_hostname_to_filename stdout ----
    // songs_m_241216031056_Windows6.2.9200_DESKTOP-KUEMADK.json
    // songs_m_241216031224_Ubuntu22.04_DESKTOP-KUEMADK.json
    // songs_m_241216031523_NixOS24.05_nixos.json
    // successes:
    // core::algorithms::mark::tests::test_add_timestamp_and_os_and_hostname_to_filename
    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 44 filtered out; finished in 0.05s
    #[test]
    fn test_add_timestamp_and_os_and_hostname_to_foldername() {
        let foldername = "fetch";
        let new_foldername = add_timestamp_and_os_and_hostname_to_foldername(foldername);
        println!("{}", new_foldername);
        assert!(new_foldername.starts_with("fetch_"));
    }
}
