use std::ptr::NonNull;

use bitfield::bitfield;

use crate::{Vector, dluid::DLUserInputDeviceImpl};

/// TODO:
/// Check if VirtMulDevice refrences fields in Mouse, Keyboard and Pad devices.
#[repr(C)]
pub struct MultiDevices {
    vftable: usize,
    pub virtual_multi_device: NonNull<VirtualMultiDevice>,
    pub pad_devices: [NonNull<PadDevice>; 4],
    unk30: [u8; 0x10],
    pub mouse_device: NonNull<MouseDevice>,
    pub keyboard_device: NonNull<KeyboardDevice>,
    unk50: [u8; 0x28],
    pub unk78: MultiDevices_0x78,
    unk3b0: [u8; 16],
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct MultiDevices_0x78 {
    vftable: usize,
    allocator: usize,
    pub bitset_fallback: [bool; 162],
    padding: [u8; 6],
    unkb8: [u8; 0x280],
    unk334: u8,
}

// #[repr(C)]
// pub struct MultiDevices_0x78ArrayEntry {
//     pub unk00: u32,
//     pub unk04: u32,
//     pub unk08: bool,
//     padding: [u8; 7],
// }

#[repr(C)]
pub struct VirtualMultiDevice {
    pub user_input_device_impl: DLUserInputDeviceImpl,
    /// Contains a list of pointers to PadDevice, MouseDevice and KeyboardDevice instances.
    device_list: Vector<NonNull<DLUserInputDeviceImpl>>,
}

#[repr(C)]
pub struct PadDevice {
    pub user_input_device_impl: DLUserInputDeviceImpl,
    //unk7d8: [u8; 0x290],
    unk7d8: i32,
    unk7dc: [u8; 4],
    unk7e0: [u8; 0x60],
    /// set by memset in vfptr[43]
    unk840: [u8; 80],
    /// `WORD` bitfield of `XInputGetState()`'s wButtons field.
    pub w_buttons: WButtons,
    // unk892: u16,
    /// Index of the user's controller. Can be a value from 0 to 3.
    pub dw_user_index: i32,
    unk898: [u8; 4],
    pub s_thumb_lx: f32,
    pub s_thumb_ly: f32,
    unk8a4: [u8; 4],
    pub s_thumb_rx: f32,
    pub s_thumb_ry: f32,
    unk8b0: [u8; 12],
    pub b_left_trigger: f32,
    pub b_right_trigger: f32,
    //unk8c4: [u8; 0x1A4]
    // TODO: fill this out...
}

bitfield! {
    /// Source: https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad
    #[repr(C)]
    pub struct WButtons(u16);
    impl Debug;

    pub dpad_up,        set_dpad_up:        0;
    pub dpad_down,      set_dpad_down:      1;
    pub dpad_left,      set_dpad_left:      2;
    pub dpad_right,     set_dpad_right:     3;

    pub start,          set_start:          4;
    pub back,           set_back:           5;

    pub left_thumb,     set_left_thumb:     6;
    pub right_thumb,    set_right_thumb:    7;

    pub left_shoulder,  set_left_shoulder:  8;
    pub right_shoulder, set_right_shoulder: 9;

    pub button_a,              set_a:              12;
    pub button_b,              set_b:              13;
    pub button_x,              set_x:              14;
    pub button_y,              set_y:              15;
}

#[repr(C)]
pub struct MouseDevice {
    pub user_input_device_impl: DLUserInputDeviceImpl,
    unk7d8: i32,
    unk7dc: [u8; 4],
    // Pointer to some Steam Gameoverlay bullshit that makes DirectInput calls to mouse and keyboard devices in DirectX itself.
    unk7e0: usize,
    /// Result of DirectX8 `GetDeviceState`.
    pub mouse_state: DIMouseState2,
    unk7fc: bool,
    unk7fd: u8,
    unk7fe: u8,
    unk7ff: u8,
    /// Horizontal mouse movement. 
    pub normalized_lx: i32,
    /// Vertical mouse movement.
    pub normalized_ly: i32,
    /// Scroll mouse movement.
    pub normalized_lz: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DIMouseButton {
    Left        = 0,
    Right       = 1,
    Middle      = 2,
    Button4     = 3,
    Button5     = 4,
    Button6     = 5,
    Button7     = 6,
    Button8     = 7,
}

/// Source of name: https://learn.microsoft.com/en-us/previous-versions/windows/desktop/ee416631(v=vs.85)
#[repr(C)]
pub struct DIMouseState2 {
    /// Horizontal mouse movement.
    pub lx: i32,
    /// Vertical mouse movement.
    pub ly: i32,
    /// Scroll mouse movement.
    pub lz: i32,
    /// Mouse buttons 1-8
    pub buttons: [u8; 8]
}

impl DIMouseState2 {
    /// See [DIMouseButton] for reference.
    pub fn pressed<K: Into<usize>>(&self, button: K) -> bool {
        self.buttons[button.into()] & 0x80 != 0
    }
}

#[repr(C)]
pub struct KeyboardDevice {
    pub user_input_device_impl: DLUserInputDeviceImpl,
    unk7d8: [u8; 0x118],
}

#[repr(C)]
pub struct DummyDevice {
    user_input_device_impl: DLUserInputDeviceImpl,
}