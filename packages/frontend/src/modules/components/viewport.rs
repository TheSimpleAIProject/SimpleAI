// %%% components / viewport.rs %%%

use simple_ai_backend::modules::utils::{node::StrongNode, prelude::NodeContainer};

use crate::modules::pages::utils::NODE_TRANSFERER;

// %% includes %%
use super::utils::*;

// %% main %%
#[derive(Clone)]
pub struct ViewportNodeContainer {
    pub backend_node_container: NodeContainer,
    pub frontend_node_container: Signal<Vec<super::node::InternNode>>,
}
impl ViewportNodeContainer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_context(&mut self, node_ctx: StrongNode) {
        self.backend_node_container.push_context(node_ctx.clone());
        self.frontend_node_container
            .push(super::node::InternNode::from(node_ctx));
    }
}
impl Default for ViewportNodeContainer {
    fn default() -> Self {
        Self {
            backend_node_container: NodeContainer::new(),
            frontend_node_container: use_signal(Vec::<super::node::InternNode>::new),
        }
    }
}

#[component]
pub fn Viewport(
    #[props(default)] node_container: Signal<ViewportNodeContainer>,
    #[props(default)] mounted_data: Signal<Option<std::rc::Rc<MountedData>>>,
) -> Element {
    // ------------------------------ VARIABLES ------------------------------ //
    let fnc = node_container().frontend_node_container.cloned();
    let rendered_nodes = fnc.iter().map(|intern| {
        rsx! {
            super::node::Node { intern: intern.clone() }
        }
    });

    let get_client_rect = move || async move {
        if let Some(data) = mounted_data() {
            if let Ok(rect) = data.get_client_rect().await {
                dioxus::logger::tracing::debug!("RECT");
                return rect;
            }
        }
        PixelsRect::default()
    };

    let mut cursor_start_coords = use_signal(Vector2D::<f64, _>::zero);
    let mut element_start_coords = use_signal(Vector2D::<f64, _>::zero);

    let mut position = use_signal(Vector2D::<f64, _>::zero);
    let mut scale = use_signal(|| 1f64);

    let mut pressed = use_signal(|| false);
    let mut pressed_node = use_signal(|| Option::<super::node::InternNode>::None);
    let mut pressed_connection = use_signal(|| Option::<super::connection::InternConnection>::None);

    let mut cursor = use_signal(String::new);

    let get_diff = move |e: &MouseEvent| -> Vector2D<f64, _> {
        e.page_coordinates().to_vector() - cursor_start_coords()
    };

    let get_coords =
        move |e: &MouseEvent| -> Vector2D<f64, _> { element_start_coords() + get_diff(e) };

    let get_node_coords = move |e: &MouseEvent| -> Vector2D<f64, _> {
        {
            get_coords(e) / scale()
        }
    };

    let _ = use_resource(move || async move {
        let mut ctx = NODE_TRANSFERER();
        if let Some(mut node) = ctx.take() {
            dioxus::logger::tracing::debug!("NODE: {:?}", node.name);
            let position = (PageVector::from(node.position.unwrap_or_default())
                - get_client_rect().await.origin.to_vector().cast_unit())
                / *scale.peek();
            node.position = Some((position.x, position.y));
            node_container().push_context(StrongNode::from(node));
        }
    });

    // ------------------------------ EVENTS ------------------------------ //
    let dragover = move |e: DragEvent| {
        e.prevent_default();
    };

    let mousedown = move |e: MouseEvent| {
        for node in node_container().frontend_node_container.cloned().iter_mut() {
            for param in node.runtime_params.iter_mut() {
                if ((param.connection)().pressed)() {
                    cursor_start_coords.set(e.page_coordinates().to_vector());
                    pressed_connection.set(Some((param.connection)().clone()));
                    return;
                }
            }
            if (node.pressed)() {
                node.cursor.set("grabbing".into());
                cursor_start_coords.set(e.page_coordinates().to_vector());
                element_start_coords.set((node.position)() * scale());
                pressed_node.set(Some(node.clone()));
                return;
            }
        }
        if let Some(button) = e.trigger_button() {
            if button.into_web_code() == 1 {
                cursor_start_coords.set(e.page_coordinates().to_vector());
                element_start_coords.set(position());
                pressed.set(true);
            }
        }
    };

    let mousemove = move |e: MouseEvent| {
        if let Some(mut connection) = pressed_connection() {
            connection.dimensions.set(get_diff(&e));
        } else if let Some(mut node) = pressed_node() {
            node.cursor.set("grabbing".into());
            node.position.set(get_node_coords(&e));
        } else if pressed() {
            cursor.set("move".into());
            position.set(get_coords(&e));
        }
    };

    let mouseup = move |e: MouseEvent| {
        if let Some(mut connection) = pressed_connection() {
            connection.dimensions.set(get_diff(&e));
            connection.pressed.set(false);
            // if let Some(mut c) = CONNECTION() {
            //     c.foreign_dimensions.set((connection.dimensions)());
            // }
            pressed_connection.set(None);
        } else if let Some(mut node) = pressed_node() {
            node.cursor.set("grab".into());
            node.position.set(get_node_coords(&e));
            node.pressed.set(false);
            pressed_node.set(None);
        } else if pressed() {
            cursor.set("unset".into());
            position.set(get_coords(&e));
            pressed.set(false);
        }
        dioxus::logger::tracing::debug!("Mouseup");
    };

    let wheel = move |e: WheelEvent| {
        e.prevent_default();
        let sub;
        match e.data().delta() {
            WheelDelta::Pixels(v) => sub = v.y / 1E3,
            WheelDelta::Lines(v) => sub = v.y / 1E3,
            WheelDelta::Pages(v) => sub = v.y / 1E3,
        }

        scale.set({ scale - sub }.max(0.0).min(1.0));

        dioxus::logger::tracing::debug!("sub: {sub}, scale: {scale}");
    };

    let mounted = move |e: MountedEvent| async move {
        mounted_data.set(Some(e.data()));
    };

    rsx! {
        article {
            class: "Viewport",
            cursor: "{cursor}",
            overflow: "hidden",
            ondrop: drop,
            ondragover: dragover,
            onmousedown: mousedown,
            onmousemove: mousemove,
            onmouseup: mouseup,
            onwheel: wheel,
            main {
                position: "absolute",
                overflow: "visible",
                top: 0,
                left: 0,
                transform: "translate({position().x}px, {position().y}px) scale({scale()})",
                user_select: "none",
                onmounted: mounted,
                {rendered_nodes}
            }
        }
    }
}
