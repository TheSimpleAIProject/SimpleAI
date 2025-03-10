use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::utils::prelude::*;

pub fn delete_node(name: String, version: Option<String>) -> Result<(), String> {
    let node_path = Path::new("nodes/").join(&name);
    let meta_path = node_path.join("meta.toml");

    if !node_path.exists() {
        return Err(format!("Node '{}' does not exist.", name));
    }

    if !meta_path.exists() {
        fs::remove_dir_all(&node_path).map_err(|e| e.to_string())?;
        return Ok(());
    }

    let meta_content = std::fs::read_to_string(&meta_path).map_err(|e| e.to_string())?;
    let mut meta: Metadata = toml::from_str(&meta_content).map_err(|e| e.to_string())?;

    if let Some(ver) = version {
        let version_index = meta.versions.iter().position(|v| v.version == ver);
        if let Some(index) = version_index {
            let env_hash = meta.versions[index].env.hash();
            let bin_path = node_path.join(&env_hash).join(format!("{}.bin", ver));

            if bin_path.exists() {
                fs::remove_file(&bin_path).map_err(|e| e.to_string())?;
            }

            // Remove empty environment folder
            let env_path = node_path.join(&env_hash);
            if env_path
                .read_dir()
                .map_err(|e| e.to_string())?
                .next()
                .is_none()
            {
                fs::remove_dir(&env_path).map_err(|e| e.to_string())?;
            }

            meta.versions.remove(index);
        } else {
            return Err(format!("Version '{}' not found for node '{}'.", ver, name));
        }
    } else {
        // Delete the entire node
        fs::remove_dir_all(&node_path).map_err(|e| e.to_string())?;
        return Ok(());
    }

    if meta.versions.is_empty() {
        fs::remove_dir_all(&node_path).map_err(|e| e.to_string())?;
    } else {
        let meta_toml = toml::to_string(&meta).map_err(|e| e.to_string())?;
        let tmp_meta_path = node_path.join("meta.toml.tmp");

        let mut tmp_meta_file = File::create(&tmp_meta_path).map_err(|e| e.to_string())?;
        tmp_meta_file
            .write_all(meta_toml.as_bytes())
            .map_err(|e| e.to_string())?;

        fs::rename(&tmp_meta_path, &meta_path).map_err(|e| e.to_string())?;
    }

    Ok(())
}
