use super::prelude::*;
use derive_builder::Builder;
use onnx_ir::NodeType;
use serde::{Deserialize, Serialize};
// -------------------- SAVE NODES -------------------- //
#[derive(Clone, Serialize, Deserialize)]
pub enum SaveNodeKind {
    Onnx { onnx: SaveOnnxNode },
    Bundled { bundle: Vec<SaveNode> },
}

#[derive(Builder, Clone, Serialize, Deserialize)]
pub struct SaveNode {
    pub name: String,
    pub version: Version,
    pub kind: SaveNodeKind,
    pub description: String,
    pub author: String,
    pub date: Date,
    pub position: Option<(f64, f64)>,
}

#[derive(Builder, Clone, Serialize, Deserialize)]
pub struct SaveOnnxNode {
    #[serde(with = "OnnxNodeType")]
    pub node_type: NodeType,
    pub name: String,
    pub inputs: Vec<SaveParam>,
    pub outputs: Vec<SaveParam>,
}

impl From<NodeContainer> for SaveNodeKind {
    fn from(nodes: NodeContainer) -> Self {
        let mut save_nodes = Vec::new();
        for context in nodes.tree.iter() {
            let node = context.context.try_lock().unwrap().to_owned();
            save_nodes.push(node.into());
        }
        SaveNodeKind::Bundled { bundle: save_nodes }
    }
}

impl From<Node> for SaveNode {
    fn from(node: Node) -> Self {
        let mut binding = SaveNodeBuilder::default();
        let mut builder = binding
            .name(node.name.clone())
            .description(node.description.clone())
            .author(node.author.clone())
            .version(Version {
                version: node.version.version.clone(),
                env: node.clone().get_full_env(),
            })
            .position(node.position)
            .date(node.date);

        if let NodeKind::Onnx { onnx } = node.kind {
            builder = builder.kind(SaveNodeKind::Onnx { onnx: onnx.into() });
        } else if let NodeKind::Bundled { bundle } = node.kind {
            builder = builder.kind(bundle.into());
        }

        builder.build().unwrap()
    }
}

impl From<OnnxNode> for SaveOnnxNode {
    fn from(node: OnnxNode) -> Self {
        let mut binding = SaveOnnxNodeBuilder::default();
        let mut builder = binding.node_type(node.node_type).name(node.name);

        builder = builder.inputs(
            node.inputs
                .iter()
                .map(|param| SaveParam::from(param.clone()))
                .collect(),
        );
        builder = builder.outputs(
            node.outputs
                .iter()
                .map(|param| SaveParam::from(param.clone()))
                .collect(),
        );

        builder.build().unwrap()
    }
}
