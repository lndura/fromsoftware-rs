use std::{ffi::c_void, ptr::NonNull};

use crate::dlkr::DLAllocatorBase;

#[derive(Clone)]
#[repr(transparent)]
/// Special type to use in std types.
pub struct DLAllocatorForStl(NonNull<DLAllocatorBase>);

impl From<NonNull<DLAllocatorBase>> for DLAllocatorForStl {
    fn from(ptr: NonNull<DLAllocatorBase>) -> Self {
        Self(ptr)
    }
}

impl fromsoftware_shared_stl::StlAllocator for DLAllocatorForStl {
    unsafe fn allocate_raw(&mut self, size: usize, align: usize) -> *mut c_void {
        let allocator = unsafe { self.0.as_mut() };
        let allocation = (allocator.vftable.allocate_aligned)(allocator, size, align);
        if allocation.is_null() {
            panic!("DLAllocator returned null pointer")
        }
        allocation as _
    }

    unsafe fn deallocate_raw(&mut self, ptr: *mut c_void) {
        let allocator = unsafe { self.0.as_mut() };
        (allocator.vftable.deallocate)(allocator, ptr as _);
    }
}

pub type DLVector<T> = fromsoftware_shared_stl::Vector<T, DLAllocatorForStl>;
