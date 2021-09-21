

use std::{mem, slice::*};
use std::{ffi::OsString, os::windows::prelude::*};

pub trait ToString {
    fn to_string(self) ->String;

}
impl ToString for *const u16 {
    fn to_string(self) ->String {
        unsafe {
            let len = (0..).position(|i| *self.offset(i) == 0).unwrap();
            let slice = from_raw_parts(self, len);
            OsString::from_wide(slice).to_string_lossy().into_owned()
        }
    }
}

impl ToString for *mut u16 {
    fn to_string(self) ->String {
        unsafe {
            let len = (0..).position(|i| *self.offset(i) == 0).unwrap();
            let slice = from_raw_parts(self, len);
            OsString::from_wide(slice).to_string_lossy().into_owned()
        }
    }
}

pub trait ToWideChars {
    fn  to_mut_wide(self) -> *mut u16;
    fn to_wide(self) -> *const u16;
}

impl ToWideChars  for &str {
    fn to_mut_wide(self) -> *mut u16 {
        let mut v = self.encode_utf16().chain(Some(0).into_iter()).collect::<Vec<u16>>();
        let ptr = v.as_mut_ptr();
        mem::forget(v);
        ptr
    }

    fn to_wide(self) -> *const u16 {
        let mut v = self.encode_utf16().chain(Some(0).into_iter()).collect::<Vec<u16>>();
        let ptr = v.as_mut_ptr();
        mem::forget(v);
        ptr
    }
}
