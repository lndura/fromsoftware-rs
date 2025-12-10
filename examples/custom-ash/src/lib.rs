use eldenring::{
    cs::{CSGaitemImp, CSTaskGroupIndex, CSTaskImp, ChrAsmArmStyle, ItemId, WorldChrMan},
    fd4::FD4TaskData,
    util::system::wait_for_system_init,
};
use shared::{FromStatic, Program, SharedTaskImpExt};
use std::time::Duration;

mod weaponlogic;
use crate::weaponlogic::WeaponList;

/// Creates a task that compares our current left and right weapons to those of our list.
fn init_custom_ashes_task() {
    // Don't insert the task if we don't have a proper list.
    let weapon_list = match WeaponList::from_file("weapon-list.toml") {
        Some(list) => {
            if !list.weapon.is_empty() {
                list
            } else {
                return;
            }
        }
        None => return,
    };

    let cs_task = (unsafe { CSTaskImp::instance() }).expect("Could not obtain CSTaskImp instance.");

    cs_task.run_recurring(
        move |_: &FD4TaskData| {
            // Don't run during loading screen
            let Some(main_player) = unsafe { WorldChrMan::instance() }
                .ok()
                .and_then(|wrld_chr_man| wrld_chr_man.main_player.as_ref())
            else {
                return;
            };

            // This singleton manages items. Can also be used to change a.e npc armor sets at runtime.
            let Some(cs_gaitem) = unsafe { CSGaitemImp::instance() }.ok() else {
                return;
            };

            // Don't run if we don't hold a weapon.
            let arm_style = &main_player.chr_asm.equipment.arm_style;
            match arm_style {
                ChrAsmArmStyle::EmptyHanded => return,
                _ => {}
            }

            // guh.
            // Binds left and right to 0 and 1 indexes and their respective equipment slot.
            // Equipment slots are ordered as: left0, right0, left1, right1, left2, right2 in the first 6 gaitem_handles.
            let inv_slots = &main_player.chr_asm.equipment.selected_slots;
            let wep_slots = [
                0 + (inv_slots.left_weapon_slot * 2) as usize,
                1 + (inv_slots.right_weapon_slot * 2) as usize,
            ];

            for (index, slot) in wep_slots.iter().enumerate() {
                // Grab the item from CSGaitem using the handle.
                let weapon_handle = &main_player.chr_asm.gaitem_handles[*slot];
                let Some(weapon_ins) = cs_gaitem
                    .gaitem_ins_by_handle_mut(weapon_handle)
                    .and_then(|gaitem_ins| gaitem_ins.as_wep_mut())
                else {
                    continue;
                };

                // Check if the weapon should have a custom ash attached to it.
                let weapon_pid = weapon_ins.gaitem_ins.item_id.item_id() as u32;
                let Some(custom_ash) = weapon_list.get_custom_ash(weapon_pid) else {
                    continue;
                };

                // Grab the item from CSGaitem using the handle.
                let gem_handle = weapon_ins.gem_slot_table.gem_slots[0].gaitem_handle;
                let Some(gem_ins) = cs_gaitem
                    .gaitem_ins_by_handle_mut(&gem_handle)
                    .and_then(|gaitem_ins| gaitem_ins.as_gem_mut())
                else {
                    continue;
                };

                // index is 0 (left) or 1 (right) and changes the equipped ash accordingly.
                gem_ins.gaitem_ins.item_id = match arm_style {
                    ChrAsmArmStyle::LeftBothHands if index == 0 => {
                        ItemId(custom_ash.two_handed as i32)
                    }
                    ChrAsmArmStyle::RightBothHands if index == 1 => {
                        ItemId(custom_ash.two_handed as i32)
                    }
                    _ => ItemId(custom_ash.one_handed as i32),
                };
            }
        },
        CSTaskGroupIndex::GameMan,
    );
}

// Exposed for dll loaders, a.e ModEngine 3.
#[unsafe(no_mangle)]
unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    if reason != 1 {
        return true;
    }

    std::thread::spawn(move || {
        // Wait for the game to initialize. Panic if it doesn't.
        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Could not await system init.");

        // Initialize task.
        init_custom_ashes_task();
    });
    true
}
