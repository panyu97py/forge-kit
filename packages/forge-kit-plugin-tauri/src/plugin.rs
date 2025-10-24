use std::any::Any;
use std::error::Error;
use tauri::AppHandle;
pub type AnyFunc = Box<dyn Fn(&[Box<dyn Any>]) -> Box<dyn Any> + Send + Sync>;
pub trait Plugin {
    // 插件名称
    fn name(&self) -> &str;

    // 注册插件
    fn register(&self, context: Box<dyn PluginContext>) -> Result<(), Box<dyn Error>>;

    // 卸载插件
    fn unregister(&self) -> Result<(), Box<dyn Error>>;
}

pub trait PluginContext {

    fn get_tauri_app(&self) -> Box<AppHandle>;

    // 注册方法
    fn register_method(&self, name: &str, method: &AnyFunc) -> Result<(), Box<dyn Error>>;

    // 调用方法
    fn invoke_method(&self, name: &str, args: &[Box<dyn Any>]) -> Box<dyn Any>;
}
