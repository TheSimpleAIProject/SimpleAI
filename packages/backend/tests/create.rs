use chrono::Utc;
use simple_ai_backend::modules::{
    nodes::save::save_node,
    utils::{
        dtype::DType,
        node::{Node, NodeKind, OnnxNode},
        param::{ParamKind, RuntimeParamKind, StrongParam},
        prelude::{Environment, NodeContainer, ParamBuilder, StrongContext, Version, WeakContext},
    },
};

#[test]
fn test_create_bundled_node() {
    let node1_param_out = ParamBuilder::default()
        .name("Param1".into())
        .desc("A param".into())
        .dtype(DType::F32)
        .kind(ParamKind::Static { value: "5".into() })
        .build()
        .expect("Failed to build param");
    let node2_param_in = ParamBuilder::default()
        .name("Param2".into())
        .desc("Another param".into())
        .dtype(DType::F32)
        .kind(ParamKind::Static { value: "5".into() })
        .build()
        .expect("Failed to build param");
    let node1_param_out = StrongParam::from(node1_param_out);
    let node2_param_in = StrongParam::from(node2_param_in);
    node1_param_out.context.try_lock().unwrap().kind = ParamKind::Runtime {
        kind: RuntimeParamKind::Output,
        connection: Some(WeakContext::from(node2_param_in.clone())),
        id: 1,
    };
    node2_param_in.context.try_lock().unwrap().kind = ParamKind::Runtime {
        kind: RuntimeParamKind::Input,
        connection: Some(WeakContext::from(node1_param_out.clone())),
        id: 2,
    };

    let node1 = Node {
        name: "node1".to_string(),
        kind: NodeKind::Onnx {
            onnx: OnnxNode {
                node_type: onnx_ir::NodeType::Relu,
                name: "node1".into(),
                inputs: vec![],
                outputs: vec![StrongParam::from(node1_param_out)],
            },
        },
        description: "First code node".to_string(),
        author: "It's mee".to_string(),
        version: Version {
            version: String::from("0.0.1"),
            env: Environment { deps: vec![] },
        },
        date: Utc::now(),
        position: Some((20.0, 50.0)),
    };

    let mut nc: NodeContainer = NodeContainer::new();
    nc.push_context(StrongContext::from(node1));

    let bundled_node = Node {
        name: "bundled_node".to_string(),
        kind: NodeKind::Bundled { bundle: nc },
        description: "A bundled node".to_string(),
        author: "Author".to_string(),
        version: Version {
            version: String::from("0.0.1"),
            env: Environment { deps: vec![] },
        },
        date: Utc::now(),
        position: None,
    };

    let res = save_node(bundled_node);
    println!("{:?}", res);

    assert!(res.is_ok());
}
