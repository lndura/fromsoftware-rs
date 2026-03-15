use std::ptr::NonNull;

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

#[repr(C)]
pub struct MultiDevices0x78ArrayEntry {
    pub unk00: u32,
    pub unk04: u32,
    pub unk08: bool,
    padding: [u8; 7],
}

#[repr(C)]
pub struct VirtualMultiDevice {
    pub user_input_device_impl: DLUserInputDeviceImpl,
    /// Contains a list of pointers to PadDevice, MouseDevice and KeyboardDevice instances.
    device_list: Vector<NonNull<usize>>,
}

#[repr(C)]
pub struct PadDevice {
    pub user_input_device_impl: DLUserInputDeviceImpl,
    unk7d8: [u8; 0x290],
}

#[repr(C)]
pub struct MouseDevice {
    pub user_input_device_impl: DLUserInputDeviceImpl,
    unk7d8: [u8; 0x38],
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
