// %%% components / search_result.rs %%%

use simple_ai_backend::modules::utils::node::Node;

// %% includes %%
use super::node::NODE_TRANSFERER;
use super::utils::*;

// %% main %%

// % Search Result % //
#[derive(PartialEq, Props, Clone, Copy)]
pub struct InternSearchResult {
    pub node: Signal<Node>,
}

impl From<Node> for InternSearchResult {
    fn from(node_context: Node) -> Self {
        Self::builder().node(Signal::new(node_context)).build()
    }
}

#[item]
pub fn SearchResult(intern: InternSearchResult) -> Element {
    let draggingend = move |v: PageVector| {
        let mut node = intern.node.cloned();
        node.position = Some((v.x, v.y));
        *NODE_TRANSFERER.write() = Some(node);
    };

    rsx! {
        article {
            class: "draggable-node",
            id: "{intern.node.cloned().author}/{intern.node.cloned().name}",
            h3 {
                span { id: "name", "{intern.node.cloned().name}" }
            }
            p { id: "description", "{intern.node.cloned().description}" }
        }
    }
}
