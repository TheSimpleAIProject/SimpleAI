use crate::nms::check_name;
use crate::utils::prelude::{save::*, *};
use anyhow::Result;
use std::fs::{self, create_dir, create_dir_all, File};
use std::io::Write;
use std::path::Path;
use toml;

// #[cfg(feature = "desktop")]
pub fn save_node(node: Node) -> Result<(), String> {
    let name = node.name.clone();

    if !check_name(name.clone()) {
        return Err(format!(
            "Node name {} is not allowed! Please only use letters, dashes and underscores.",
            name
        ));
    }

    let node_path = Path::new("nodes/").join(&name);
    if !node_path.exists() {
        create_dir_all(&node_path).map_err(|e| e.to_string())?;
    }

    let meta_path = node_path.join("meta.toml");
    let mut meta: Metadata = if meta_path.exists() {
        let meta_content = std::fs::read_to_string(&meta_path).map_err(|e| e.to_string())?;
        toml::from_str(&meta_content).map_err(|e| e.to_string())?
    } else {
        Metadata::from(node.clone())
    };

    let new_version = Version {
        version: node.version.version.clone(),
        env: node.clone().get_full_env(),
    };

    let existing_index = meta
        .versions
        .iter()
        .position(|v| v.version == new_version.version);

    if let Some(index) = existing_index {
        let existing_env_hash = meta.versions[index].env.hash();
        let new_env_hash = new_version.env.hash();

        if existing_env_hash != new_env_hash {
            // Remove old binary
            let old_bin_path = node_path
                .join(&existing_env_hash)
                .join(format!("{}.bin", new_version.version));
            if old_bin_path.exists() {
                fs::remove_file(&old_bin_path).map_err(|e| e.to_string())?;
            }

            // Remove empty env folder
            let old_env_path = node_path.join(&existing_env_hash);
            if old_env_path.exists()
                && old_env_path
                    .read_dir()
                    .map_err(|e| e.to_string())?
                    .next()
                    .is_none()
            {
                fs::remove_dir(&old_env_path).map_err(|e| e.to_string())?;
            }

            // Update the existing entry
            meta.versions[index] = new_version.clone();
        }
    } else {
        meta.versions.push(new_version.clone());
    }

    let meta_toml = toml::to_string(&meta).map_err(|e| e.to_string())?;
    let tmp_meta_path = node_path.join("meta.toml.tmp");

    let mut tmp_meta_file = File::create(&tmp_meta_path).map_err(|e| e.to_string())?;
    tmp_meta_file
        .write_all(meta_toml.as_bytes())
        .map_err(|e| e.to_string())?;

    // Atomically replace the old meta file
    fs::rename(&tmp_meta_path, &meta_path).map_err(|e| e.to_string())?;

    let env_hash = new_version.env.hash();
    let env_path = node_path.join(&env_hash);
    if !env_path.exists() {
        create_dir(&env_path).map_err(|e| e.to_string())?;
    }

    let bin_path = env_path.join(format!("{}.bin", new_version.version));

    let node_bin = bincode::serialize(&SaveNode::from(node)).map_err(|e| e.to_string())?;
    let mut node_file = File::create(&bin_path).map_err(|e| e.to_string())?;
    node_file.write_all(&node_bin).map_err(|e| e.to_string())?;

    // Remove empty node folder if necessary
    if node_path
        .read_dir()
        .map_err(|e| e.to_string())?
        .next()
        .is_none()
    {
        fs::remove_dir(&node_path).map_err(|e| e.to_string())?;
    }

    Ok(())
}
