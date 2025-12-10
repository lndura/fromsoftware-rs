use eldenring::fd4::{FD4PadManager, CSPad};
use hudhook::imgui::{TreeNodeFlags, Ui};

use super::DebugDisplay;

const INPUT_LOW: i32 = 0;
const INPUT_HIGH: i32 = 100_000;

impl DebugDisplay for FD4PadManager {
    fn render_debug(&self, ui: &&mut Ui) {
        if let Some(cs_pad) = self.get_cs_pad() {
            if ui.collapsing_header("CSPad", TreeNodeFlags::empty()) {
                ui.indent();
                cs_pad.render_debug(ui);
            }
        } else {
            ui.text("CSPad not found");
        };
    }
}

impl DebugDisplay for CSPad {
    fn render_debug(&self, ui: &&mut Ui) {
        let pointer = self as *const CSPad;
        let mut pointer_string = format!("{pointer:#x?}");
        ui.input_text("CSPad instance", &mut pointer_string)
            .read_only(true)
            .build();

        let low = INPUT_LOW;
        let high = INPUT_HIGH;

        ui.columns(2, "CSPadColumnsStart", false);
        if ui.collapsing_header("Action", TreeNodeFlags::empty()){
            for input in low..high {
                if self.poll_action_input(input) {
                    ui.text(format!("Input {}: Pressed", input));
                }
            }
        }
        ui.next_column();
        if ui.collapsing_header("Movement", TreeNodeFlags::empty()){
            for input1 in (low..high).step_by(2) {
                let input2 = input1 + 1;
                let (x, y) = self.poll_stick_input(input1, input2);
                if x != 0.0 {
                    ui.text(format!("Input {}: {}", input1, x));
                }
                if y != 0.0 {
                    ui.text(format!("Input {}: {}", input2, y));
                }
            }
        }

        ui.columns(1, "CSPadColumnsEnd", false);
    }
}