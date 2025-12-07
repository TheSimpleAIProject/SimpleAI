use simple_ai_backend::modules::projects::query::query_projects;
use simple_ai_backend::modules::utils::prelude::ProjectQueryFilter;

use super::super::components::project::Project;
use super::utils::*;

#[page]
pub fn Projects() -> Element {
    let mut search_results = use_signal(|| query_projects(vec![]));
    let input = move |e: FormEvent| {
        search_results.set(query_projects(vec![ProjectQueryFilter::Name {
            name: e.value(),
        }]));
    };

    rsx! {
        main {
            input {
                oninput: input,
                r#type: "search",
                placeholder: "search",
                id: "search",
            }
            article { class: "projects-wrapper",
                div { class: "projects-view",
                    for res in search_results() {
                        Project { name: res.name, date: res.date, desc: res.date }
                    }
                }
            }
        }
    }
}
