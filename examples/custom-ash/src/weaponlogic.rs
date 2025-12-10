
use eldenring::{fd4::FD4ParamRepository, param::EQUIP_PARAM_WEAPON_ST};
use serde::Deserialize;
use shared::FromStatic;

#[derive(Default, Deserialize, Clone)]
pub struct CustomAsh {
    pub weapon_pid: u32,
    pub one_handed: u32,
    pub two_handed: u32,
    pub ignore_lvl: bool,
}

impl CustomAsh {
    pub fn max_range(&self) -> u32 {
        let weapon_pid = self.weapon_pid as u32;
        if self.ignore_lvl {
            if let Some(weapon_param) = unsafe { FD4ParamRepository::instance() }
                .ok()
                .and_then(|param| param.get::<EQUIP_PARAM_WEAPON_ST>(weapon_pid))
            {
                if weapon_param.origin_equip_wep16() == i32::MAX {
                    return weapon_pid + 10;
                } else {
                    return weapon_pid + 25;
                }
            }
        }
        weapon_pid
    }
}

#[derive(Default, Deserialize, Clone)]
pub struct WeaponList {
    pub weapon: Vec<CustomAsh>,
}

impl WeaponList {
    pub fn from_file(path: &str) -> Option<WeaponList> {
        let text = std::fs::read_to_string(path).ok()?;
        toml::from_str(&text).ok()?
    }
    pub fn get_custom_ash(&self, weapon_pid: u32) -> Option<&CustomAsh> {
        self.weapon.iter().find_map(|weapon| {
            let max_range = weapon.max_range();
            if weapon_pid >= weapon.weapon_pid && weapon_pid <= max_range {
                Some(weapon)
            } else {
                None
            }
        })
    }
}
