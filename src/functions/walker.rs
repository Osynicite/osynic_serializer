use crate::error::Result;
use std::path::{Path, PathBuf};

// 函数一：进入文件夹，返回文件夹路径，如果不存在则新建后返回
pub fn enter_folder(folder_path: &str) -> Result<PathBuf> {
    let folder_path = Path::new(folder_path);
    if folder_path.exists() {
        Ok(folder_path.to_path_buf())
    } else {
        std::fs::create_dir_all(folder_path)?;
        Ok(folder_path.to_path_buf())
    }
}

// 函数二：进入文件夹，返回文件夹路径，如果不存在则报错
pub fn enter_folder_if_exists(folder_path: &str) -> Result<PathBuf> {
    let folder_path = Path::new(folder_path);
    if folder_path.exists() {
        Ok(folder_path.to_path_buf())
    } else {
        Err("Folder does not exist".into())
    }
}

// 函数三：进入传入文件夹的某个子文件夹，返回子文件夹路径，如果不存在则新建后返回
pub fn enter_sub_folder(folder_path: &str, sub_folder_name: &str) -> Result<PathBuf> {
    let folder_path = Path::new(folder_path);
    let sub_folder_path = folder_path.join(sub_folder_name);
    if sub_folder_path.exists() {
        Ok(sub_folder_path)
    } else {
        std::fs::create_dir_all(&sub_folder_path)?;
        Ok(sub_folder_path)
    }
}

// 函数四：进入传入文件夹的某个子文件夹，返回子文件夹路径，如果不存在则报错
pub fn enter_sub_folder_if_exists(folder_path: &str, sub_folder_name: &str) -> Result<PathBuf> {
    let folder_path = Path::new(folder_path);
    let sub_folder_path = folder_path.join(sub_folder_name);
    if sub_folder_path.exists() {
        Ok(sub_folder_path)
    } else {
        Err("Folder does not exist".into())
    }
}

// 函数五、遍历文件夹下的所有文件夹，返回entry
pub fn walk_folder(folder_path: &str) -> Result<std::fs::ReadDir> {
    let folder_path = Path::new(folder_path);
    let read_dir = std::fs::read_dir(folder_path)?;
    Ok(read_dir)
}

// 函数六、遍历文件夹下的所有文件夹，返回entry的路径
pub fn walk_folder_path(folder_path: &str) -> Result<Vec<PathBuf>> {
    let mut folder_paths = Vec::new();
    for entry in walk_folder(folder_path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            folder_paths.push(entry_path);
        }
    }
    Ok(folder_paths)
}

// 函数七、遍历文件夹下的所有文件夹，返回所有子文件夹的名称、
pub fn walk_folder_name(folder_path: &str) -> Result<Vec<String>> {
    let mut folder_names = Vec::new();
    for entry in walk_folder(folder_path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            let folder_name = entry.file_name().into_string().unwrap();
            folder_names.push(folder_name);
        }
    }
    Ok(folder_names)
}

//函数八、遍历文件夹下的所有文件，返回entry的路径
pub fn walk_file_path(folder_path: &str) -> Result<Vec<PathBuf>> {
    let mut file_paths = Vec::new();
    for entry in walk_folder(folder_path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_file() {
            file_paths.push(entry_path);
        }
    }
    Ok(file_paths)
}

//函数九、遍历文件夹下的所有文件，返回所有文件的名称
pub fn walk_file_name(folder_path: &str) -> Result<Vec<String>> {
    let mut file_names = Vec::new();
    for entry in walk_folder(folder_path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_file() {
            let file_name = entry.file_name().into_string().unwrap();
            file_names.push(file_name);
        }
    }
    Ok(file_names)
}

//函数十、遍历文件夹下的所有文件，返回指定后缀的文件的名称
pub fn walk_file_name_with_extension(folder_path: &str, extension: &str) -> Result<Vec<String>> {
    let mut file_names = Vec::new();
    for entry in walk_folder(folder_path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_file() {
            let file_name = entry.file_name().into_string().unwrap();
            if file_name.ends_with(extension) {
                file_names.push(file_name);
            }
        }
    }
    Ok(file_names)
}

//函数十一、遍历文件夹下的所有文件，返回指定后缀的第一个文件的名称
pub fn walk_file_name_with_extension_first(folder_path: &str, extension: &str) -> Result<String> {
    for entry in walk_folder(folder_path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_file() {
            let file_name = entry.file_name().into_string().unwrap();
            if file_name.ends_with(extension) {
                return Ok(file_name);
            }
        }
    }
    Err("No file with extension found".into())
}
