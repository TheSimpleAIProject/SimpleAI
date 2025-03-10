use crate::utils::prelude::*;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

// ---------------- QUERY FILTER ---------------- //
#[derive(Clone)]
pub enum QueryFilter {
    Older { date: Date },
    Newer { date: Date },
    Author { author: String },
    Environment { env: Environment },
    Name { name: String },
}

impl QueryFilter {
    pub fn is_ok(self, node: Node) -> bool {
        match self {
            QueryFilter::Name { name } => {
                let matcher = SkimMatcherV2::default();
                matcher.fuzzy_match(&node.name, &name).is_some()
            }
            QueryFilter::Older { date } => date > node.date,
            QueryFilter::Newer { date } => date < node.date,
            QueryFilter::Author { author } => {
                let matcher = SkimMatcherV2::default();
                matcher.fuzzy_match(&node.author, &author).is_some()
            }
            QueryFilter::Environment { env } => env.merge(&node.version.env).is_ok(),
        }
    }
}
