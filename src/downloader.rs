use std::{
    fs,
    io::{copy, Cursor},
    path::{Path, PathBuf},
};

pub async fn try_to_download_file(
    file_name: &String,
    file_host: &String,
    dest_folder_path: &PathBuf,
) {
    let new_file_path = Path::new(dest_folder_path).join(&file_name);
    let file_exists = new_file_path.exists();

    if file_exists {
        println!(
            "`{:?}/{}` already exists, skipping",
            dest_folder_path.as_os_str(),
            &file_name
        );
        return;
    }

    let target = format!("{}/{}", file_host, file_name);
    let response = reqwest::get(&target).await;

    match response {
        Err(e) => println!("Error downloading from {file_host}: ({e})"),
        Ok(r) => {
            if r.status() != 200 {
                println!("Unable to find {target}, skipping");
            } else {
                let new_file_path = Path::new(dest_folder_path).join(file_name);
                if let Ok(mut dest) = fs::File::create(&new_file_path) {
                    dbg!(&r);
                    if let Ok(content) = r.bytes().await {
                        let mut content_cusror = Cursor::new(content);
                        if let Ok(_success) = copy(&mut content_cusror, &mut dest) {
                            println!("Saved {target} to {:?}", &new_file_path.as_os_str());
                            return ();
                        }
                    }
                }
                println!(
                    "Failed to save downloaded file {target} to {:?}",
                    &new_file_path.as_os_str()
                );
            }
        }
    }
}
