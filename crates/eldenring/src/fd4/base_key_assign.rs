use std::ptr::NonNull;

use crate::{Pair, Tree, Vector};

#[repr(C)]
pub struct FD4BaseKeyAssign {
    pub vftable: usize,
    allocator: usize,
    input_mapper: usize,
    pub keybind_vector: Vector<KeyBind>,
    /// Takes the result from the [InputTypeGroup] and maps it to an index to the [DLVirtualInputData].
    pub virtual_input_data_index_map: NonNull<Tree<Pair<i32, i32>>>,
    // what the skibidi
    pub unk78_index_map: Tree<Pair<i32, i32>>,
    /// Contains the same pointer that the [DLFixedVector<>] in `FD4PadManager.unka8` has.
    unk58: usize,
    unk60: u32,
    unk64: u32,
    unk68: u32,
    unk6c: u32,
    unk70: u32,
    padding: [u8; 4],
}

#[repr(C)]
pub struct KeyBind {
    pub key: i32,
    pub value: i32,
    pub unk8: i32,
    pub unkc: i32,
    pub unk10: i32,
}
