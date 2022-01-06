use core::{
    alloc::Layout,
    any::{type_name, TypeId},
    ptr::{copy_nonoverlapping, drop_in_place, slice_from_raw_parts_mut},
};

/// Re-export derive proc macros.
pub use ::edict_proc::{PinComponent, UnpinComponent};

/// Trait that is implemented for all types that can act as a component.
pub trait Component: 'static {}

impl<T> Component for T where T: 'static {}

#[derive(Clone, Copy, Debug)]
pub struct ComponentInfo {
    pub id: TypeId,
    pub layout: Layout,
    pub debug_name: &'static str,
    pub drop: unsafe fn(*mut u8, usize),
    pub drop_one: unsafe fn(*mut u8),
    pub copy: unsafe fn(*const u8, *mut u8, usize),
    pub copy_one: unsafe fn(*const u8, *mut u8),
}

impl ComponentInfo {
    pub fn of<T>() -> Self
    where
        T: Component,
    {
        ComponentInfo {
            id: TypeId::of::<T>(),
            layout: Layout::new::<T>(),
            debug_name: type_name::<T>(),
            drop: |ptr, count| unsafe {
                drop_in_place::<[T]>(slice_from_raw_parts_mut(ptr.cast(), count))
            },
            copy: |src, dst, count| unsafe {
                copy_nonoverlapping(src as *const T, dst as *mut T, count)
            },
            drop_one: |ptr| unsafe { drop_in_place::<T>(ptr.cast()) },
            copy_one: |src, dst| unsafe { copy_nonoverlapping(src as *const T, dst as *mut T, 1) },
        }
    }
}
