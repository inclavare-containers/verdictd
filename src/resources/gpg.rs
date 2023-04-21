use crate::resources::file;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::fs;
use std::path::Path;

lazy_static! {
    // Global file lock
    pub static ref FILE_LOCK: RwLock<u32> = RwLock::new(0);
}

pub const GPG_PATH: &str = "/opt/verdictd/gpg/";
pub const GPG_KEYRING: &str = "/opt/verdictd/gpg/keyring.gpg";

pub fn export_base64() -> Result<String, String> {
    let lock = FILE_LOCK.read();
    assert_eq!(*lock, 0);

    file::export_base64(GPG_KEYRING).map_err(|e| format!("export GPG keyring failed:{:?}", e))
}

pub fn size_base64() -> Result<usize, String> {
    let lock = FILE_LOCK.read();
    assert_eq!(*lock, 0);

    file::export_base64(GPG_KEYRING)
        .map_err(|e| format!("Fetch GPG keyring size failed:{:?}", e))
        .and_then(|content| Ok(content.len()))
}

pub fn default() -> Result<(), String> {
    if !Path::new(&GPG_PATH.to_string()).exists() {
        fs::create_dir_all(GPG_PATH).map_err(|_| format!("create {:?} failed", GPG_PATH))?;
    }

    Ok(())
}
