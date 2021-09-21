#![allow(dead_code)]
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![allow(improper_ctypes)]


use std::os::raw::c_int;

pub mod load_reasons {
    use crate::arx::acad_app::LoadReasons;

    pub const kOnProxyDetection: LoadReasons = 1;
    pub const kOnAutoCADStartup: LoadReasons = 2;
    pub const kOnCommandInvocation: LoadReasons = 4;
    pub const kOnLoadRequest: LoadReasons = 8;
    pub const kLoadDisabled: LoadReasons = 16;
    pub const kTransparentlyLoadable: LoadReasons = 32;
    pub const kOnIdleLoad: LoadReasons = 64;
}
pub type LoadReasons = c_int;

pub mod error_status {
    use crate::arx::acad_app::ErrorStatus;
    pub const eOk: ErrorStatus = 0;
    pub const eInvalidKey: ErrorStatus = 1;
    pub const eInvalidSubKey: ErrorStatus = 2;
    pub const eKeyNotFound: ErrorStatus = 3;
    pub const eOutOfMemory: ErrorStatus = 4;
    pub const eInvalidValue: ErrorStatus = 5;
    pub const eValueNotFound: ErrorStatus = 6;
    pub const eKeyExists: ErrorStatus = 7;
    pub const eRegAccessDenied: ErrorStatus = 8;
    pub const eRejected: ErrorStatus = 9;
    pub const eUnknownError: ErrorStatus = 10;   
}

pub type ErrorStatus = c_int;