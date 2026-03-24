use core::slice;
use std::ptr::NonNull;

use crate::{Vector, dlkr::DLPlainLightMutex};

/// Source of name: RTTI
#[repr(C)]
pub struct DLUserInputDeviceImpl {
    _vftable: usize,
    unk008: usize,
    /// Contains a reference to the same [DLVirtualInputData] from `initial_virtual_input_data`.
    ///
    /// The game accesses this from [FD4PadManager] and it's [CSPad] instances to poll inputs.
    pub virtual_input_data: DLVirtualInputData,
    user_input_converters: Vector<NonNull<usize>>,
    unk080: usize,
    unk088: usize,
    pub mutex: DLPlainLightMutex,
    unk0c0: f32,
    unk0c4: f32,
    pub analog_positive_axis: DLVirtualAnalogKeyInfo<f32>,
    pub analog_negative_axis: DLVirtualAnalogKeyInfo<f32>,
    unk118: u8,
    unk11c: u32,
    unk120: usize,
    unk128: u32,
    unk12c: u32,
    unk130: usize,
    pub unk138: DLuserInputDeviceImpl0x138,
    unk750: [u8; 0x18],
    user_input_mapper_slots: Vector<NonNull<usize>>,
    /// The [DLVirtualInputData] is inserted here and gets memcpy'd over to `virtual_input_data`
    pub initial_virtual_input_data: DLVirtualInputData,
}

#[repr(C)]
pub struct DLuserInputDeviceImpl0x138 {
    pub entries: [DLuserInputDeviceImpl0x138Entry; 0x40],
    /// index game will use to update from an entry
    pub index: usize,
    /// counter that gets incremented
    pub counter: u64,
    /// copied over from counter.
    pub counter_reference: u64
}

#[repr(C)]
pub struct DLuserInputDeviceImpl0x138Entry {
    pub virtual_input_data: NonNull<DLVirtualInputData>,
    /// reference to counter in DLuserInputDeviceImpl0x138.counter
    pub counter_reference: u64,
    /// Result of Windows QueryPerformanceCounter.
    pub performance_counter: usize,
}

impl DLUserInputDeviceImpl {
    pub fn get_virtual_analog_state(&self, index: usize) -> f32 {
        self.virtual_input_data.get_analog(index)
    }
    pub fn set_virtual_analog_state(&mut self, index: usize, state: f32) {
        self.virtual_input_data.set_analog(index, state)
    }
    pub fn get_virtual_digital_state(&self, index: usize) -> bool {
        self.virtual_input_data.get_digital(index)
    }
    pub fn set_virtual_digital_state(&mut self, index: usize, state: bool) {
        self.virtual_input_data.set_digital(index, state)
    }
}

/// Source of name: RTTI
#[repr(C)]
pub struct DLVirtualAnalogKeyInfo<T> {
    vftable: usize,
    pub vector: Vector<T>,
}

/// Source of name: RTTI
#[repr(C)]
pub struct DLVirtualInputData {
    vftable: usize,
    /// Corresponds to movement inputs such as Mouse, Stick and character movement keys.
    pub analog_key_info: DLVirtualAnalogKeyInfo<f32>,
    /// Corresponds to action inputs such as jump, crouch and attacks.
    pub dynamic_bitset: DynamicBitset,
}

impl DLVirtualInputData {
    pub fn get_analog(&self, index: usize) -> f32 {
        let vector = &self.analog_key_info.vector;
        if index < vector.len() {
            let items = self.analog_key_info.vector.items();
            return items[index];
        }

        0.0
    }
    pub fn set_analog(&mut self, index: usize, state: f32) {
        let vector = &mut self.analog_key_info.vector;
        if index < vector.len() {
            let items = vector.items_mut();
            items[index] = state;
        }
    }
    pub fn get_digital(&self, index: usize) -> bool {
        self.dynamic_bitset.get(index)
    }
    pub fn set_digital(&mut self, index: usize, state: bool) {
        self.dynamic_bitset.set(index, state);
    }
}

/// Source of name: RTTI
#[repr(C)]
pub struct DynamicBitset {
    vftable: usize,
    /// Corresponds to the amount of integers (32 bit-size) required to store the bitfield.
    ///
    /// Calculated during creation as:
    ///
    /// integer_count = bit_count // 32 * 4.
    integer_count: usize,
    /// Bitfield that this [DynamicBitset] corresponds to.
    ///
    /// It's allocated as an array of integers with the size of `integer_count`.
    bitset: NonNull<u32>,
    allocator: usize,
}

impl DynamicBitset {
    pub fn as_slice(&self) -> &[u32] {
        unsafe {
            let data = self.bitset.as_ptr();
            slice::from_raw_parts(data, self.len())
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [u32] {
        unsafe {
            let data = self.bitset.as_ptr();
            slice::from_raw_parts_mut(data, self.len())
        }
    }

    pub fn len(&self) -> usize {
        self.integer_count
    }

    pub fn get(&self, bit_index: usize) -> bool {
        let slice: &[u32] = self.as_slice();

        let index: usize = bit_index / 32;
        let row: u32 = slice[index];
        let shift: usize = bit_index & 31;

        ((row >> shift) & 1) == 1
    }

    pub fn set(&mut self, bit_index: usize, state: bool) {
        let slice = self.as_slice_mut();

        let index = bit_index / 32;
        let row = &mut slice[index];
        let shift = bit_index & 31;

        let mask = 1u32 << shift;

        *row = (*row & !mask) | ((state as u32) << shift);
    }
}
