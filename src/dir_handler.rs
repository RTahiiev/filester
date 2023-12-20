use std::path::Path;
use std::fs::create_dir_all;

use log::debug;

// TODO: move to env
pub const BASE_DIR: &str = "/home/rt/my_projects/filester_storage/root/";

pub fn path_exist(path: &str) -> bool {
    Path::new(path).exists()
}


pub fn create_folder(path: &str) {
    debug!("create_folder: {}", path);
    let _ = create_dir_all(path)
        .map_err(|err| {debug!("{:?}", err)});
}


pub fn read_folder(path: &str) -> Result<Vec<String>, std::io::Error> {
    debug!("read_folder: {}", path);
    let mut result = Vec::new();
    let path = Path::new(path);

    let entries = match path.read_dir() {
        Ok(entries) => entries,
        Err(err) => return Err(err),
    };

    for entry in entries {
        match entry {
            Ok(dir) => {
                if let Some(path_str) = dir.file_name().to_str() {
                    result.push(path_str.to_owned());
                }
            },
            Err(err) => debug!("{:?}", err)
        }
    }

    Ok(result)
}