use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Clone)]
pub struct FileReadError;

pub struct FileUri {
    pub uri: String
}

impl From<String> for FileUri {
    fn from(value: String) -> Self {
        Self {
            uri: String::from("file://") + &value
        }
    }
}

impl TryFrom<PathBuf> for FileUri {
    type Error = FileReadError;

    fn try_from(value: PathBuf) -> Result<Self, FileReadError> {
        match value.to_str() {
            Some(path) => { 
                Ok(Self {uri: String::from("file://") + &path})
            },
            None => Err(FileReadError)
        }
    }
}

pub struct AssetManager {
    pub tiles: HashMap<String, FileUri>
}

impl AssetManager {
    // TODO: Load tiles only for now
    pub fn load_assets(dir: &Path) -> Self {
        let tiles = fs::read_dir(dir.join("tiles"))
            .unwrap_or_else(|_| panic!("Cannot open {}", dir.display()))
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                let path = e.path();
                if path.extension().map_or(false, |e| e == "png") {
                    let name = path.file_stem()?.to_str()?.to_string();
                    let path_display = path.display().to_string();
                    Some((
                        name, 
                        FileUri::try_from(path)
                            .expect(&format!("Could not read asset from {}", path_display))
                    ))
                } else {
                    None
                }
            })
            .collect();

        Self { tiles }
    }
}