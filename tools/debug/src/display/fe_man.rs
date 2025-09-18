use eldenring::cs::{
    CSFeManImp, ChrEnemyTagEntry, ChrFriendTagEntry, FrontEndViewValues, TagHudData,
};
use hudhook::imgui::{TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for CSFeManImp {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("HUD State: {:?}", self.hud_state));

        if ui.collapsing_header("Debug Tag", TreeNodeFlags::empty()) {
            ui.indent();
            self.debug_tag.render_debug(ui);
            ui.unindent();
        }

        if ui.collapsing_header("Enemy Character Tags", TreeNodeFlags::empty()) {
            ui.indent();
            for (i, tag) in self.enemy_chr_tag_displays.iter().enumerate() {
                if ui.collapsing_header(format!("Enemy Tag {i}"), TreeNodeFlags::empty()) {
                    ui.indent();
                    tag.render_debug(ui);
                    ui.unindent();
                }
            }
            ui.unindent();
        }

        if ui.collapsing_header("Friendly Character Tags", TreeNodeFlags::empty()) {
            ui.indent();
            for (i, tag) in self.friendly_chr_tag_displays.iter().enumerate() {
                if ui.collapsing_header(format!("Friendly Tag {i}"), TreeNodeFlags::empty()) {
                    ui.indent();
                    tag.render_debug(ui);
                    ui.unindent();
                }
            }
            ui.unindent();
        }

        if ui.collapsing_header("Boss Health Displays", TreeNodeFlags::empty()) {
            ui.indent();
            for (i, boss) in self.boss_health_displays.iter().enumerate() {
                if ui.collapsing_header(format!("Boss {i}"), TreeNodeFlags::empty()) {
                    ui.indent();
                    ui.text(format!("FMG ID: {}", boss.fmg_id));
                    ui.text(format!("Handle: {:?}", boss.field_ins_handle));
                    ui.text(format!("Damage Taken: {}", boss.damage_taken));
                    ui.unindent();
                }
            }
            ui.unindent();
        }

        if ui.collapsing_header("Status Messages", TreeNodeFlags::empty()) {
            ui.indent();
            ui.text(format!(
                "Read Index: {}",
                self.proc_status_messages_read_index
            ));
            ui.text(format!(
                "Write Index: {}",
                self.proc_status_messages_write_index
            ));

            if ui.collapsing_header("Message Buffer", TreeNodeFlags::empty()) {
                ui.indent();
                for (i, msg_id) in self.proc_status_messages.iter().enumerate() {
                    ui.text(format!("Message {i}: {msg_id}"));
                }
                ui.unindent();
            }

            ui.text(format!(
                "Subarea Name Popup ID: {}",
                self.subarea_name_popup_message_id
            ));
            ui.text(format!(
                "Area Welcome Message Request: {}",
                self.area_welcome_message_request
            ));
            ui.text(format!(
                "Damage Number Decay Time: {:.1}s",
                self.damage_number_decay_time
            ));
            ui.unindent();
        }

        if ui.collapsing_header("FrontEndView", TreeNodeFlags::empty()) {
            ui.indent();
            self.frontend_values.render_debug(ui);
            ui.unindent();
        }
    }
}

impl DebugDisplay for TagHudData {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!(
            "HP: {}/{} (Max Uncapped: {})",
            self.hp,
            self.hp_max_uncapped - self.hp_max_uncapped_difference,
            self.hp_max_uncapped
        ));

        ui.text(format!("Name: {}", self.chr_name));
        ui.text(format!("Role: {}", self.role_string));
        ui.text(format!("Role Name Color: {}", self.role_name_color));
        ui.text(format!("Has Rune Arc: {}", self.has_rune_arc));
        ui.text(format!("Is Visible: {}", self.is_visible));
        ui.text(format!("Update Position: {}", self.update_position));
        ui.text(format!("Not On Screen: {}", self.is_not_on_screen));
        ui.text(format!("Is Down Scaled: {}", self.is_down_scaled));
        ui.text(format!("Last Damage Taken: {}", self.last_damage_taken));
        ui.text(format!("Last HP Value: {}", self.last_hp_value));
        ui.text(format!(
            "Screen Position: ({:.1}, {:.1})",
            self.screen_pos_x, self.screen_pos_y
        ));
        ui.text(format!("Handle: {:?}", self.field_ins_handle));
    }
}

impl DebugDisplay for ChrFriendTagEntry {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Is Visible: {}", self.is_visible));
        ui.text(format!(
            "Line of Sight Blocked: {}",
            self.is_line_of_sight_blocked
        ));
        ui.text(format!("Not On Screen: {}", self.is_not_on_screen));
        ui.text(format!("Is Debug Summon: {}", self.is_debug_summon));
        ui.text(format!("Is Down Scaled: {}", self.is_down_scaled));
        ui.text(format!("Has Rune Arc: {}", self.has_rune_arc));

        ui.text(format!("Team Type: {}", self.team_type));
        ui.text(format!("Role Name Color: {}", self.role_name_color));
        ui.text(format!("Voice Chat State: {}", self.voice_chat_state));

        ui.text(format!("Name: {}", self.name_string));
        ui.text(format!("Role: {}", self.role_string));

        ui.text(format!(
            "HP: {}/{} (Max Uncapped: {})",
            self.hp, self.max_hp, self.hp_max_uncapped
        ));
        ui.text(format!("Max Recoverable HP: {}", self.max_recoverable_hp));
        ui.text(format!(
            "Last Damage Time: {:.1}s",
            self.last_damage_time_delta
        ));
        ui.text(format!(
            "Screen Position: ({:.1}, {:.1}, {:.1}, {:.1})",
            self.screen_pos.0, self.screen_pos.1, self.screen_pos.2, self.screen_pos.3
        ));
        ui.text(format!("Handle: {:?}", self.field_ins_handle));
    }
}

impl DebugDisplay for ChrEnemyTagEntry {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Is Visible: {}", self.is_visible));
        ui.text(format!("Damage Taken: {}", self.damage_taken));
        ui.text(format!("Pre-Damage HP: {}", self.pre_damage_hp));
        ui.text(format!(
            "Last Update Time: {:.1}s",
            self.last_update_time_delta
        ));
        ui.text(format!(
            "Last Damage Time: {:.1}s",
            self.last_damage_time_delta
        ));
        ui.text(format!(
            "Screen Position: ({:.1}, {:.1}, {:.1}, {:.1})",
            self.screen_pos.0, self.screen_pos.1, self.screen_pos.2, self.screen_pos.3
        ));
        ui.text(format!("Handle: {:?}", self.field_ins_handle));
    }
}

impl DebugDisplay for FrontEndViewValues {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Player Stats", TreeNodeFlags::DEFAULT_OPEN) {
            ui.indent();
            ui.text(format!(
                "HP: {}/{} (Max Uncapped: {})",
                self.player_hp,
                self.hp_max_uncapped - self.hp_max_uncapped_difference,
                self.hp_max_uncapped
            ));
            ui.text(format!("Max Recoverable HP: {}", self.max_recoverable_hp));
            ui.text(format!("FP: {}/{}", self.fp, self.fp_max));
            ui.text(format!("Stamina: {}/{}", self.stamina, self.stamina_max));
            ui.text(format!("HP Rally Enabled: {}", self.enable_hp_rally));
            ui.text(format!("Equip HUD Enabled: {}", self.enable_equip_hud));
            ui.text(format!("Sword Arts Name: {}", self.sword_arts_name_string));
            ui.unindent();
        }

        if ui.collapsing_header("Enemy Tags", TreeNodeFlags::empty()) {
            ui.indent();
            for (i, tag) in self.enemy_chr_tag_data.iter().enumerate() {
                if ui.collapsing_header(format!("Enemy {i}"), TreeNodeFlags::empty()) {
                    ui.indent();
                    tag.render_debug(ui);
                    ui.unindent();
                }
            }
            ui.unindent();
        }

        if ui.collapsing_header("Boss List Tags", TreeNodeFlags::empty()) {
            ui.indent();
            for (i, tag) in self.boss_list_tag_data.iter().enumerate() {
                if ui.collapsing_header(format!("Boss {i}"), TreeNodeFlags::empty()) {
                    ui.indent();
                    tag.render_debug(ui);
                    ui.unindent();
                }
            }
            ui.unindent();
        }

        if ui.collapsing_header("Friendly Tags", TreeNodeFlags::empty()) {
            ui.indent();
            for (i, tag) in self.friendly_chr_tag_data.iter().enumerate() {
                if ui.collapsing_header(format!("Friendly {i}"), TreeNodeFlags::empty()) {
                    ui.indent();
                    tag.render_debug(ui);
                    ui.unindent();
                }
            }
            ui.unindent();
        }

        if ui.collapsing_header("Status Message", TreeNodeFlags::empty()) {
            ui.indent();
            ui.text(format!("Message: {}", self.proc_status_message));
            ui.text(format!("Timer: {:.1}s", self.proc_status_message_timer));
            ui.text(format!(
                "Full Screen Message: {:?}",
                self.full_screen_message_request_id
            ));
            ui.unindent();
        }

        if ui.collapsing_header("Spirit Ashes", TreeNodeFlags::empty()) {
            ui.indent();
            ui.text(format!(
                "Summoned Spirit Ash Count: {}",
                self.summoned_spirit_ash_count
            ));

            for (i, spirit) in self.spirit_ash_display.iter().enumerate() {
                if ui.collapsing_header(format!("Spirit Ash {i}"), TreeNodeFlags::empty()) {
                    ui.indent();
                    ui.text(format!(
                        "HP: {}/{} (Max Uncapped: {})",
                        spirit.hp,
                        spirit.hp_max_uncapped - spirit.hp_max_uncapped_difference,
                        spirit.hp_max_uncapped
                    ));
                    ui.text(format!("Handle: {:?}", spirit.field_ins_handle));
                    ui.unindent();
                }
            }
            ui.unindent();
        }

        if ui.collapsing_header("Arena Info", TreeNodeFlags::empty()) {
            ui.indent();
            ui.text(format!(
                "Elimination Count: {}",
                self.quickmatch_elimination_count
            ));
            ui.unindent();
        }
    }
}
