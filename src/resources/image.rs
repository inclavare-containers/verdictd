use crate::resources::file;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::fs;
use std::path::Path;

lazy_static! {
    // Global file lock
    pub static ref FILE_LOCK: RwLock<u32> = RwLock::new(0);
}

pub const IMAGE_PATH: &str = "/opt/verdictd/image/";
pub const POLICY: &str = "/opt/verdictd/image/policy.json";
pub const SIGSTORE: &str = "/opt/verdictd/image/sigstore.yaml";
pub const COSIGN: &str = "/opt/verdictd/image/cosign.key";
pub const CREDENTIAL: &str = "/opt/verdictd/image/auth.json";

pub fn export(name: &str) -> Result<String, String> {
    let lock = FILE_LOCK.read();
    assert_eq!(*lock, 0);

    file::export_string(name)
}

pub fn export_base64(name: &str) -> Result<String, String> {
    let lock = FILE_LOCK.read();
    assert_eq!(*lock, 0);

    file::export_base64(name)
}

pub fn set(name: &str, content: &str) -> Result<(), String> {
    let lock = FILE_LOCK.write();
    assert_eq!(*lock, 0);

    file::set(name, content)
}

pub fn size_base64(name: &str) -> Result<usize, String> {
    let lock = FILE_LOCK.write();
    assert_eq!(*lock, 0);

    file::export_base64(name)
        .map_err(|e| format!("Fetch {} size failed:{:?}", name, e))
        .and_then(|content| Ok(content.len()))
}

pub fn default() -> Result<(), String> {
    if !Path::new(&IMAGE_PATH.to_string()).exists() {
        fs::create_dir_all(IMAGE_PATH).map_err(|_| format!("create {:?} failed", IMAGE_PATH))?;
    }

    if !Path::new(&POLICY.to_string()).exists() {
        info!("{} isn't exist", POLICY);
        let policy = r#"{
    "default": [
        {
            "type": "insecureAcceptAnything"
        }
    ],
}"#;

        file::write(&String::from(POLICY), &policy.to_string())
            .map_err(|e| format!("Set {} failed with error {:?}", POLICY, e))?;
    }

    if !Path::new(&SIGSTORE.to_string()).exists() {
        info!("{} isn't exist", SIGSTORE);
        let sigstore = "default:
    sigstore: file:///var/lib/containers/sigstore
";

        file::write(&String::from(SIGSTORE), sigstore)
            .map_err(|e| format!("Set {} failed with error {:?}", SIGSTORE, e))?;
    }

    Ok(())
}
