// %%% components / connection.rs %%%

use simple_ai_backend::modules::utils::param::RuntimeParamKind;

// %% includes %%
use super::utils::*;

// %% main %%
#[derive(PartialEq, Props, Clone)]
pub struct InternConnection {
    pub kind: RuntimeParamKind,
    #[props(default = Signal::default())]
    pub dimensions: Signal<Vector2D<f64, PageSpace>>,
    #[props(default = Signal::default())]
    pub foreign_dimensions: Signal<Vector2D<f64, PageSpace>>,
    #[props(default = Signal::default())]
    pub pressed: Signal<bool>,
}
impl From<RuntimeParamKind> for InternConnection {
    fn from(kind: RuntimeParamKind) -> Self {
        Self::builder().kind(kind).build()
    }
}

#[component]
pub fn Connection(intern: InternConnection) -> Element {
    let stroke_width = use_signal(|| 3);

    let mut left = use_signal(|| "unset");
    let mut right = use_signal(|| "unset");
    let mut class = use_signal(|| "");

    let mut div_rect = use_signal(PixelsRect::zero);

    let mut abs_dims = use_signal(Vector2D::<f64, PageSpace>::zero);

    use_effect(move || abs_dims.set((intern.dimensions)().abs()));

    let mut svg_pos = use_signal(Vector2D::<f64, PageSpace>::zero);

    use_effect(move || {
        let d = (intern.dimensions)();
        let mut p = Vector2D::zero();
        if d.x < 0f64 {
            p.x = d.x
        }
        if d.y < 0f64 {
            p.y = d.y
        }
        svg_pos.set(p);
        dioxus::logger::tracing::debug!("POS: {}, DIMS: {}", svg_pos().x, abs_dims().x);
    });

    let mut svg_path = use_signal(String::new);

    use_effect(move || {
        dioxus::logger::tracing::debug!("RECT: {}", div_rect().height());

        let start = svg_pos() * -1f64; // .add(Vector2D::splat(div_rect().height()) / 2f64);
        let end = (intern.dimensions)();
        let ls = Vector2D::<f64, PageSpace>::new(end.x, 0f64);
        let le = Vector2D::<f64, PageSpace>::new(0f64, end.y);

        svg_path.set(format!(
            "m {} {} c {} {}, {} {}, {} {}",
            start.x, start.y, ls.x, ls.y, le.x, le.y, end.x, end.y
        ));
    });

    let offset = "calc((var(--connection-diameter) + var(--node-border-width) / 2) / -2)";

    let kind = intern.kind.clone();
    use_effect(move || match kind {
        RuntimeParamKind::Input => {
            class.set("Input");
            left.set(offset);
        }
        RuntimeParamKind::Output => {
            class.set("Output");
            right.set(offset);
        }
    });

    let div_mousedown = move |e: MouseEvent| {
        intern.pressed.set(true);
    };

    let div_mouseup = move |e: MouseEvent| {
        // *CONNECTION.write() = Some(intern.clone());
    };

    let div_mounted = move |e: MountedEvent| async move {
        div_rect.set(
            e.data()
                .get_client_rect()
                .await
                .unwrap_or(PixelsRect::zero()),
        );
    };
    rsx! {
        body {
            class: "Connection",
            display: "block",
            height: "fit-content",
            width: "fit-content",
            z_index: 0,
            position: "fixed",
            left: "{left}",
            right: "{right}",
            div {
                class: "Wrapper",
                width: "{abs_dims().x}px",
                height: "{abs_dims().y}px",
                position: "fixed",
                transform: "translate({svg_pos().x}px, {svg_pos().y}px)",
                display: "block",
                z_index: 1,
                svg {
                    class: "Draw",
                    height: "100%",
                    width: "100%",
                    view_box: "0 0 {abs_dims().x} {abs_dims().y}",
                    path {
                        class: "Curve",
                        stroke_width: "{stroke_width()}px",
                        d: "{svg_path()}",
                    }
                }
            }
            div {
                class: "{class}",
                position: "relative",
                z_index: 2,
                onmousedown: div_mousedown,
                onmouseup: div_mouseup,
                onmounted: div_mounted,
            }
        }
    }
}
