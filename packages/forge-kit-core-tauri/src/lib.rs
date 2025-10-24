// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use forge_kit_plugin_tauri::kernel::{Kernel, PluginRegistry};

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

            // 注册插件
            let plugin_registry: &mut dyn PluginRegistry = &mut forge_kit_kernel;
            plugin_registry.load_plugin("name", "path").unwrap();
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
