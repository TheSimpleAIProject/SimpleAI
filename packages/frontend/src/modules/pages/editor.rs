use super::utils::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use tokio::time::*;
use uuid::Uuid;

const CREATE_VIEWPORT: &str = r#"
    window.activeOnnxViewport = new window.Viewport(dioxus, document.getElementById("viewport"));
"#;

#[derive(Serialize, Deserialize, Debug)]
struct VNodeParameter {
    r#type: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct VNode {
    x: f32,
    y: f32,
    label: String,
    params: Vec<VNodeParameter>,
}

#[derive(Serialize, Deserialize, Debug)]
struct VConnection {
    from_node: VNode,
    from_output: VNodeParameter,
    to_node: VNode,
    to_input: VNodeParameter,
}

#[derive(Serialize, Deserialize, Debug)]
struct ViewportSave {
    nodes: Vec<VNode>,
    connections: Vec<VConnection>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AddNodeData {
    id: Uuid,
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct SaveNodeData {
    viewport_save: ViewportSave,
}

/// # handy for json representation:
/// let dbg: String = ViewportEvent::AddNode(Uuid::default()).into();
/// debug!("{}", dbg);
///
/// ``` { AddNode: ".." } ```
#[derive(Serialize, Deserialize, Debug)]
enum ViewportEvent {
    AddNode(AddNodeData),
    Save(SaveNodeData),
}

impl ViewportEvent {
    pub fn exec(self) {
        match self {
            Self::AddNode(AddNodeData { id, x, y }) => {
                // TODO: let node = simple_ai_backend::modules::nodes::query_onnx();

                // then remove this
                let params = vec![
                    VNodeParameter {
                        r#type: "input".into(),
                        name: "in1".into(),
                    },
                    VNodeParameter {
                        r#type: "output".into(),
                        name: "out1".into(),
                    },
                ];
                let node = VNode {
                    x,
                    y,
                    label: "sample".into(),
                    params,
                };

                let json_node = serde_json::to_string(&node).unwrap();

                document::eval(&format!(
                    r#"window.activeOnnxViewport.handleAddNode({})"#,
                    json_node
                ));
            }
            Self::Save(SaveNodeData { viewport_save }) => {
                // TODO: simple_ai_backend::modules::porject::save_onnx(viewport_save);
                debug!("Saving...");
            }
        }
    }
}

impl From<JsonValue> for ViewportEvent {
    fn from(value: JsonValue) -> Self {
        serde_json::from_value(value).expect("Could not convert ViewportEvent from json")
    }
}

impl Into<String> for ViewportEvent {
    fn into(self) -> String {
        serde_json::to_string(&self).expect("Could not convert ViewportEvent to json.")
    }
}

impl From<String> for ViewportEvent {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).expect("Could not convert ViewportEvent from json string.")
    }
}

// TODO:
//  - add a train button
//     -> add something like simple_ai_backend :: onnx :: Project . train()
//  - BUT FIRST: add a save button and
//     -> js getNodesFunction
//     -> convert to Node of backend (serde json)
//     -> add something like simple_ai_backend :: onnx :: Project . save_nodes(Vec<Nodes>)
#[page]
pub fn Editor(children: Element) -> Element {
    rsx! {
        main {
            onmounted: move |e| async move {
                sleep(Duration::from_millis(100)).await;

                let mut handle = document::eval(CREATE_VIEWPORT);

                loop {
                    let s: JsonValue = handle.recv().await.expect("error recieving string");
                    let e: ViewportEvent = s.into();
                    e.exec();
                }

                // loop {
                //     TODO: also convert the string to a rust new Event enum
                //      - the event types of the enum are AddNode and CheckConnection
                //      - the backend will need something like simple_ai_backend :: onnx ::
                //      check_connection (param1, param2)
                //

                //     TODO: If something failes make a notification.
                // debug!("{:?}", viewport_listener.recv::< String > (). await);
                // ViewportEvent::from(viewport_listener.recv::<String>().await.unwrap())
                //     .exec();
                // }
            },
            section { id: "viewport" }
            Divider { id: "editor-aside-viewport", orientation: 'v' }
            aside { Search {} }
            document::Script { src: asset!("/assets/scripts/onnx-viewport.js") }
        }
    }
}
