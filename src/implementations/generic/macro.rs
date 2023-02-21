use std::{rc::Rc};




use crate::events::CausedBy;
use crate::macro_executor::{self, MainWorkerGenerator};

use super::bridge::procedure_call::{
    emit_console_out, emit_result, on_procedure, proc_bridge_ready, ProcedureBridge,
};
use super::GenericInstance;

pub struct GenericMainWorkerGenerator {
    bridge: ProcedureBridge,
    instance: GenericInstance,
}

impl GenericMainWorkerGenerator {
    pub fn new(bridge: ProcedureBridge, instance: GenericInstance) -> Self {
        Self { bridge, instance }
    }
}

impl MainWorkerGenerator for GenericMainWorkerGenerator {
    fn generate(&self, args: Vec<String>, _caused_by : CausedBy) -> deno_runtime::worker::MainWorker {
        let bootstrap_options = deno_runtime::BootstrapOptions {
            args,
            ..Default::default()
        };

        let mut worker_options = deno_runtime::worker::WorkerOptions {
            bootstrap: bootstrap_options,
            ..Default::default()
        };

        let ext = deno_core::Extension::builder()
            .ops(vec![
                on_procedure::decl(),
                emit_result::decl(),
                proc_bridge_ready::decl(),
                emit_console_out::decl(),
            ])
            .state({
                let brige = self.bridge.clone();
                let instance = self.instance.clone();
                move |state| {
                    state.put(brige.clone());
                    state.put(instance.clone());
                    Ok(())
                }
            })
            .build();
        worker_options.extensions.push(ext);
        worker_options.module_loader = Rc::new(macro_executor::TypescriptModuleLoader::default());

        let main_module = deno_core::resolve_path(".").expect("Failed to resolve path");
        // todo(CheatCod3) : limit the permissions
        let permissions = deno_runtime::permissions::Permissions::allow_all();
        deno_runtime::worker::MainWorker::bootstrap_from_options(
            main_module,
            permissions,
            worker_options,
        )
    }
}