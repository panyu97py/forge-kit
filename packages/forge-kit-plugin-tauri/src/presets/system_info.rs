use crate::plugin::{Plugin, PluginContext};
use std::error::Error;

pub struct SystemInfo {}

impl Plugin for SystemInfo {
    fn name(&self) -> &str {
        "forge_kit_plugin_system_info"
    }

    fn register(&self, context: Box<dyn PluginContext>) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn unregister(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
