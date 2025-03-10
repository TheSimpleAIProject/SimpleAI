use chrono::Utc;
use sai_backend::nms::save::save_node;
use sai_backend::utils::prelude::*;
use uuid::Uuid;

#[test]
fn test_create_code_node() {
    let node = Node {
        name: "test_code_node".to_string(),
        params: vec![],
        kind: NodeKind::Code {
            code: "fn main() { println!(\"Hello, world!\"); }".to_string(),
        },
        description: "A simple code node".to_string(),
        author: "Author".to_string(),
        compiled: None,
        version: Version {
            version: String::from("0.0.1"),
            env: Environment { deps: vec![] },
        },
        date: Utc::now(),
    };

    let res = save_node(node);
    println!("{:?}", res);

    assert!(res.is_ok());
}

#[test]
fn test_create_bundled_node() {
    let code_node1 = Node {
        name: "code_node1".to_string(),
        params: vec![],
        kind: NodeKind::Code {
            code: "fn main() { println!(\"Node 1\"); }".to_string(),
        },
        description: "First code node".to_string(),
        author: "Author".to_string(),
        compiled: None,
        version: Version {
            version: String::from("0.0.1"),
            env: Environment { deps: vec![] },
        },
        date: Utc::now(),
    };

    let code_node2 = Node {
        name: "code_node2".to_string(),
        params: vec![],
        kind: NodeKind::Code {
            code: "fn main() { println!(\"Node 2\"); }".to_string(),
        },
        description: "Second code node".to_string(),
        author: "Author".to_string(),
        compiled: None,
        version: Version {
            version: String::from("0.0.1"),
            env: Environment { deps: vec![] },
        },
        date: Utc::now(),
    };

    let mut nc: NodeContainer = NodeContainer::new();
    nc.push_context(StrongContext::from(code_node1));
    nc.push_context(StrongContext::from(code_node2));

    let bundled_node = Node {
        name: "bundled_node".to_string(),
        params: vec![],
        kind: NodeKind::Bundled { bundle: nc },
        description: "A bundled node".to_string(),
        author: "Author".to_string(),
        compiled: None,
        version: Version {
            version: String::from("0.0.1"),
            env: Environment { deps: vec![] },
        },
        date: Utc::now(),
    };

    let res = save_node(bundled_node);

    println!("{:?}", res);

    assert!(res.is_ok());
}

#[test]
fn test_create_complex_bundled_node() {
    let static_param = StrongParam::from(Param {
        name: "static_param".to_string(),
        desc: "A static parameter".to_string(),
        dtype: DType::String,
        kind: ParamKind::Static {
            value: "static_value".to_string(),
        },
    });

    let runtime_param1 = StrongParam::from(Param {
        name: "runtime_param1".to_string(),
        desc: "A runtime parameter".to_string(),
        dtype: DType::String,
        kind: ParamKind::Runtime {
            kind: RuntimeParamKind::Input,
            connection: None,
            id: Uuid::new_v4().as_u128(),
        },
    });

    let runtime_param2 = StrongParam::from(Param {
        name: "runtime_param2".to_string(),
        desc: "Another runtime parameter".to_string(),
        dtype: DType::String,
        kind: ParamKind::Runtime {
            kind: RuntimeParamKind::Output,
            connection: Some(WeakContext::from(runtime_param1.clone())),
            id: Uuid::new_v4().as_u128(),
        },
    });

    let code_node = Node {
        name: "complex_code_node".to_string(),
        params: vec![runtime_param1.clone(), runtime_param2.clone()],
        kind: NodeKind::Code {
            code: "fn main() { println!(\"Complex Node\"); }".to_string(),
        },
        description: "A complex code node".to_string(),
        author: "Author".to_string(),
        compiled: None,
        version: Version {
            version: String::from("0.0.1"),
            env: Environment {
                deps: vec![Dependency {
                    name: "serde".to_string(),
                    versions: vec!["1.0".to_string()],
                    lib: true,
                }],
            },
        },
        date: Utc::now(),
    };

    let code_node2 = Node {
        name: "complex_code_node".to_string(),
        params: vec![runtime_param1.clone(), runtime_param2.clone()],
        kind: NodeKind::Code {
            code: "fn main() { println!(\"Complex Node\"); }".to_string(),
        },
        description: "A complex code node".to_string(),
        author: "Author".to_string(),
        compiled: None,
        version: Version {
            version: String::from("0.0.2"),
            env: Environment {
                deps: vec![
                    Dependency {
                        name: "serde".to_string(),
                        versions: vec!["1.0".to_string()],
                        lib: true,
                    },
                    Dependency {
                        name: "torch".to_string(),
                        versions: vec!["2.0".to_string()],
                        lib: true,
                    },
                ],
            },
        },
        date: Utc::now(),
    };

    let mut nc: NodeContainer = NodeContainer::new();
    nc.push_context(StrongContext::from(code_node));

    let mut nc2: NodeContainer = NodeContainer::new();
    nc2.push_context(StrongContext::from(code_node2));

    let bundled_node = Node {
        name: "complex_bundled_node".to_string(),
        params: vec![static_param.clone()],
        kind: NodeKind::Bundled { bundle: nc },
        description: "A complex bundled node".to_string(),
        author: "Author".to_string(),
        compiled: None,
        version: Version {
            version: String::from("0.0.1"),
            env: Environment { deps: vec![] },
        },
        date: Utc::now(),
    };

    let bundled_node2 = Node {
        name: "complex_bundled_node".to_string(),
        params: vec![static_param.clone()],
        kind: NodeKind::Bundled { bundle: nc2 },
        description: "A complex bundled node".to_string(),
        author: "Author".to_string(),
        compiled: None,
        version: Version {
            version: String::from("0.0.2"),
            env: Environment { deps: vec![] },
        },
        date: Utc::now(),
    };

    let res = save_node(bundled_node);
    let res2 = save_node(bundled_node2);
    println!("{:?}", res);
    println!("{:?}", res2);

    assert!(res.is_ok());
    assert!(res2.is_ok());
}
