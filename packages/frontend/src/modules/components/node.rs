// %%% components / node.rs %%%

use simple_ai_backend::modules::utils::node::StrongNode;
use simple_ai_backend::modules::utils::param::ParamKind;

// %% includes %%
use super::runtime_param::{InternRuntimeParam, RuntimeParam};
use super::static_param::InternStaticParam;
use super::utils::*;

// %% main %%
// % transferrer % //
pub static NODE_TRANSFERER: GlobalSignal<Option<simple_ai_backend::modules::utils::node::Node>> =
    GlobalSignal::new(|| None);

// % Node % //
#[derive(PartialEq, Props, Clone)]
pub struct InternNode {
    pub node: StrongNode,
    #[props(default = Signal::default())]
    pub runtime_params: Signal<Vec<InternRuntimeParam>>,
    #[props(default = Signal::default())]
    pub static_params: Signal<Vec<InternStaticParam>>,
    #[props(default = Signal::default())]
    pub pressed: Signal<bool>,
    #[props(default = Signal::default())]
    pub position: Signal<Vector2D<f64, PageSpace>>,
    #[props(default = Signal::default())]
    pub cursor: Signal<String>,
}

impl From<StrongNode> for InternNode {
    fn from(node_ctx: StrongNode) -> Self {
        let b = Self::builder()
            .node(node_ctx.clone())
            .position(Signal::new(PageVector::from(
                node_ctx
                    .context
                    .try_lock()
                    .unwrap()
                    .position
                    .unwrap_or_default(),
            )));
        let node = node_ctx.context.try_lock().unwrap();
        let mut runtime_params = Vec::<InternRuntimeParam>::new();
        let mut static_params = Vec::<InternStaticParam>::new();
        node.get_params().iter().for_each(move |param_ctx| {
            let param = param_ctx.context.try_lock().unwrap();
            match param.kind {
                ParamKind::Runtime { .. } => {
                    runtime_params.push(InternRuntimeParam::from(param_ctx.clone()));
                }
                ParamKind::Static { .. } => {
                    static_params.push(InternStaticParam::from(param_ctx.clone()));
                }
            }
        });
        dioxus::logger::tracing::debug!("creating");
        b.build()
    }
}

#[component]
pub fn Node(intern: InternNode) -> Element {
    let mousedown = move |_| {
        intern.pressed.set(true);
    };

    let mounted = move |e: MountedEvent| async move {
        if let Ok(client_rect) = e.data().get_client_rect().await {
            intern.position -= PageVector::new(client_rect.width(), client_rect.width()) / 2f64;
        }
    };

    let mut node_name = use_signal(String::new);

    use_future(move || {
        let context = intern.node.context.clone();
        async move {
            let node = context.lock().await;
            node_name.set(node.name.clone());
        }
    });

    let rendered_params = intern.runtime_params.iter().map(|intern| {
        rsx! {
            RuntimeParam { intern: intern.clone() }
        }
    });

    rsx! {
        body {
            class: "Node",
            position: "absolute",
            top: 0,
            left: 0,
            transform: "translate({(intern.position)().x}px, {(intern.position)().y}px) scale(100%)",
            z_index: 1,
            onmounted: mounted,
            header {
                cursor: "{intern.cursor}",
                user_select: "none",
                onmousedown: mousedown,
                onmouseover: move |_| { intern.cursor.set("grab".into()) },
                h1 { {} }
            }
            main {
                display: "flex",
                flex_direction: "column",
                justify_content: "space-evenly",
                align_items: "center",
                {rendered_params}
            }
            footer {
            }
        }
    }
}
