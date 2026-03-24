use debug::UiExt;
use eldenring::{
    Tree,
    cs::{CSKeyAssign, CSPad},
    dluid::{
        DLUserInputDeviceImpl, DLVirtualAnalogKeyInfo, DLVirtualInputData, DynamicBitset, KeyboardDevice, MouseDevice, MultiDevices, MultiDevices_0x78, PadDevice, VirtualMultiDevice
    },
    dlut::DLFixedVector,
    fd4::{FD4PadManager, InputType, InputTypeGroup, PadEntry, WindowCursorContext},
};
use fromsoftware_shared::{Program, vftable_classname};
use hudhook::imgui::{TableColumnSetup, Ui};

use crate::display::DebugDisplay;

fn render_fixed_vector<F: Fn(&T), T, const C: usize, S: AsRef<str>>(
    ui: &Ui,
    label: S,
    list: &DLFixedVector<T, C>,
    f: F,
) {
    ui.header(label, || {
        ui.text(format!(
            "{} out of {} entries are being used.",
            list.len(),
            C
        ));
        for (index, item) in list.iter().enumerate() {
            ui.header(format!("Entry {}", index), || f(item));
        }
    });
}

fn render_tree<F: Fn(&T), T, S: AsRef<str>>(ui: &Ui, label: S, tree: &Tree<T>, f: F) {
    ui.header(label, || {
        if !tree.is_empty() {
            ui.text(format!("This tree has {} nodes", tree.len()));
            for (index, node) in tree.iter().enumerate() {
                ui.header(format!("Entry {}", index), || f(node));
            }
        } else {
            ui.text("This tree is empty.");
        }
    });
}

impl DebugDisplay for FD4PadManager {
    fn render_debug(&self, ui: &Ui) {
        render_fixed_vector(ui, "MultiDevices list", &self.multi_device_list, |entry| {
            let multi_device = unsafe { entry.as_ref() };
            multi_device.render_debug(ui);
        });

        render_fixed_vector(ui, "Pad list", &self.pad_list, |entry| {
            let pad_entry_tree = unsafe { entry.as_ref() };
            render_tree(ui, "Pad Tree", pad_entry_tree, |pair| {
                ui.text(format!("{:#X}", pair.key));
                pair.value.render_debug(ui);
            });
        });

        render_fixed_vector(ui, "KeyAssign list", &self.key_assign_list, |entry| {
            let key_assign_tree = unsafe { entry.as_ref() };
            render_tree(ui, "Pad Tree", key_assign_tree, |pair| {
                ui.text(format!("{:#X}", pair.key));
                ui.header("CSKeyAssign", || {
                    let key_assign = unsafe { pair.value.as_ref() };
                    key_assign.render_debug(ui);
                });
            });
        });

        ui.text(format!(
            "exit_foreground_signaled: {}",
            self.exit_foreground_signaled
        ));
        ui.text(format!(
            "is_back_ground_window: {}",
            self.is_back_ground_window
        ));

        ui.separator();

        ui.header("CSInGamePad", || {
            if let Some(game_pad) = self.get_in_game_pad() {
                game_pad.render_debug(ui);

                ui.header("UserInputKey states", || {
                    let input_type_group = unsafe { game_pad.input_type_group.as_ref() };
                    ui.table(
                        "UserInputKey",
                        [
                            TableColumnSetup::new("UserInputKey"),
                            TableColumnSetup::new("State"),
                        ],
                        input_type_group.iter(),
                        |ui, _, pair| {
                            ui.table_next_column();
                            ui.text(format!("{}", pair.key));
                            ui.table_next_column();
                            ui.text(format!("{}", game_pad.poll_digital_input(pair.key)));
                        },
                    );
                });
            } else {
                ui.text("Failed to get CSInGamePad");
            }
        });
    }
}

impl DebugDisplay for PadEntry {
    fn render_debug(&self, ui: &Ui) {
        ui.header("CSPad", || {
            let pad = unsafe { self.entry.as_ref() };
            pad.render_debug(ui);
        });
        ui.text(format!("enable_use: {}", self.enable_use));
    }
}

impl DebugDisplay for CSPad {
    fn render_debug(&self, ui: &Ui) {
        let program = &Program::current();
        let vftable = self.vftable;
        if let Some(class_name) = vftable_classname(program, vftable) {
            ui.text(format!("addr: {:#X}\nname: {}", vftable, class_name));
        } else {
            ui.text("CSPad instance rtti couldn't be read.");
        }

        ui.header("multi_devices", || {
            let multi_device = unsafe { self.multi_devices.as_ref() };
            multi_device.render_debug(ui);
        });

        let pad_name = unsafe {
            let data = self.pad_name;
            if !data.is_null() {
                let mut len = 0;
                while *data.add(len) != 0 {
                    len += 1;
                }

                let slice = std::slice::from_raw_parts(data, len);
                String::from_utf16_lossy(slice)
            } else {
                String::new()
            }
        };

        ui.text(format!(
            "pad_name: {}\nallow_polling: {}",
            pad_name, self.allow_polling
        ));

        ui.header("key_assign", || {
            let key_assign = unsafe { self.key_assign.as_ref() };
            key_assign.render_debug(ui);
        });

        ui.header("input_type_group", || {
            let input_type_group = unsafe { self.input_type_group.as_ref() };
            ui.table(
                "input_type_group",
                [TableColumnSetup::new("Key"), TableColumnSetup::new("Group")],
                input_type_group.iter(),
                |ui, _, pair| {
                    ui.table_next_column();
                    ui.text(format!("{}", pair.key));
                    ui.table_next_column();
                    pair.value.render_debug(ui);
                },
            );
        });

        ui.header("input_code_check", || {
            let input_code_check = unsafe { self.input_code_check.as_ref() };
            ui.table(
                "input_code_check",
                [
                    TableColumnSetup::new("Key"),
                    TableColumnSetup::new("state_1"),
                    TableColumnSetup::new("state_2"),
                ],
                input_code_check.iter(),
                |ui, _, pair| {
                    ui.table_next_column();
                    ui.text(format!("{}", pair.key));
                    ui.table_next_column();
                    ui.text(format!("{}", pair.value.state_1));
                    ui.table_next_column();
                    ui.text(format!("{}", pair.value.state_2));
                },
            );
        });

        ui.text(format!(
            "unk48 DLString<wchar_t>: {}",
            self.empty_str.to_string()
        ));

        ui.header("window_cursor_context", || {
            let window_cursor_context = unsafe { self.window_cursor_context.as_ref() };
            window_cursor_context.render_debug(ui);
        });
    }
}

impl DebugDisplay for InputTypeGroup {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.table(
            "input_type_group",
            [
                TableColumnSetup::new("input_code"),
                TableColumnSetup::new("input_type"),
            ],
            self.iter(),
            |ui, _, (input_code, input_type)| {
                ui.table_next_column();
                ui.text(format!("{}", input_code));
                ui.table_next_column();
                let text = match input_type {
                    InputType::AreKeysDown => "AreKeysDown",
                    InputType::AreKeysUp => "AreKeysUp",
                    InputType::IsStickMoving => "IsStickMoving",
                };
                ui.text(text);
            },
        );
    }
}

impl DebugDisplay for CSKeyAssign {
    fn render_debug(&self, ui: &Ui) {
        let program = &Program::current();
        let vftable = self.vftable;
        if let Some(class_name) = vftable_classname(program, vftable) {
            ui.text(format!("addr {:#X}\nname: {}", vftable, class_name));
        } else {
            ui.text("CSKeyAssign instance rtti couldn't be read.");
        }

        ui.header("keybind_vector", || {
            let keybind_vector = &self.keybind_vector;
            ui.table(
                "keybind_vector",
                [
                    TableColumnSetup::new("key"),
                    TableColumnSetup::new("value"),
                    TableColumnSetup::new("unk8"),
                    TableColumnSetup::new("unkc"),
                    TableColumnSetup::new("unk10"),
                ],
                keybind_vector.items().iter(),
                |ui, _, pair| {
                    ui.table_next_column();
                    ui.text(format!("{}", pair.key));
                    ui.table_next_column();
                    ui.text(format!("{}", pair.value));
                    ui.table_next_column();
                    ui.text(format!("{}", pair.unk8));
                    ui.table_next_column();
                    ui.text(format!("{}", pair.unkc));
                    ui.table_next_column();
                    ui.text(format!("{}", pair.unk10));
                },
            );
        });

        ui.header("virtual_input_data_index_map", || {
            let bitset_index_map = unsafe { self.virtual_input_data_index_map.as_ref() };
            ui.table(
                "virtual_input_data_index_map",
                [TableColumnSetup::new("key"), TableColumnSetup::new("value")],
                bitset_index_map.iter(),
                |ui, _, pair| {
                    ui.table_next_column();
                    ui.text(format!("{}", pair.key));
                    ui.table_next_column();
                    ui.text(format!("{}", pair.value));
                },
            );
        });

        ui.header("unk78_index_map", || {
            let bitset_fallback_map = &self.unk78_index_map;
            ui.table(
                "unk78_index_map",
                [TableColumnSetup::new("key"), TableColumnSetup::new("value")],
                bitset_fallback_map.iter(),
                |ui, _, pair| {
                    ui.table_next_column();
                    ui.text(format!("{}", pair.key));
                    ui.table_next_column();
                    ui.text(format!("{}", pair.value));
                },
            );
        });
    }
}

impl DebugDisplay for MultiDevices {
    fn render_debug(&self, ui: &Ui) {
        ui.header("VirtualMultiDevice", || {
            let vm_device = unsafe { self.virtual_multi_device.as_ref() };
            vm_device.render_debug(ui);
        });

        
        for (index, pad_device_ptr) in self.pad_devices.iter().enumerate() {
            ui.header(format!("PadDevice[{}]", index), || {
                let pad_device = unsafe { pad_device_ptr.as_ref() };
                pad_device.render_debug(ui);
            });
        }

        ui.header("MouseDevice", || {
            let mouse_device = unsafe { self.mouse_device.as_ref() };
            mouse_device.render_debug(ui);
        });

        ui.header("KeyboardDevice", || {
            let keyboard_device = unsafe { self.keyboard_device.as_ref() };
            keyboard_device.render_debug(ui);
        });

        ui.header("unk78", || {
            self.unk78.render_debug(ui);
        });
    }
}

impl DebugDisplay for MultiDevices_0x78 {
    fn render_debug(&self, ui: &Ui) {
        ui.table(
            "Bitset fallback",
            [
                TableColumnSetup::new("Index"),
                TableColumnSetup::new("State"),
            ],
            self.bitset_fallback.iter(),
            |ui, index, state| {
                ui.table_next_column();
                ui.text(format!("{}", index));
                ui.table_next_column();
                ui.text(format!("{}", *state));
            },
        );
    }
}

impl DebugDisplay for VirtualMultiDevice {
    fn render_debug(&self, ui: &Ui) {
        ui.header("DLUserInputDeviceImpl", || {
            self.user_input_device_impl.render_debug(ui);
        });
    }
}

impl DebugDisplay for PadDevice {
    fn render_debug(&self, ui: &Ui) {
        ui.header("DLUserInputDeviceImpl", || {
            self.user_input_device_impl.render_debug(ui);
        });
    }
}

impl DebugDisplay for MouseDevice {
    fn render_debug(&self, ui: &Ui) {
        ui.header("DLUserInputDeviceImpl", || {
            self.user_input_device_impl.render_debug(ui);
        });
    }
}

impl DebugDisplay for KeyboardDevice {
    fn render_debug(&self, ui: &Ui) {
        ui.header("DLUserInputDeviceImpl", || {
            self.user_input_device_impl.render_debug(ui);
        });
    }
}

impl DebugDisplay for DLUserInputDeviceImpl {
    fn render_debug(&self, ui: &Ui) {
        ui.header("virtual_input_data", || {
            self.virtual_input_data.render_debug(ui);
        });
        ui.header("analog_positive_axis", || {
            self.analog_positive_axis.render_debug(ui);
        });
        ui.header("analog_negative_axis", || {
            self.analog_negative_axis.render_debug(ui);
        });
        ui.header("initial_virtual_input_data", || {
            self.initial_virtual_input_data.render_debug(ui);
        });
    }
}

impl DebugDisplay for DLVirtualInputData {
    fn render_debug(&self, ui: &Ui) {
        ui.header("DLVirtualAnalogKeyInfo<f32>", || {
            self.analog_key_info.render_debug(ui);
        });
        ui.header("DynamicBitset", || {
            self.dynamic_bitset.render_debug(ui);
        });
    }
}

impl DebugDisplay for DLVirtualAnalogKeyInfo<f32> {
    fn render_debug(&self, ui: &Ui) {
        ui.table(
            "DLVirtualAnalogKeyInfo",
            [
                TableColumnSetup::new("Index"),
                TableColumnSetup::new("Item"),
            ],
            self.vector.items(),
            |ui, index, item| {
                ui.table_next_column();
                ui.text(format!("{}", index));
                ui.table_next_column();
                ui.text(format!("{:010.4}", item));
            },
        );
    }
}

impl DebugDisplay for DynamicBitset {
    fn render_debug(&self, ui: &Ui) {
        ui.table(
            "DynamicBitset",
            [
                TableColumnSetup::new("Index"),
                TableColumnSetup::new("Item"),
            ],
            self.as_slice().iter(),
            |ui, index, item| {
                ui.table_next_column();
                ui.text(format!("{}", index));
                ui.table_next_column();
                ui.text(format!("{:032b}", item));
            },
        );
    }
}

impl DebugDisplay for WindowCursorContext {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("window_handle: {:#X}", self.window_handle));

        ui.text(format!(
            "Cursor\nx: {:06}\ny: {:06}",
            self.cursor_x, self.cursor_y
        ));
    }
}
