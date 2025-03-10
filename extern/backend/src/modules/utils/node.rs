use super::prelude::{save::*, *};
use derive_builder::Builder;
use std::collections::HashMap;
// -------------------- NODE KIND -------------------- //
#[derive(Clone, PartialEq)]
pub enum NodeKind {
    Code { code: String },
    Bundled { bundle: NodeContainer },
}

// -------------------- NODE -------------------- //
pub type StrongNode = StrongContext<Node>;
pub type WeakNode = WeakContext<Node>;

#[derive(Builder, Clone, PartialEq)]
pub struct Node {
    pub name: String,
    pub params: Vec<StrongParam>,
    pub version: Version,
    pub kind: NodeKind,
    pub description: String,
    pub author: String,
    pub compiled: Option<String>, // or and bytes...
    pub date: Date,
}

impl Node {
    pub fn get_full_env(self) -> Environment {
        let mut env = self.version.env;
        if let NodeKind::Bundled { bundle } = self.kind {
            for context in bundle.tree.iter() {
                let node: Node = context.context.try_lock().unwrap().to_owned();
                env = node.get_full_env().merge(&env).unwrap();
            }
        }

        env
    }
}

impl From<SaveNode> for Node {
    fn from(node: SaveNode) -> Self {
        let mut param_map: HashMap<u128, StrongParam> = HashMap::new();
        let mut params = Vec::new();

        // First pass: instantiate Params without setting connections
        for save_param in &node.params {
            let strong_param = StrongParam::from(Param {
                name: save_param.name.clone(),
                desc: save_param.desc.clone(),
                dtype: save_param.dtype.clone(),
                kind: match &save_param.kind {
                    SaveParamKind::Static { value } => ParamKind::Static {
                        value: value.clone(),
                    },
                    SaveParamKind::Runtime { kind, .. } => ParamKind::Runtime {
                        kind: kind.clone(),
                        connection: None,
                        id: save_param.id,
                    },
                },
            });
            param_map.insert(save_param.id, strong_param.clone());
            params.push(strong_param);
        }

        // Second pass: resolve connections
        for strong_param in &params {
            let mut param = strong_param.context.try_lock().unwrap();
            if let ParamKind::Runtime { connection, id, .. } = &mut param.kind {
                if let Some(target) = param_map.get(id) {
                    *connection = Some(WeakContext::from(target.clone()));
                }
            }
        }

        let mut binding = NodeBuilder::default();
        let builder = binding
            .name(node.name)
            .description(node.description)
            .author(node.author)
            .compiled(node.compiled)
            .version(Version {
                version: node.version.version,
                env: node.version.env,
            })
            .date(node.date)
            .params(params);

        let kind = match node.kind {
            SaveNodeKind::Code { code } => NodeKind::Code { code },
            SaveNodeKind::Bundled { bundle } => {
                let mut node_container = NodeContainer::new();
                for save_node in bundle {
                    node_container.push_context(StrongNode::from(Node::from(save_node)));
                }
                NodeKind::Bundled {
                    bundle: node_container,
                }
            }
        };

        builder.kind(kind).build().expect("Failed to build Node")
    }
}
