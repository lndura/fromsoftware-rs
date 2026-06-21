use crate::{DLMap, dltx::{DLString, DLUTF16StringKind}};


#[repr(C)]
/// Source of name: RTTI
#[shared::singleton("UserConfig")]
pub struct UserConfig {
    vfptr: *const (),
    allocator: *const (),
    pub debug_property_map: DLMap<DLString<DLUTF16StringKind>, DLString<DLUTF16StringKind>>
}