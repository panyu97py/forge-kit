use crate::plugin::{AnyFunc, Plugin, PluginContext};
use libloading::Library;
use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use tauri::App;

trait PluginRegistry {
    // 加载插件
    fn load_plugin(&mut self, path: &str) -> Result<(), Box<dyn Error>>;
}

struct Kernel {
    // 已注册的方法映射
    methods: HashMap<String, Box<AnyFunc>>,
    // 已加载的插件映射
    plugins: HashMap<String, Box<dyn Plugin>>,
    // 引用的动态库数组
    _libs: Vec<Library>,
}

impl PluginRegistry for Kernel {
    fn load_plugin(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl PluginContext for Kernel {
    fn get_tauri_app(&self) -> Box<App> {
        todo!()
    }

    fn register_method(&self, name: &str, method: &AnyFunc) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn invoke_method(&self, name: &str, args: &[Box<dyn Any>]) -> Box<dyn Any> {
        todo!()
    }
}
