use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// -------------------- PROJECT -------------------- //
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub date: DateTime<Utc>,
    pub desc: String,
    pub author: String,
    pub node: String,
}
