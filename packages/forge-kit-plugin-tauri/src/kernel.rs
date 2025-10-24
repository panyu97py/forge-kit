use crate::plugin::{AnyFunc, Plugin, PluginContext};
use libloading::Library;
use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use tauri::AppHandle;

pub trait PluginRegistry {
    // 加载插件
    fn load_plugin(&mut self, name: &str, path: &str) -> Result<(), Box<dyn Error>>;
}

pub struct Kernel {
    // tauri app 实例
    app: AppHandle,
    // 已注册的方法映射
    methods: HashMap<String, Box<AnyFunc>>,
    // 已加载的插件映射
    plugins: HashMap<String, Box<dyn Plugin>>,
    // 引用的动态库数组
    _libs: Vec<Library>,
}

impl Kernel {
    pub fn new(app: AppHandle) -> Self {
        let methods = HashMap::new();
        let plugins = HashMap::new();
        let _libs = Vec::new();
        return Self { app, methods, plugins, _libs };
    }
}

impl PluginRegistry for Kernel {
    fn load_plugin(&mut self, name: &str, path: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl PluginContext for Kernel {
    fn get_tauri_app(&self) -> Box<AppHandle> {
        todo!()
    }

    fn register_method(&self, name: &str, method: &AnyFunc) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn invoke_method(&self, name: &str, args: &[Box<dyn Any>]) -> Box<dyn Any> {
        todo!()
    }
}
