use std::any::Any;
use std::collections::HashMap;
use crate::plugin::{Plugin, PluginContext};
use std::error::Error;
use std::sync::Arc;

#[derive(Clone)]
pub struct SystemInfo;

impl SystemInfo {
    fn get_system_info(&self)  {
        println!("SystemInfo");
    }
}

impl Plugin for SystemInfo {
    fn name(&self) -> &str { "forge_kit_plugin_system_info" }

    fn register(&self, context: &mut dyn PluginContext) -> Result<(), Box<dyn Error>> {
        // 手动注册方法
        let instance = Arc::new(self.clone()); // 需要 Plugin 实现 Clone
        context.register_method(
            "get_system_info",
            Box::new(move |_args| {
                instance.get_system_info();
                Box::new(())
            }),
        )?;
        Ok(())
    }

    fn unregister(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
