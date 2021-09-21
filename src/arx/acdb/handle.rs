use std::os::raw::c_int;
use crate::arx::{ACHAR, size_t};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Handle {
    pub mLow: u32,
    pub mHigh: u32,
}


extern "C" {
    #[link_name = "\u{1}?getIntoAsciiBuffer@AcDbHandle@@QEBA_NPEA_W_K@Z"]
    pub fn get_into_ascii_buffer(
        this: *const Handle,
        pBuf: *mut ACHAR,
        nBufLen: size_t,
    ) -> bool;

    #[link_name = "\u{1}?isNull@AcDbHandle@@QEBA_NXZ"]
    pub fn is_null(this: *const Handle) -> bool;

    #[link_name = "\u{1}?setNull@AcDbHandle@@QEAAXXZ"]
    pub fn set_null(this: *mut Handle);

    #[link_name = "\u{1}?low@AcDbHandle@@QEBAIXZ"]
    pub fn low(this: *const Handle) -> u32;

    #[link_name = "\u{1}?high@AcDbHandle@@QEBAIXZ"]
    pub fn high(this: *const Handle) -> u32;

    #[link_name = "\u{1}?setLow@AcDbHandle@@QEAAXI@Z"]
    pub fn set_low(this: *mut Handle, low: u32);

    #[link_name = "\u{1}?setHigh@AcDbHandle@@QEAAXI@Z"]
    pub fn set_high(this: *mut Handle, high: u32);

    #[link_name = "\u{1}?isOne@AcDbHandle@@QEBA_NXZ"]
    pub fn is_one(this: *const Handle) -> bool;

    #[link_name = "\u{1}?increment@AcDbHandle@@QEAAXXZ"]
    pub fn increment(this: *mut Handle);

    #[link_name = "\u{1}?decrement@AcDbHandle@@QEAAXXZ"]
    pub fn decrement(this: *mut Handle);

    #[link_name = "\u{1}?slowOperatorPlus@AcDbHandle@@QEBA?AV1@AEBV1@@Z"]
    pub fn slow_operator_plus(
        this: *const Handle,
        arg1: *const Handle,
    ) -> Handle;

    #[link_name = "\u{1}?compare@AcDbHandle@@QEBAHAEBV1@@Z"]
    pub fn compare(
        this: *const Handle,
        arg1: *const Handle,
    ) -> c_int;

    #[link_name = "\u{1}?copyToOldType@AcDbHandle@@QEBAXQEAE@Z"]
    pub fn copy_to_old_type(this: *const Handle, hand: *mut u8);

    #[link_name = "\u{1}?copyFromOldType@AcDbHandle@@QEAAXQEBE@Z"]
    pub fn copy_from_old_type(this: *mut Handle, hand: *const u8);

    #[link_name = "\u{1}?getValueBytes@AcDbHandle@@QEBAXPEAE0@Z"]
    pub fn get_value_bytes(
        this: *const Handle,
        arg1: *mut u8,
        arg2: *mut u8,
    );

    #[link_name = "\u{1}?setValueBytes@AcDbHandle@@QEAAXEPEBE@Z"]
    pub fn set_value_bytes(
        this: *mut Handle,
        arg1: u8,
        arg2: *const u8,
    );

    #[link_name = "\u{1}?print@AcDbHandle@@QEBAXXZ"]
    pub fn print(this: *const Handle);

    #[link_name = "\u{1}?byte@AcDbHandle@@QEBAHI@Z"]
    pub fn byte(this: *const Handle, i: u32) -> c_int;

    #[link_name = "\u{1}?restZeros@AcDbHandle@@QEBA_NH@Z"]
    pub fn rest_zeros(this: *const Handle, i: c_int) -> bool;

    #[link_name = "\u{1}??0AcDbHandle@@QEAA@XZ"]
    pub fn handle(this: *mut Handle);

    #[link_name = "\u{1}??0AcDbHandle@@QEAA@HH@Z"]
    pub fn handle1(
        this: *mut Handle,
        lo: c_int,
        hi: c_int,
    );

    #[link_name = "\u{1}??0AcDbHandle@@QEAA@PEB_W@Z"]
    pub fn handle2(this: *mut Handle, arg1: *const ACHAR);

    #[link_name = "\u{1}??0AcDbHandle@@QEAA@_K@Z"]
    pub fn handle3(this: *mut Handle, arg1: u64);
}
impl Handle {
    #[inline]
    pub unsafe fn get_into_ascii_buffer(&self, pBuf: *mut ACHAR, nBufLen: size_t) -> bool {
        get_into_ascii_buffer(self, pBuf, nBufLen)
    }
    #[inline]
    pub unsafe fn is_null(&self) -> bool {
        is_null(self)
    }
    #[inline]
    pub unsafe fn set_null(&mut self) {
        set_null(self)
    }
    #[inline]
    pub unsafe fn low(&self) -> u32 {
        low(self)
    }
    #[inline]
    pub unsafe fn high(&self) -> u32 {
        high(self)
    }
    #[inline]
    pub unsafe fn set_low(&mut self, low: u32) {
        set_low(self, low)
    }
    #[inline]
    pub unsafe fn set_high(&mut self, high: u32) {
        set_high(self, high)
    }
    #[inline]
    pub unsafe fn is_one(&self) -> bool {
        is_one(self)
    }
    #[inline]
    pub unsafe fn increment(&mut self) {
        increment(self)
    }
    #[inline]
    pub unsafe fn decrement(&mut self) {
        decrement(self)
    }
    #[inline]
    pub unsafe fn slow_operator_plus(&self, arg1: *const Handle) -> Handle {
        slow_operator_plus(self, arg1)
    }
    #[inline]
    pub unsafe fn compare(&self, arg1: *const Handle) -> c_int {
        compare(self, arg1)
    }
    #[inline]
    pub unsafe fn copy_to_old_type(&self, hand: *mut u8) {
        copy_to_old_type(self, hand)
    }
    #[inline]
    pub unsafe fn copy_from_old_type(&mut self, hand: *const u8) {
        copy_from_old_type(self, hand)
    }
    #[inline]
    pub unsafe fn get_value_bytes(&self, arg1: *mut u8, arg2: *mut u8) {
        get_value_bytes(self, arg1, arg2)
    }
    #[inline]
    pub unsafe fn set_value_bytes(&mut self, arg1: u8, arg2: *const u8) {
        set_value_bytes(self, arg1, arg2)
    }
    #[inline]
    pub unsafe fn print(&self) {
        print(self)
    }
    #[inline]
    pub unsafe fn byte(&self, i: u32) -> c_int {
        byte(self, i)
    }
    #[inline]
    pub unsafe fn restZeros(&self, i: c_int) -> bool {
        rest_zeros(self, i)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        handle(bindgen_tmp.as_mut_ptr());
        bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(lo: c_int, hi: c_int) -> Self {
        let mut bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        handle1(bindgen_tmp.as_mut_ptr(), lo, hi);
        bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(arg1: *const ACHAR) -> Self {
        let mut bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        handle2(bindgen_tmp.as_mut_ptr(), arg1);
        bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new3(arg1: u64) -> Self {
        let mut bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        handle3(bindgen_tmp.as_mut_ptr(), arg1);
        bindgen_tmp.assume_init()
    }
}