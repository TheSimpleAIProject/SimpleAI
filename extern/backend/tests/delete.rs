use std::path::Path;

use chrono::Utc;
use sai_backend::{
    nms::{delete::delete_node, save::save_node},
    utils::prelude::*,
};

#[test]
fn test_delete_entire_node() {
    let node = Node {
        name: "test_delete_node".to_string(),
        params: vec![],
        kind: NodeKind::Code {
            code: "fn main() { println!(\"Delete me\"); }".to_string(),
        },
        description: "A node to be deleted".to_string(),
        author: "Author".to_string(),
        compiled: None,
        version: Version {
            version: String::from("1.0.0"),
            env: Environment { deps: vec![] },
        },
        date: Utc::now(),
    };

    let res = save_node(node.clone());
    assert!(res.is_ok());

    let del_res = delete_node(String::from("test_delete_node"), None);
    assert!(del_res.is_ok());

    let node_path = Path::new("nodes/").join("test_delete_node");
    assert!(!node_path.exists(), "Node folder should be deleted");
}

#[test]
fn test_delete_specific_version() {
    let node_v1 = Node {
        name: "test_delete_version".to_string(),
        params: vec![],
        kind: NodeKind::Code {
            code: "fn main() { println!(\"Version 1\"); }".to_string(),
        },
        description: "A node with multiple versions".to_string(),
        author: "Author".to_string(),
        compiled: None,
        version: Version {
            version: String::from("1.0.0"),
            env: Environment { deps: vec![] },
        },
        date: Utc::now(),
    };

    let node_v2 = Node {
        name: "test_delete_version".to_string(),
        params: vec![],
        kind: NodeKind::Code {
            code: "fn main() { println!(\"Version 2\"); }".to_string(),
        },
        description: "A node with multiple versions".to_string(),
        author: "Author".to_string(),
        compiled: None,
        version: Version {
            version: String::from("2.0.0"),
            env: Environment { deps: vec![] },
        },
        date: Utc::now(),
    };

    assert!(save_node(node_v1.clone()).is_ok());
    assert!(save_node(node_v2.clone()).is_ok());

    let del_res = delete_node(
        String::from("test_delete_version"),
        Some(String::from("1.0.0")),
    );
    assert!(del_res.is_ok());

    let meta_path = Path::new("nodes/test_delete_version/meta.toml");
    assert!(meta_path.exists(), "Meta file should still exist");

    let meta_content = std::fs::read_to_string(meta_path).unwrap();
    let meta: Metadata = toml::from_str(&meta_content).unwrap();

    assert_eq!(meta.versions.len(), 1);
    assert_eq!(meta.versions[0].version, "2.0.0");
}

#[test]
fn test_delete_nonexistent_node() {
    let del_res = delete_node(String::from("nonexistent_node"), None);
    assert!(del_res.is_err());
}

#[test]
fn test_delete_nonexistent_version() {
    let node = Node {
        name: "test_delete_invalid_version".to_string(),
        params: vec![],
        kind: NodeKind::Code {
            code: "fn main() { println!(\"Only one version\"); }".to_string(),
        },
        description: "A node with a single version".to_string(),
        author: "Author".to_string(),
        compiled: None,
        version: Version {
            version: String::from("1.0.0"),
            env: Environment { deps: vec![] },
        },
        date: Utc::now(),
    };

    assert!(save_node(node.clone()).is_ok());

    let del_res = delete_node(
        String::from("test_delete_invalid_version"),
        Some(String::from("2.0.0")),
    );
    assert!(del_res.is_err());
}
