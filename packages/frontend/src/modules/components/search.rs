// %%% components / search.rs %%%

// %% includes %%
use super::search_result::{InternSearchResult, SearchResult};
use super::utils::*;
use simple_ai_backend::modules::nodes::query::query_nodes;
use simple_ai_backend::modules::utils::prelude::NodeQueryFilter;
use tokio::time::*;

// %% main %%
#[item]
pub fn Search(#[props(extends = GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    let mut intern_search_results = use_signal(Vec::<InternSearchResult>::new);
    let mut search_results = use_signal(|| query_nodes(vec![]));

    let input = move |e: FormEvent| {
        search_results.set(query_nodes(vec![NodeQueryFilter::Name { name: e.value() }]));
        intern_search_results.clear();
    };

    use_effect(move || {
        intern_search_results.set(
            search_results()
                .iter()
                .map(|result| InternSearchResult::from(result.context.try_lock().unwrap().clone()))
                .collect::<Vec<InternSearchResult>>(),
        );
    });

    let future = use_resource(move || async move {
        // You can create as many eval instances as you want
        let mut eval = document::eval(r#"console.log("HELLOO")"#);
        // let mut eval = document::eval(
        //     r#"console.log("helloworld"); const di = new window.OnnxDragIn(document.getElementById("onnx-drag-in"), document.getElementById("search"), document.getElementById("nodeContainer")); console.log(di);"#,
        // );
    });

    rsx! {
        article {
            class: "Search",
            id: "onnx-drag-in",
            onmounted: move |_| async move {
                sleep(Duration::from_millis(100)).await;
                _ = document::eval(
                    r#"const di = new window.OnnxDragIn(document.getElementById("onnx-drag-in"), document.getElementById("search"), document.getElementById("nodeContainer"));"#,
                );
            },
            ..attributes,
            document::Script { src: asset!("/assets/scripts/onnx-drag-in.js"), defer: true }
            header {
                input {
                    oninput: input,
                    r#type: "search",
                    placeholder: "search",
                    id: "search",
                }
                        // nav {
            // 	button { "your nodes" }
            // 	button { "installed nodes" }
            // 	button { "profiles" }
            // }
            }
            main { id: "nodeContainer",
                for intern in intern_search_results() {
                    SearchResult { intern }
                }
            }
        }
    }
}
