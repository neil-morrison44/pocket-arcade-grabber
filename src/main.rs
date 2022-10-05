use std::env;
use std::fs;
use std::io::copy;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct DataSlot {
    id: i32,
    filename: String,
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
        if md.is_dir() {
            let inner_json_paths = find_json_files(path.as_ref().unwrap().path().to_str().unwrap());
            for inner_json_path in inner_json_paths {
                json_file_paths.push(inner_json_path);
            }
        }
    }
    return json_file_paths;
}

fn read_json(path: &PathBuf) -> Vec<String> {
    println!("Reading JSON asset: {}", path.display());
    let mut file_names: Vec<String> = Vec::new();
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let asset_data: AssetFile = serde_json::from_str(&contents).expect("Error reading file");
    for data_slot in asset_data.instance.data_slots {
        println!("Found DataSlot File: {}", data_slot.filename);
        file_names.push(data_slot.filename);
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
    let common_folder_path = json_path.parent().unwrap().parent().unwrap().join("common");
    return common_folder_path;
}

async fn try_to_download_file(file_name: String, file_host: &String, common_folder_path: &PathBuf) {
    let target = format!("{}/{}", file_host, file_name);
    let response = reqwest::get(&target).await;

    if response.is_err() || response.as_ref().unwrap().status() != 200 {
        println!("Unable to find {}, skipping", target);
        return;
    }
    let new_file_path = Path::new(common_folder_path).join(file_name);
    let mut dest = fs::File::create(new_file_path).expect("error creating file");
    let content = response.unwrap().text().await.unwrap();
    copy(&mut content.as_bytes(), &mut dest).expect("error writing file");
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let pocket_path = &args[1];
    let config = read_config_file(&pocket_path);
    let json_paths = find_json_files(&(pocket_path.to_owned() + "/Assets"));

    for json in json_paths {
        let json_file_names = read_json(&json);
        let common_folder = find_common_path(&json);
        for json_file_name in json_file_names {
            try_to_download_file(json_file_name, &config.file_host, &common_folder).await;
        }
    }
}
