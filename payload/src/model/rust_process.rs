use crate::common::*;

#[doc = "Struct to represent a Rust process"]
#[derive(Debug, Serialize, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct RustProcess {
    pub binary_path: String,
    pub temp_exe_name: String,
    pub working_dir_path: String,
}

impl RustProcess {
    pub fn new(binary_path: String, temp_exe_name: String, working_dir_path: String) -> Self {
        RustProcess {
            binary_path,
            temp_exe_name,
            working_dir_path,
        }
    }
}
