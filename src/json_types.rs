use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DataSlot {
    pub filename: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Instance {
    pub data_slots: Vec<DataSlot>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoreMetadata {
    pub platform_ids: Vec<String>,
    pub shortname: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Core {
    pub metadata: CoreMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetFile {
    pub instance: Instance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataFile {
    pub data: Instance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub file_host: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoreFile {
    pub core: Core,
}
