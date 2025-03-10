use super::prelude::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
// -------------------- METADATA -------------------- //
#[derive(PartialEq, Builder, Clone, Serialize, Deserialize)]
pub struct Version {
    pub version: String,
    pub env: Environment,
}

#[derive(Builder, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub author: String,
    pub date: Date,
    pub versions: Vec<Version>,
}

impl From<Node> for Metadata {
    fn from(node: Node) -> Self {
        MetadataBuilder::default()
            .name(node.name.clone())
            .description(node.description.clone())
            .author(node.author.clone())
            .date(node.date)
            .versions(vec![Version {
                version: node.version.version.clone(),
                env: node.get_full_env(),
            }])
            .build()
            .expect("Internal error")
    }
}
