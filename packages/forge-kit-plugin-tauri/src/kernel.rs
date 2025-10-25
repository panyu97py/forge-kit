use crate::plugin::{AnyFunc, Plugin, PluginContext};
use libloading::Library;
use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use tauri::AppHandle;

pub struct Kernel {
    // tauri app 实例
    app: AppHandle,
    // 已注册的方法映射
    methods: HashMap<String, AnyFunc>,
    // 已加载的插件映射
    plugins: HashMap<String, Box<dyn Plugin>>,
    // 引用的动态库数组
    _libs: Vec<Library>,
}

pub trait PluginRegistry {
    // 加载插件
    fn load_plugin(&mut self, path: Option<&str>) -> Result<(), Box<dyn Error>>;

    // 注册插件
    fn register_plugin(&mut self, plugin:Box<dyn Plugin>) -> Result<(), Box<dyn Error>>;
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
    fn load_plugin(&mut self, path: Option<&str>) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn register_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<(), Box<dyn Error>> {
        plugin.register(self as &mut dyn PluginContext)?;
        Ok(())
    }
}

impl PluginContext for Kernel {
    fn get_tauri_app(&self) -> Box<AppHandle> {
        todo!()
    }

    fn register_method(&mut self, name: &str, method: AnyFunc) -> Result<(), Box<dyn Error>> {
        self.methods.insert(name.to_string(), method);
        Ok(())
    }

    fn invoke_method(&self, name: &str, args: Option<&[Box<dyn Any>]>) -> Result<(), Box<dyn Error>>  {
        let method = self.methods.get(name).unwrap();
        let _result = method(args.unwrap_or(&[]));
        Ok(())
    }
}
