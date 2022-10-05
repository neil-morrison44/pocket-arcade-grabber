use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::copy;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

#[derive(Serialize, Deserialize, Debug)]
struct DataSlot {
    filename: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Instance {
    data_slots: Vec<DataSlot>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AssetFile {
    instance: Instance,
}

#[derive(Serialize, Deserialize, Debug)]
struct DataFile {
    data: Instance,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigFile {
    file_host: String,
}

fn find_json_files(pocket_path: &str) -> Vec<PathBuf> {
    let mut json_file_paths: Vec<PathBuf> = Vec::new();
    let paths = fs::read_dir(pocket_path).unwrap();
    for path in paths {
        if let Some(extension) = path.as_ref().unwrap().path().extension() {
            if extension == "json" {
                json_file_paths.push(path.as_ref().unwrap().path())
            }
        }
        let md = fs::metadata(path.as_ref().unwrap().path()).unwrap();

        // Explicitly avoiding NeoGeo since there's loads of JSON files in there we won't find anything for
        let file_name = path.as_ref().unwrap().file_name();
        if md.is_dir() && (file_name != "Mazamars312.NeoGeo" && file_name != "ng") {
            let inner_json_paths = find_json_files(path.as_ref().unwrap().path().to_str().unwrap());
            for inner_json_path in inner_json_paths {
                json_file_paths.push(inner_json_path);
            }
        }
    }
    return json_file_paths;
}

fn read_asset_json(path: &PathBuf) -> Vec<String> {
    println!("Reading JSON asset: {}", path.display());
    let mut file_names: Vec<String> = Vec::new();
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let asset_data: AssetFile = serde_json::from_str(&contents).expect("Error reading file");
    for data_slot in asset_data.instance.data_slots {
        if data_slot.filename.is_some() {
            let file_name = data_slot.filename.unwrap();
            println!("Found DataSlot File: {}", &file_name);
            file_names.push(file_name);
        }
    }

    return file_names;
}

fn read_core_json(path: &PathBuf) -> Vec<String> {
    println!("Reading JSON asset: {}", path.display());
    let mut file_names: Vec<String> = Vec::new();
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let data_data: DataFile = serde_json::from_str(&contents).expect("Error reading file");
    for data_slot in data_data.data.data_slots {
        if data_slot.filename.is_some() {
            let file_name = data_slot.filename.unwrap();
            println!("Found DataSlot File: {}", &file_name);
            file_names.push(file_name);
        }
    }

    return file_names;
}

fn read_config_file(path: &str) -> ConfigFile {
    let config_path = &(path.to_owned() + "/arcade_grabber.json");
    let exists = Path::new(config_path).exists();
    if !exists {
        let mut file = fs::File::create(config_path).expect("Error when creating config file");
        file.write_all(b"{\"file_host\": \"\"}")
            .expect("Error when writing config file");
    }
    println!("{}", Path::new(config_path).exists());
    let contents = fs::read_to_string(config_path).expect("Unable to read config file");
    let config_data: ConfigFile =
        serde_json::from_str(&contents).expect("Error reading config file");
    return config_data;
}

fn find_common_path(json_path: &PathBuf) -> PathBuf {
    let mut current_path: PathBuf = json_path.clone();

    while !current_path.join("common").exists() {
        current_path = current_path.parent().unwrap().to_path_buf();
    }

    return current_path.join("common");
}

fn find_core_specific_asset_path(json_path: &PathBuf, asset_path: &str) -> Option<PathBuf> {
    let core_folder_name = json_path.parent().unwrap().file_name();

    let walker = WalkDir::new(asset_path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        if entry.as_ref().unwrap().path().file_name() == core_folder_name {
            return Some(entry.unwrap().path().to_path_buf());
        }
    }
    println!(
        "Filed to find asset folder for {:?}",
        core_folder_name.unwrap()
    );
    return None;
}

async fn try_to_download_file(file_name: String, file_host: &String, dest_folder_path: &PathBuf) {
    let new_file_path = Path::new(dest_folder_path).join(&file_name);
    let file_exists = new_file_path.exists();

    if file_exists {
        println!("`{}` already exists, skipping", &file_name);
        return;
    }

    let target = format!("{}/{}", file_host, file_name);
    let response = reqwest::get(&target).await;

    if response.is_err() || response.as_ref().unwrap().status() != 200 {
        println!("Unable to find {}, skipping", target);
        return;
    }
    let new_file_path = Path::new(dest_folder_path).join(file_name);
    let mut dest = fs::File::create(new_file_path).expect("error creating file");
    let content = response.unwrap().text().await.unwrap();
    copy(&mut content.as_bytes(), &mut dest).expect("error writing file");
}

fn find_core_data_files(cores_path: &str) -> Vec<PathBuf> {
    let json_paths = find_json_files(cores_path);

    let data_paths = json_paths
        .iter()
        .filter(|path| path.file_name().unwrap() == "data.json")
        .cloned()
        .collect::<Vec<PathBuf>>();

    return data_paths;
}

fn get_file_host(pocket_path: &String) -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        return args[2].to_owned();
    } else {
        let config = read_config_file(&pocket_path);
        return config.file_host.to_string();
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let pocket_path = &args[1];
    let json_paths = find_json_files(&(pocket_path.to_owned() + "/Assets"));
    let file_host = get_file_host(pocket_path);

    for json in json_paths {
        let file_names = read_asset_json(&json);
        let common_folder = find_common_path(&json);
        for file_name in file_names {
            try_to_download_file(file_name, &file_host, &common_folder).await;
        }
    }

    let core_data_paths = find_core_data_files(&pocket_path);

    for core_data_path in core_data_paths {
        let file_names = read_core_json(&core_data_path);
        let core_asset_folder = find_core_specific_asset_path(&core_data_path, pocket_path);

        if core_asset_folder.is_none() {
            continue;
        }
        let dest = core_asset_folder.unwrap();
        for file_name in file_names {
            try_to_download_file(file_name, &file_host, &dest).await;
        }
    }
}
