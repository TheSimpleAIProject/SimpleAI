use super::prelude::{save::*, *};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
// -------------------- SAVE NODES -------------------- //
#[derive(Clone, Serialize, Deserialize)]
pub enum SaveNodeKind {
    Code { code: String },
    Bundled { bundle: Vec<SaveNode> },
}

#[derive(Builder, Clone, Serialize, Deserialize)]
pub struct SaveNode {
    pub name: String,
    pub params: Vec<SaveParam>,
    pub kind: SaveNodeKind,
    pub description: String,
    pub author: String,
    pub compiled: Option<String>,
    pub version: Version,
    pub date: Date,
}

impl From<NodeContainer> for SaveNodeKind {
    fn from(nodes: NodeContainer) -> Self {
        let mut save_nodes: Vec<SaveNode> = Vec::new();
        for context in nodes.tree.iter() {
            let node: Node = context.context.try_lock().unwrap().to_owned();
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
            .compiled(node.compiled.clone())
            .version(Version {
                version: node.version.version.clone(),
                env: node.clone().get_full_env(),
            })
            .date(node.date);

        if let NodeKind::Code { code } = node.kind {
            builder = builder.kind(SaveNodeKind::Code { code });
        } else if let NodeKind::Bundled { bundle } = node.kind {
            builder = builder.kind(bundle.into());
        }

        builder = builder.params(
            node.params
                .iter()
                .map(|param| SaveParam::from(param.clone()))
                .collect(),
        );

        builder.build().unwrap()
    }
}
