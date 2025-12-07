use super::utils::*;

#[derive(Formifiable)]
pub struct Project {
    pub name: String,
    pub description: String,
}

#[page]
pub fn Edit() -> Element {
    let mut proj = Project {
        name: "bundlenode".into(),
        description: "hello this is bundlenode".into(),
    };
    rsx! {
        main {
            {proj.rsx_edit_form(|e| { debug!("editing {:?}", e) })}
                // form {
        //     LabeledBox {
        //         name: "name",
        //         kind: "text",
        //         required: true,
        //         placeholder: "SampleProject",
        //     }
        //     LabeledBox {
        //         name: "description",
        //         kind: "text",
        //         required: false,
        //         placeholder: "this is a description",
        //     }
        //     section { class: "button-wrapper",
        //         button { r#type: "submit", NewIcon {} }
        //     }
        // }
        }
    }
}
