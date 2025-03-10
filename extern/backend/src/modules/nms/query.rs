use std::path::Path;

use crate::utils::prelude::{save::*, *};
use walkdir::WalkDir;

pub fn get_all_nodes() -> Result<NodeContainer, String> {
    let nodes_dir = Path::new("nodes");
    if !nodes_dir.exists() {
        return Ok(NodeContainer::default());
    }

    let mut nc = NodeContainer::default();

    for entry in WalkDir::new(nodes_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "bin"))
    {
        let path = entry.path();
        let data =
            std::fs::read(path).map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

        let save_node: SaveNode = bincode::deserialize(&data)
            .map_err(|e| format!("Failed to deserialize {}: {}", path.display(), e))?;

        let node = Node::from(save_node);
        nc.push_context(StrongNode::from(node));
    }

    Ok(nc)
}

/// This function searches through all available Nodes and returns a NodeContainer containing all Nodes available for the inferred environment.
pub fn query(query_filter: Vec<QueryFilter>) -> NodeContainer {
    let all_nodes = get_all_nodes().expect("Error walking directory!");

    all_nodes
        .iter()
        .filter(|node| {
            query_filter.iter().all(|filter| {
                filter
                    .clone()
                    .is_ok(node.context.try_lock().unwrap().to_owned())
            })
        })
        .cloned()
        .collect()
}
