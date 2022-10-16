use crate::{
    files::{is_hidden, FileAndDestination},
    json_types,
};
use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

fn open_asset_file_and_find_roms(asset_path: &PathBuf) -> Result<Vec<String>, bool> {
    let file_path = Path::new(asset_path);

    if let Ok(contents) = fs::read_to_string(&file_path) {
        if let Ok(json) = serde_json::from_str(&contents) {
            let data_data: json_types::AssetFile = json;
            let rom_slots: Vec<json_types::DataSlot> = data_data
                .instance
                .data_slots
                .into_iter()
                .filter(|d| {
                    d.filename
                        .as_ref()
                        .unwrap_or(&String::new())
                        .ends_with(".rom")
                })
                .collect();

            let file_names: Vec<String> = rom_slots
                .into_iter()
                .map(|d| d.filename)
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect();
            return Ok(file_names);
        }
    }
    return Err(false);
}

fn assets_looking_for_roms(cores: Vec<PathBuf>) -> Vec<PathBuf> {
    return cores
        .into_iter()
        .filter(|p| match open_asset_file_and_find_roms(&p) {
            Ok(file_names) => file_names.len() > 0,
            Err(_) => false,
        })
        .collect();
}

fn find_json_files(starting_path: &PathBuf) -> Vec<PathBuf> {
    let mut found_json_paths: Vec<PathBuf> = Vec::new();

    let walker = WalkDir::new(starting_path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        if let Ok(found_entry) = entry {
            if found_entry.file_type().is_file() {
                match found_entry.path().extension().and_then(OsStr::to_str) {
                    Some("json") => {
                        println!("Found json file: {:?}", found_entry.path().as_os_str());
                        found_json_paths.push(found_entry.into_path());
                    }
                    Some(&_) => (),
                    None => (),
                }
            }
        }
    }

    return found_json_paths;
}

fn find_common_path(json_path: &PathBuf) -> PathBuf {
    // This isn't great, but the common folder is probably nearby
    let mut current_path: PathBuf = json_path.clone();

    while !current_path.join("common").exists() {
        if let Some(parent_path) = current_path.parent().and_then(|p| Some(p.to_path_buf())) {
            current_path = parent_path
        } else {
            return current_path.join("common");
        }
    }

    return current_path.join("common");
}

pub fn find_asset_files(starting_path: &PathBuf) -> Vec<FileAndDestination> {
    let mut found_files: Vec<FileAndDestination> = Vec::new();
    let assets_folder = Path::new(starting_path).join("Assets");
    let asset_file_paths = find_json_files(&assets_folder);
    let rom_core_paths = assets_looking_for_roms(asset_file_paths);

    for asset_path in rom_core_paths {
        if let Ok(roms) = open_asset_file_and_find_roms(&asset_path) {
            for rom in roms {
                found_files.push(FileAndDestination {
                    file_name: rom,
                    destination: find_common_path(&asset_path),
                });
            }
        }
    }

    return found_files;
}
