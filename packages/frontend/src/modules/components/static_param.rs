// %%% components / static_param.rs %%%

use simple_ai_backend::modules::utils::param::StrongParam;

// %% includes %%
use super::utils::*;

// %% main %%
#[derive(PartialEq, Props, Clone)]
pub struct InternStaticParam {
    pub param: StrongParam,
}
impl From<StrongParam> for InternStaticParam {
    fn from(param: StrongParam) -> Self {
        let b = Self::builder();
        b.param(param).build()
    }
}

#[component]
pub fn StaticParam(style: String, intern: InternStaticParam) -> Element {
    rsx! {
        style { {style} }
        body { class: "Param" }
    }
}
