use super::mark::add_timestamp_and_os_and_hostname_to_filename;
use crate::error::Result;

// 函数三、检测传入的路径是否存在，如果不存在新建路径
pub fn create_path_if_not_exists(path: &str) -> Result<()> {
    if !std::path::Path::new(path).exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}
// 函数十、将文件保存到指定路径，如果文件不存在则新建文件
pub fn save_file_in_if_not_exists(file_path: &str, file_name: &str, content: &str) -> Result<()> {
    let mix_path = format!("{}/{}", file_path, file_name);
    // 如果文件不存在，则新建文件
    if !std::path::Path::new(&mix_path).exists() {
        // 保存文件
        std::fs::write(&mix_path, content)?;
    }

    Ok(())
}

pub fn marked_save_to(save_path: &str, filename: &str, content: &str) -> Result<()> {
    let marked_filename = add_timestamp_and_os_and_hostname_to_filename(filename);

    create_path_if_not_exists(&save_path)?;
    save_file_in_if_not_exists(save_path, &marked_filename, content)?;
    Ok(())
}
