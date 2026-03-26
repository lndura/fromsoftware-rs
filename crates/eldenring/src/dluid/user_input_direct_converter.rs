use std::ptr::NonNull;

use crate::Vector;

/// Base class that manages modifiers and converters.
#[repr(C)]
pub struct UserInputExtension {
    vftable: usize,
}

/// Subclass of [UserInputExtension].
#[repr(C)]
pub struct DLUserInputPhysicalAnalogModifier {
    vftable: usize,
}

/// Subclass of [UserInputExtension].
#[repr(C)]
pub struct DLUserInputDirectConverter {
    vftable: usize,
    unk08: Vector<usize>,
    unk28: Vector<usize>,
    /// Points to field `unk08`.
    unk08_ptr: NonNull<Vector<usize>>,
    /// Points to field `unk28`.
    unk28_ptr: NonNull<Vector<usize>>,
}

