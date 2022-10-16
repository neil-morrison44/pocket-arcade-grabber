use crate::{
    files::{is_hidden, FileAndDestination},
    json_types,
};
use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

fn contains_core_and_data_json(entry: &DirEntry) -> bool {
    let is_directory = entry.file_type().is_dir();
    if !is_directory {
        return false;
    } else {
        let path = entry.clone().into_path();
        let data_file_path = path.clone().join("data.json");
        let core_file_path: PathBuf = path.clone().join("core.json");

        return data_file_path.exists() && core_file_path.exists();
    }
}

fn find_cores(starting_path: &PathBuf) -> Vec<PathBuf> {
    let mut found_core_paths: Vec<PathBuf> = Vec::new();

    let walker = WalkDir::new(starting_path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        if let Ok(found_entry) = entry {
            if contains_core_and_data_json(&found_entry) {
                println!("Found core: {:?}", found_entry.path().as_os_str());
                found_core_paths.push(found_entry.into_path());
            }
        }
    }

    return found_core_paths;
}

fn open_data_file_and_find_roms(core_path: &PathBuf) -> Result<Vec<String>, bool> {
    let file_path = Path::new(core_path).join("data.json");

    if let Ok(contents) = fs::read_to_string(&file_path) {
        if let Ok(json) = serde_json::from_str(&contents) {
            let data_data: json_types::DataFile = json;
            let rom_slots: Vec<json_types::DataSlot> = data_data
                .data
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

fn cores_looking_for_roms(cores: Vec<PathBuf>) -> Vec<PathBuf> {
    return cores
        .into_iter()
        .filter(|p| match open_data_file_and_find_roms(&p) {
            Ok(file_names) => file_names.len() > 0,
            Err(_) => false,
        })
        .collect();
}

fn find_platform_id(core_path: &PathBuf) -> Result<String, &str> {
    let core_file_path = Path::new(core_path).join("core.json");

    if let Ok(contents) = fs::read_to_string(core_file_path) {
        if let Ok(json) = serde_json::from_str(&contents) {
            let core_data: json_types::CoreFile = json;
            if let Some(first_platform_id) = core_data.core.metadata.platform_ids.first() {
                return Ok(first_platform_id.to_owned());
            }
        }
    }
    println!("Failed to read platform ids");
    return Err("Unknown error");
}

fn find_asset_folder(
    starting_path: &PathBuf,
    platform_id: &String,
    core_path: &PathBuf,
) -> PathBuf {
    let common_path = starting_path
        .clone()
        .to_owned()
        .join("Assets")
        .join(&platform_id)
        .join("common");

    if common_path.exists() {
        return common_path;
    } else {
        if let Some(parent) = common_path.parent() {
            if let Some(core_name) = core_path.file_name() {
                let core_folder = parent.join(core_name);

                if core_folder.exists() && core_folder.is_dir() {
                    return core_folder;
                }
            }
        }
    }
    println!("Unable to find a good common folder for {platform_id}");
    return common_path;
}

pub fn find_core_files(starting_path: &PathBuf) -> Vec<FileAndDestination> {
    let mut found_files: Vec<FileAndDestination> = Vec::new();
    let cores_folder = Path::new(starting_path).join("Cores");
    let core_paths = find_cores(&cores_folder);
    let rom_core_paths = cores_looking_for_roms(core_paths);

    for core_path in rom_core_paths {
        if let Ok(roms) = open_data_file_and_find_roms(&core_path) {
            if let Ok(platform_id) = find_platform_id(&core_path) {
                for rom in roms {
                    found_files.push(FileAndDestination {
                        file_name: rom,
                        destination: find_asset_folder(starting_path, &platform_id, &core_path),
                    });
                }
            }
        }
    }

    return found_files;
}
