use std::os::raw::c_int;
use crate::arx::string::ToWideChars;
use crate::arx::acdb::reg_app;


pub fn RegApp(appname: &str) -> c_int {
    unsafe{reg_app(appname.to_wide())}
}



#[macro_export]
macro_rules! qprintln {
    ($a:expr) => {
        crate::arx::acut::printf(format!("{}\n", $a).to_wide());
    };
    ($a:expr, $($arg:tt)*) => {
        crate::arx::acut::printf(format!("{}\n", format!($a,$($arg)*)).to_wide());
    };
}

#[macro_export]
macro_rules! qprint {
    ($a:expr) => {
        crate::arx::acut::printf(format!("{}", $a).to_wide());
    };
    ($a:expr, $($arg:tt)*) => {
        crate::arx::acut::printf(format!($a,$($arg)*).to_wide());
    };
}


#[macro_export]
macro_rules! qeprint {
    ($a:expr) => {
        crate::arx::acdb::fail(format!("{}", $a).to_wide());
    };
    ($a:expr, $($arg:tt)*) => {
        crate::arx::acdb::fail(format!($a,$($arg)*).to_wide());
    };
}
#[macro_export]
macro_rules! qeprintln {
    ($a:expr) => {
        crate::arx::acdb::fail(format!("{}\n", $a).to_wide());
    };
    ($a:expr, $($arg:tt)*) => {
        crate::arx::acdb::fail(format!("{}\n", format!($a,$($arg)*)).to_wide());
    };
}

#[macro_export]
macro_rules! wrap_noise {
    ($wrapper_name:ident, $t:ty, $alg:expr $(, $arg:ident)*) => {
        #[no_mangle]
        pub unsafe fn $wrapper_name(seed: *mut noise::Seed $(, $arg: $t)*) -> $t {
            let seed: &noise::Seed = &*seed;
            $alg(&seed, &[$($arg),*])
        }
    };
}