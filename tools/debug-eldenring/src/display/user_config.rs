use debug::UiExt;
use eldenring::cs::UserConfig;

use crate::display::DebugDisplay;

impl DebugDisplay for UserConfig {
    fn render_debug(&self,ui: &hudhook::imgui::Ui) {
        ui.header("DebugPropertyMap", || {
            ui.text(format!("Items: {}", self.debug_property_map.len()));
            for pair in self.debug_property_map.iter() {
                ui.text(format!(
                    "Key: {}\nValue: {}",
                    pair.first,
                    pair.second                   
                ));
            }
        });
    }
}