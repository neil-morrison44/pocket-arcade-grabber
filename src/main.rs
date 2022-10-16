use crate::files::FileAndDestination;
use assets::find_asset_files;
use clap::Parser;
use cores::find_core_files;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process;

mod assets;
mod cores;
mod downloader;
mod files;
mod json_types;

/// Grabs your files you need for Analgoue Pocket Arcade Cores
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the root of your Pocket SD card
    path: PathBuf,
    /// (Optional) Pass in a file host to use instead of reading from the config json
    #[arg(short, long)]
    file_host: Option<String>,
}

fn read_config_file(path: &PathBuf) -> json_types::ConfigFile {
    let config_path = Path::new(path).join("arcade_grabber.json");
    let exists = config_path.exists();
    if !exists {
        let mut file = fs::File::create(&config_path).expect("Error when creating config file");
        file.write_all(b"{\"file_host\": \"\"}")
            .expect("Error when writing config file");
    }
    let contents = fs::read_to_string(&config_path).expect("Unable to read config file");
    let config_data: json_types::ConfigFile =
        serde_json::from_str(&contents).expect("Error reading config file");
    return config_data;
}

fn get_file_host(pocket_path: &PathBuf, arg_file_host: Option<String>) -> String {
    if let Some(host) = arg_file_host {
        return host;
    } else {
        let config = read_config_file(&pocket_path);
        return config.file_host.to_string();
    }
}

#[tokio::main]
async fn main() {
    // let args: Vec<String> = env::args().collect();
    let args = Args::parse();
    let pocket_path = args.path;
    let path = PathBuf::from(pocket_path);

    if !path.exists() {
        eprintln!("Error, unable to find: {:?}", path.as_os_str());
        process::exit(1);
    }

    let file_host = get_file_host(&path, args.file_host);
    let mut core_files = find_core_files(&path);
    core_files.append(&mut find_asset_files(&path));

    for FileAndDestination {
        file_name,
        destination,
    } in core_files
    {
        downloader::try_to_download_file(&file_name, &file_host, &destination).await;
    }

    process::exit(0);
}
