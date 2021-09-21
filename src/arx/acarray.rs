use std::os::raw::c_int;

#[repr(C)]
#[derive(Debug)]
pub struct Array<T> {
    pub mpArray: *mut T,
    pub mPhysicalLen: c_int,
    pub mLogicalLen: c_int,
    pub mGrowLen: c_int,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}