use std::{fs, path::PathBuf};

use anyhow::*;

const KEY_DIR: &str = "/opt/verdictd/keys";

#[derive(Debug)]
pub struct KeyManager {
    pub key_dir: String,
}

impl KeyManager {
    pub fn default() -> Result<Self> {
        Self::new(KEY_DIR)
    }

    pub fn new(key_dir: &str) -> Result<Self> {
        let res = Self {
            key_dir: key_dir.into(),
        };

        res.prepare_default_resources()?;

        Ok(res)
    }

    fn prepare_default_resources(&self) -> Result<()> {
        let key_dir = &self.key_dir;
        fs::create_dir_all(key_dir).context("init  key dir")?;

        Ok(())
    }

    pub fn get_key(&self, key_id: &str) -> Result<Vec<u8>> {
        let mut path = PathBuf::from(&self.key_dir);
        path.push(key_id);
        info!("read key {:?}", path);
        let data = fs::read(path).context("read key")?;

        Ok(data)
    }

    pub fn set_key(&self, key_id: &str, key: &[u8]) -> Result<()> {
        let mut path = PathBuf::from(&self.key_dir);
        path.push(key_id);

        // TODO: judge whether in self.key_dir
        if let Some(parent_dir) = path.parent() {
            fs::create_dir_all(parent_dir).context("create key subdir")?;
        }

        info!("set key for keyFile: {:?}", path);

        fs::write(path, key).context("write key")?;
        Ok(())
    }
}
