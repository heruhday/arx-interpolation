#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use std::os::raw::*;

use crate::arx::acarray::Array;
use crate::arx::acdb::object_id::ObjectId;

pub mod string;
pub mod acrx;
pub mod acarray;
pub mod aclocale;
pub mod adesk;
pub mod aced;
pub mod acut;
pub mod resbuf;
pub mod rlname;
pub mod acdb;
pub mod macros;
pub mod acgi;
pub mod acad_app;
pub mod transaction;
pub mod ename;
pub mod object;
pub mod ssname;
pub mod xdata;
pub mod editor;
pub mod curve;

pub type size_t = c_ulonglong;
pub type int_least8_t = c_schar;
pub type int_least16_t = c_short;
pub type int_least32_t = c_int;
pub type int_least64_t = c_longlong;
pub type uint_least8_t = c_uchar;
pub type uint_least16_t = c_ushort;
pub type uint_least32_t = c_uint;
pub type uint_least64_t = c_ulonglong;
pub type int_fast8_t = c_schar;
pub type int_fast16_t = c_int;
pub type int_fast32_t = c_int;
pub type int_fast64_t = c_longlong;
pub type uint_fast8_t = c_uchar;
pub type uint_fast16_t = c_uint;
pub type uint_fast32_t = c_uint;
pub type uint_fast64_t = c_ulonglong;
pub type intmax_t = c_longlong;
pub type uintmax_t = c_ulonglong;
pub type ACHAR = u16;
pub type ads_real = f64;
pub type ads_point = [ads_real; 3usize];
pub type ads_name = [i64; 2usize];
pub type ads_namep = *mut i64;
pub type ads_matrix = [[ads_real; 4usize]; 4usize];
pub type ads_pointp = *mut ads_real;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GUID {
    _unused: [u8; 0],
}
pub type CLSID = _GUID;
pub type wchar_t = u16;
pub const RTNONE: i32 = 5000;
pub const RTREAL: i32 = 5001;
pub const RTPOINT: i32 = 5002;
pub const RTSHORT: i32 = 5003;
pub const RTANG: i32 = 5004;
pub const RTSTR: i32 = 5005;
pub const RTENAME: i32 = 5006;
pub const RTPICKS: i32 = 5007;
pub const RTORINT: i32 = 5008;
pub const RT3DPOINT: i32 = 5009;
pub const RTLONG: i32 = 5010;
pub const RTVOID: i32 = 5014;
pub const RTLB: i32 = 5016;
pub const RTLE: i32 = 5017;
pub const RTDOTE: i32 = 5018;
pub const RTNIL: i32 = 5019;
pub const RTDXF0: i32 = 5020;
pub const RTT: i32 = 5021;
pub const RTRESBUF: i32 = 5023;
pub const RTMODELESS: i32 = 5027;
pub const RTLONG_PTR: i32 = 5030;
pub const RTINT64: i32 = 5031;
pub const RTCOROUTINEINFO: i32 = 5035;
pub const RTNORM: i32 = 5100;
pub const RTERROR: i32 = -5001;
pub const RTCAN: i32 = -5002;
pub const RTREJ: i32 = -5003;
pub const RTFAIL: i32 = -5004;
pub const RTKWORD: i32 = -5005;


extern "C" {
    pub fn free(_Block: *mut u8);
    pub fn malloc(_Size: usize) -> *mut u8;
    pub fn memcpy(_Dst: *mut u8, _Src: *const u8, _Size: usize) -> *mut u8;
    pub fn memmove(_Dst: *mut u8, _Src: *const u8, _Size: usize) -> *mut u8;
    pub fn memset(_Dst: *mut u8, _Val: i32, _Size: usize) -> *mut u8;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct matrix3d {
    _unused: [u8; 0],
}

pub type AppNameChangeFuncPtr = ::std::option::Option<unsafe extern "C" fn(classObj: *const acrx::Class,newAppName: *mut *mut wchar_t,saveVer: i32)>;
pub type Adesk_LongPtr = i64;
pub type Adesk_ULongPtr = u64;
pub type Adesk_IntPtr = i64;
pub type Adesk_UIntPtr = u64;
pub type Adesk_GsMarker = Adesk_IntPtr;
pub type Point3dArray = Array<point3d>;
pub type Vector3dArray = Array<vector3d>;
pub type VoidPointerArray = Array<*mut c_void>;
pub type IntArray = Array<c_int>;
pub type IntPtrArray = Array<Adesk_IntPtr>;
pub type ObjectIdArray = Array<ObjectId>;