// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use forge_kit_plugin_tauri::kernel::{Kernel, PluginRegistry};
use forge_kit_plugin_tauri::plugin::PluginContext;
use forge_kit_plugin_tauri::presets::system_info::{SystemInfo};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // 初始化内核
            let mut forge_kit_kernel = Kernel::new(app.handle().clone());
            // 注册系统信息插件
            forge_kit_kernel.register_plugin(Box::new(SystemInfo{})).expect("加载 SystemInfo 插件失败");

            forge_kit_kernel.invoke_method("get_system_info", None).expect("get_system_info fail");
            // 成功
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
