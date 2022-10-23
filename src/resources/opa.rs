use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::fs;
use std::path::Path;
use std::process::Command;
use crate::resources::file;

lazy_static! {
    // Global file lock
    pub static ref FILE_LOCK: RwLock<u32> = RwLock::new(0);
}

pub const OPA_PATH: &str = "/opt/verdictd/opa/";
pub const OPA_POLICY_SGX: &str = "sgxPolicy.rego";
pub const OPA_DATA_SGX: &str = "sgxData";

pub const OPA_POLICY_CSV: &str = "csvPolicy.rego";
pub const OPA_DATA_CSV: &str = "csvData";

pub fn set_reference(name: &str, reference: &str) -> Result<(), String> {
    let lock = FILE_LOCK.write();
    assert_eq!(*lock, 0);

    let name = String::from(OPA_PATH) + name;
    file::set(&name, reference)
}

/// Save the input raw policy file
/// Note that the OPA binary program needs to be installed and placed in the system path
pub fn set_policy(name: &str, policy: &str) -> Result<(), String> {
    let lock = FILE_LOCK.write();
    assert_eq!(*lock, 0);

    let src = String::from(OPA_PATH) + name;
    let bak = String::from(OPA_PATH) + name + ".bak";

    if Path::new(&src).exists() {
        fs::copy(&src, &bak).unwrap();
    }

    file::write(&src, policy)
        .map_err(|e| {
            if Path::new(&bak).exists() {
                fs::copy(&bak, &src).unwrap();
            }
            format!("Store policy failed: {}", e)
        })
        .and_then(|_| {
            let status = 
                Command::new("opa")
                .arg("check")
                .arg(&src)
                .status()
                .map_err(|_e| {
                    if Path::new(&bak).exists() {
                        fs::copy(&bak, &src).unwrap();
                    }
                    format!("Policy syntax check execution failed: {}", _e.to_string())
                });
            status
        })
        .and_then(|status| {
            match status.success() {
                true => {
                    if Path::new(&bak).exists() {
                        fs::remove_file(&bak).unwrap();
                    }
                    Ok(())
                }
                false => {
                    if Path::new(&bak).exists() {
                        fs::copy(&bak, &src).unwrap();
                    }
                    Err(format!("Policy syntax check failed"))         
                }
            }
        })
}

// Export existing policy from verdictd
pub fn export(name: &str) -> Result<String, String> {
    let lock = FILE_LOCK.read();
    assert_eq!(*lock, 0);
    let name = String::from(OPA_PATH) + name;
    file::export_string(&name)
}

pub fn default() -> Result<(), String> {
    if !Path::new(&OPA_PATH.to_string()).exists() {
        fs::create_dir_all(OPA_PATH)
            .map_err(|_| format!("create {:?} failed", OPA_PATH))?;
    }

    if !Path::new(&(OPA_PATH.to_string() + OPA_POLICY_SGX)).exists() {
        info!("{} isn't exist", OPA_POLICY_SGX);
        let policy = r#"
package policy

# By default, deny requests.
default allow = false

allow {
    mrEnclave_is_grant
    mrSigner_is_grant
    input.productId >= data.productId
    input.svn >= data.svn
}

mrEnclave_is_grant {
    count(data.mrEnclave) == 0
}
mrEnclave_is_grant {
    count(data.mrEnclave) > 0
    input.mrEnclave == data.mrEnclave[_]
}

mrSigner_is_grant {
    count(data.mrSigner) == 0
}
mrSigner_is_grant {
    count(data.mrSigner) > 0
    input.mrSigner == data.mrSigner[_]
}
"#;
        file::write(&(String::from(OPA_PATH) + OPA_POLICY_SGX), &policy.to_string())
            .map_err(|e| format!("Set {} failed with error {:?}", OPA_POLICY_SGX, e))?;
    }

    if !Path::new(&(OPA_PATH.to_string() + OPA_DATA_SGX)).exists() {
        info!("{} isn't exist", OPA_DATA_SGX);
        let sgx_data = r#"{
    "mrEnclave": [],
    "mrSigner": [],
    "productId": 0,
    "svn": 0
}"#;

        let lock = FILE_LOCK.write();
        assert_eq!(*lock, 0);
        
        file::write(&(String::from(OPA_PATH) + OPA_DATA_SGX), &sgx_data.to_string())
            .map_err(|e| format!("Set {} failed with error {:?}", OPA_DATA_SGX, e))?;
    }    

    Ok(())
}
