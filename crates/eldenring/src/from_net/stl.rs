use crate::from_net::FnAllocatorProxy;

pub type FNVector<T> = fromsoftware_shared_stl::Vector<T, FnAllocatorProxy>;
pub type FNString = fromsoftware_shared_stl::BasicString<u8, FnAllocatorProxy>;
