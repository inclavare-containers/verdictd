use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::*;

const RESOURCE_DIR: &str = "/opt/verdictd/resources";

#[derive(Debug)]
pub struct Resources {
    pub resource_dir: String,
}

impl Resources {
    pub fn default() -> Result<Self> {
        Self::new(RESOURCE_DIR)
    }

    pub fn new(resource_dir: &str) -> Result<Self> {
        let res = Self {
            resource_dir: resource_dir.into(),
        };

        res.prepare_default_resources()?;

        Ok(res)
    }

    fn prepare_default_resources(&self) -> Result<()> {
        let resource_dir = &self.resource_dir;
        fs::create_dir_all(format!("{resource_dir}/default/gpg-public-config"))
            .context("init gpg-public key dir")?;

        fs::create_dir_all(format!("{resource_dir}/default/security-policy"))
            .context("init image policy dir")?;

        let default_policy_path = format!("{resource_dir}/default/security-policy/test");
        if !Path::new(&default_policy_path).exists() {
            let policy = r#"{
                "default": [
                    {
                        "type": "insecureAcceptAnything"
                    }
                ],
            }"#;
            fs::write(&default_policy_path, policy).context("write default policy")?;
        }

        fs::create_dir_all(format!("{resource_dir}/default/sigstore-config"))
            .context("init sigstore config dir")?;
        let default_sigstore_config_path = format!("{resource_dir}/default/sigstore-config/test");
        if !Path::new(&default_sigstore_config_path).exists() {
            let sigstore = "default:
    sigstore: file:///var/lib/containers/sigstore
";
            fs::write(&default_sigstore_config_path, sigstore).context("write default policy")?;
        }

        fs::create_dir_all(format!("{resource_dir}/default/cosign-public-key"))
            .context("init cosign public key dir")?;

        fs::create_dir_all(format!("{resource_dir}/default/credential"))
            .context("init credential dir")?;

        Ok(())
    }

    pub fn get_resource(&self, resource_id: &str) -> Result<Vec<u8>> {
        let mut path = PathBuf::from(&self.resource_dir);
        path.push(resource_id);
        let data = fs::read(path).context("read resource")?;

        Ok(data)
    }

    pub fn set_resource(&self, resource_id: &str, resource: &[u8]) -> Result<()> {
        let mut path = PathBuf::from(&self.resource_dir);
        path.push(resource_id);

        // TODO: judge whether in self.key_dir
        if let Some(parent_dir) = path.parent() {
            fs::create_dir_all(parent_dir).context("create resource subdir")?;
        }

        info!("set resource for resource file: {:?}", path);

        fs::write(path, resource).context("write resource")?;

        Ok(())
    }

    pub fn get_resource_info(&self, resource_id: &str) -> Result<usize> {
        let data = self.get_resource(resource_id)?;
        let base64_encoded = base64::encode(data);
        Ok(base64_encoded.len())
    }
}
