use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct FolderList {
    pub name: Vec<String>,
}


#[derive(Deserialize)]
pub struct Folder {
    pub path: String,
}
