// %%% components / runtime_param.rs %%%

use simple_ai_backend::modules::utils::param::{ParamKind, RuntimeParamKind, StrongParam};

// %% includes %%
use super::connection::{Connection, InternConnection};
use super::utils::*;

// %% main %%
#[derive(PartialEq, Props, Clone)]
pub struct InternRuntimeParam {
    pub param: StrongParam,
    pub connection: Signal<InternConnection>,
}
impl From<StrongParam> for InternRuntimeParam {
    fn from(param_ctx: StrongParam) -> Self {
        let param = param_ctx.context.blocking_lock();
        let mut runtime_param_kind = RuntimeParamKind::Input; // This will not happen just so that no error
                                                              // is thrown
        if let ParamKind::Runtime { kind, .. } = &param.kind {
            runtime_param_kind = kind.clone()
        }
        Self::builder()
            .param(param_ctx.clone())
            .connection(Signal::new(InternConnection::from(
                runtime_param_kind.clone(),
            )))
            .build()
    }
}

#[component]
pub fn RuntimeParam(intern: InternRuntimeParam) -> Element {
    rsx! {
        body { class: "Param",
            Connection { intern: (intern.connection)() }
        }
    }
}
