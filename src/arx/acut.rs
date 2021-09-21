use crate::arx::{ads_real, ACHAR, size_t};
use std::os::raw::c_int;
use crate::arx::resbuf::_resbuf;
use crate::arx::acdb::DxfCode;

#[inline]

pub fn new_rb(code: DxfCode) -> *mut _resbuf{
    unsafe {new_rb_(code as _)}
}
extern "C" {
    #[link_name = "\u{1}?acutCvUnit@@YAHNPEB_W0PEAN@Z"]
    pub fn cv_unit(
        value: ads_real,
        oldunit: *const ACHAR,
        newunit: *const ACHAR,
        result: *mut ads_real,
    ) -> c_int;

    #[link_name = "\u{1}?acutWcMatch@@YAHPEB_W0@Z"]
    pub fn wc_match(string: *const ACHAR, pattern: *const ACHAR) -> c_int;

    #[link_name = "\u{1}?acutPrintf@@YAHPEB_WZZ"]
    pub fn printf(format: *const ACHAR, ...) -> c_int;

    #[link_name = "\u{1}?acutSPrintf@@YAHPEA_W_KPEB_WZZ"]
    pub fn sprintf(
        buffer: *mut ACHAR,
        nBufLen: size_t,
        format: *const ACHAR,
        ...
    ) -> c_int;

    #[link_name = "\u{1}?acutNewRb@@YAPEAUresbuf@@H@Z"]
    pub fn new_rb_(v: c_int) -> *mut _resbuf;


    #[link_name = "\u{1}?acutRelRb@@YAHPEAUresbuf@@@Z"]
    pub fn rel_rb(rb: *mut _resbuf) -> c_int;

    #[link_name = "\u{1}?acutBuildList@@YAPEAUresbuf@@HZZ"]
    pub fn build_list(rtype: c_int, ...) -> *mut _resbuf;

    #[link_name = "\u{1}?acutRbDup@@YAHPEBUresbuf@@PEAPEAU1@@Z"]
    pub fn rb_dup(source: *const _resbuf, result: *mut *mut _resbuf) -> c_int;

    #[link_name = "\u{1}?acutAngle@@YANQEBN0@Z"]
    pub fn angle(pt1: *mut ads_real, pt2: *mut ads_real) -> ads_real;

    #[link_name = "\u{1}?acutDistance@@YANQEBN0@Z"]
    pub fn distance(pt1: *mut ads_real, pt2: *mut ads_real) -> ads_real;

    #[link_name = "\u{1}?acutPolar@@YAXQEBNNNQEAN@Z"]
    pub fn polar(pt: *mut ads_real, angle: ads_real, dist: ads_real, ptres: *mut ads_real);

    #[link_name = "\u{1}?acutIsAlpha@@YAHH@Z"]
    pub fn is_alpha(c: c_int) -> c_int;

    #[link_name = "\u{1}?acutIsUpper@@YAHH@Z"]
    pub fn is_upper(c: c_int) -> c_int;

    #[link_name = "\u{1}?acutIsLower@@YAHH@Z"]
    pub fn is_lower(c: c_int) -> c_int;

    #[link_name = "\u{1}?acutIsDigit@@YAHH@Z"]
    pub fn is_digit(c: c_int) -> c_int;

    #[link_name = "\u{1}?acutIsXDigit@@YAHH@Z"]
    pub fn is_xdigit(c: c_int) -> c_int;

    #[link_name = "\u{1}?acutIsSpace@@YAHH@Z"]
    pub fn is_space(c: c_int) -> c_int;

    #[link_name = "\u{1}?acutIsPunct@@YAHH@Z"]
    pub fn is_punct(c: c_int) -> c_int;

    #[link_name = "\u{1}?acutIsAlNum@@YAHH@Z"]
    pub fn is_alnum(c: c_int) -> c_int;

    #[link_name = "\u{1}?acutIsPrint@@YAHH@Z"]
    pub fn is_print(c: c_int) -> c_int;

    #[link_name = "\u{1}?acutIsGraph@@YAHH@Z"]
    pub fn is_graph(c: c_int) -> c_int;

    #[link_name = "\u{1}?acutIsCntrl@@YAHH@Z"]
    pub fn is_cntrl(c: c_int) -> c_int;

    #[link_name = "\u{1}?acutToUpper@@YAHH@Z"]
    pub fn to_upper(c: c_int) -> c_int;

    #[link_name = "\u{1}?acutToLower@@YAHH@Z"]
    pub fn to_lower(c: c_int) -> c_int;
}