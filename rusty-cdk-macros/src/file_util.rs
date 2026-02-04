use dirs::home_dir;
use serde::Serialize;
use std::fs::{read_to_string, write};
use std::path::{Path, PathBuf, absolute};

pub(crate) fn read_info(file_name: &str) -> Option<String> {
    get_file_path(file_name).and_then(|p| read_to_string(p).ok())
}

pub(crate) fn write_info<T: Serialize>(file_name: &str, info: T) {
    if let Some(path) = get_file_path(file_name) {
        let info_as_string = serde_json::to_string(&info).expect("to be able to serialize bucket info");
        let _result = write(&path, info_as_string);
    }
}

pub(crate) fn get_file_path(file_name: &str) -> Option<PathBuf> {
    home_dir().map(|home_dir| home_dir.join(file_name))
}

pub(crate) fn get_absolute_file_path(value: &str) -> Result<String, String> {
    let path = Path::new(&value);

    if !path.exists() {
        return Err(format!("did not find file `{value}`"));
    }

    let value = if path.is_relative() {
        match absolute(path) {
            Ok(absolute_path) => absolute_path.to_str().expect("path to be valid unicode").to_string(),
            Err(e) => {
                return Err(format!("failed to convert path `{value}` to absolute path: {e}"));
            }
        }
    } else {
        path.to_str().expect("path to be valid unicode").to_string()
    };

    Ok(value)
}
