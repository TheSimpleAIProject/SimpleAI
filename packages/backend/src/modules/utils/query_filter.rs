use crate::modules::utils::prelude::*;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

// ---------------- QUERY FILTER ---------------- //
#[derive(Clone)]
pub enum NodeQueryFilter {
    Older { date: Date },
    Newer { date: Date },
    Author { author: String },
    Environment { env: Environment },
    Name { name: String },
}

#[derive(Clone)]
pub enum ProjectQueryFilter {
    Older { date: Date },
    Newer { date: Date },
    Name { name: String },
    Author { author: String },
    Node { node: String },
}

impl NodeQueryFilter {
    pub fn is_ok(self, node: Node) -> bool {
        match self {
            NodeQueryFilter::Name { name } => {
                let matcher = SkimMatcherV2::default();
                matcher.fuzzy_match(&node.name, &name).is_some()
            }
            NodeQueryFilter::Older { date } => date > node.date,
            NodeQueryFilter::Newer { date } => date < node.date,
            NodeQueryFilter::Author { author } => {
                let matcher = SkimMatcherV2::default();
                matcher.fuzzy_match(&node.author, &author).is_some()
            }
            NodeQueryFilter::Environment { env } => env.merge(&node.version.env).is_ok(),
        }
    }
}

impl ProjectQueryFilter {
    pub fn is_ok(self, project: &Project) -> bool {
        match self {
            ProjectQueryFilter::Name { name } => {
                let matcher = SkimMatcherV2::default();
                matcher.fuzzy_match(&project.name, &name).is_some()
            }
            ProjectQueryFilter::Node { node } => {
                let matcher = SkimMatcherV2::default();
                matcher.fuzzy_match(&project.node, &node).is_some()
            }
            ProjectQueryFilter::Older { date } => date > project.date,
            ProjectQueryFilter::Newer { date } => date < project.date,
            ProjectQueryFilter::Author { author } => {
                let matcher = SkimMatcherV2::default();
                matcher.fuzzy_match(&project.author, &author).is_some()
            }
        }
    }
}
