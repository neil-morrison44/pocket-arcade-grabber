use crate::files::FileAndDestination;
use assets::find_asset_files;
use cores::find_core_files;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

mod assets;
mod cores;
mod downloader;
mod files;
mod json_types;

fn read_config_file(path: &str) -> json_types::ConfigFile {
    let config_path = &(path.to_owned() + "/arcade_grabber.json");
    let exists = Path::new(config_path).exists();
    if !exists {
        let mut file = fs::File::create(config_path).expect("Error when creating config file");
        file.write_all(b"{\"file_host\": \"\"}")
            .expect("Error when writing config file");
    }
    let contents = fs::read_to_string(config_path).expect("Unable to read config file");
    let config_data: json_types::ConfigFile =
        serde_json::from_str(&contents).expect("Error reading config file");
    return config_data;
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
    let file_host = get_file_host(pocket_path);

    if let Ok(path) = PathBuf::from_str(pocket_path) {
        let mut core_files = find_core_files(&path);
        core_files.append(&mut find_asset_files(&path));

        for FileAndDestination {
            file_name,
            destination,
        } in core_files
        {
            downloader::try_to_download_file(&file_name, &file_host, &destination).await;
        }
    }
}
