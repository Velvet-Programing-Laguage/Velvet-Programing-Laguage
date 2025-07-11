use crate::types::VelvetValue;
use crate::error::{VelvetError, ErrorKind};
use std::collections::HashMap;
use tokio::task::JoinHandle;

pub struct ModuleRegistry {
    modules: HashMap<String, Box<dyn Fn(&str) -> String + Send + Sync>>,
    async_modules: HashMap<String, Box<dyn Fn(&str) -> JoinHandle<Result<String, VelvetError>> + Send + Sync>>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        let mut reg = ModuleRegistry {
            modules: HashMap::new(),
            async_modules: HashMap::new(),
        };
        reg.register_builtin_modules();
        reg
    }

    pub fn register_builtin_modules(&mut self) {
        self.modules.insert("python_requests".to_string(), Box::new(modules::python::requests));
        self.modules.insert("cpp_boost".to_string(), Box::new(modules::cpp::boost));
        self.modules.insert("csharp_json".to_string(), Box::new(modules::csharp::json));
        self.modules.insert("ruby_httparty".to_string(), Box::new(modules::ruby::httparty));
        self.modules.insert("js_axios".to_string(), Box::new(modules::javascript::axios));
        self.modules.insert("rust_flate2".to_string(), Box::new(modules::rust::flate2));
        self.modules.insert("java_jython".to_string(), Box::new(modules::java::jython));
        self.modules.insert("tauri_gui".to_string(), Box::new(modules::tauri::gui));
        self.modules.insert("wayland_gui".to_string(), Box::new(modules::wayland::gui));
        self.modules.insert("ai_tensorflow".to_string(), Box::new(modules::ai::tensorflow));
        self.modules.insert("ai_pytorch".to_string(), Box::new(modules::ai::pytorch));
        self.modules.insert("perf_parallel".to_string(), Box::new(modules::perf::parallel));
        self.modules.insert("perf_crypto".to_string(), Box::new(modules::perf::crypto));
        self.modules.insert("gpu_cuda".to_string(), Box::new(modules::gpu::cuda));
        self.async_modules.insert("db_sqlite".to_string(), Box::new(|args| {
            tokio::spawn(modules::db::sqlite(args.to_string()))
        }));
        self.async_modules.insert("net_websocket".to_string(), Box::new(|args| {
            tokio::spawn(modules::net::websocket(args.to_string()))
        }));
    }

    pub fn execute(&self, module: &str, args: &str) -> Option<String> {
        self.modules.get(module).map(|f| f(args))
    }

    pub async fn execute_async(module: &str, args: &str) -> Result<String, VelvetError> {
        let registry = MODULE_REGISTRY.lock().unwrap();
        if let Some(f) = registry.async_modules.get(module) {
            let handle = f(args);
            match handle.await {
                Ok(res) => res,
                Err(e) => Err(VelvetError::new(
                    ErrorKind::AsyncError,
                    &format!("Async execution failed: {}", e)
                )),
            }
        } else {
            Err(VelvetError::new(
                ErrorKind::ModuleError,
                &format!("Module {} not found", module)
            ))
        }
    }
}

lazy_static::lazy_static! {
    static ref MODULE_REGISTRY: std::sync::Mutex<ModuleRegistry> = std::sync::Mutex::new(ModuleRegistry::new());
}
