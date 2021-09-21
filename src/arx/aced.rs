use crate::arx::*;
use std::os::raw::c_int;
use crate::arx::resbuf::_resbuf;
use crate::arx::adesk::Int32;


extern "C" {
    #[link_name = "\u{1}?acedGetAppName@@YAPEB_WXZ"]
    pub fn get_appname() -> *const ACHAR;
}
extern "C" {
    #[link_name = "\u{1}?acedUpdate@@YAHHQEAN0@Z"]
    pub fn update(
        vport: c_int,
        p1: *mut ads_real,
        p2: *mut ads_real,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedRegFunc@@YAHP6AHXZH@Z"]
    pub fn reg_func(
        fhdl: ::std::option::Option<unsafe extern "C" fn() -> c_int>,
        fcode: c_int,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedUsrBrk@@YAHXZ"]
    pub fn UsrBrk() -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedDefun@@YAHPEB_WH@Z"]
    pub fn defun(
        pszName: *const ACHAR,
        nFuncNum: c_int,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedDefunEx@@YAHPEB_W0H@Z"]
    pub fn defun_ex(
        pszGlobalName: *const ACHAR,
        pszLocalName: *const ACHAR,
        nFuncNum: c_int,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSetFunHelp@@YAHPEB_W00H@Z"]
    pub fn set_fun_help(
        pszFunctionName: *const ACHAR,
        pszHelpfile: *const ACHAR,
        pszTopic: *const ACHAR,
        iCmd: c_int,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedUndef@@YAHPEB_WH@Z"]
    pub fn undef(sname: *const ACHAR, nFuncNum: c_int)
                      -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGetFunCode@@YAHXZ"]
    pub fn get_fun_code() -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGetArgs@@YAPEAUresbuf@@XZ"]
    pub fn get_args() -> *mut _resbuf;
}
extern "C" {
    #[link_name = "\u{1}?acedRetList@@YAHPEBUresbuf@@@Z"]
    pub fn ret_list(rbuf: *const _resbuf) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedRetVal@@YAHPEBUresbuf@@@Z"]
    pub fn ret_val(rbuf: *const _resbuf) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedRetPoint@@YAHQEBN@Z"]
    pub fn ret_point(pt: *mut ads_real) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedRetStr@@YAHPEB_W@Z"]
    pub fn ret_str(s: *const ACHAR) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedRetName@@YAHQEB_JH@Z"]
    pub fn ret_name(aname: *mut i64, type_: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedRetInt@@YAHH@Z"]
    pub fn ret_int(ival: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedRetReal@@YAHN@Z"]
    pub fn ret_real(rval: ads_real) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedRetT@@YAHXZ"]
    pub fn ret_T() -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedRetNil@@YAHXZ"]
    pub fn ret_nil() -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedRetVoid@@YAHXZ"]
    pub fn ret_void() -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedEntSel@@YAHPEB_WQEA_JQEAN@Z"]
    pub fn ent_sel(
        str_: *const ACHAR,
        entres: *mut i64,
        ptres: *mut ads_real,
    ) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AcSelectionPreview {
    _unused: [u8; 0],
}
extern "C" {
    #[link_name = "\u{1}?acedEntSel@@YAHPEB_WQEA_JQEANPEAVAcSelectionPreview@@@Z"]
    pub fn ent_sel1(
        str_: *const ACHAR,
        entres: *mut i64,
        ptres: *mut ads_real,
        pSelectionPreview: *mut AcSelectionPreview,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedNEntSel@@YAHPEB_WQEA_JQEANQEAY02NPEAPEAUresbuf@@@Z"]
    pub fn nent_sel(
        str_: *const ACHAR,
        entres: *mut i64,
        ptres: *mut ads_real,
        xformres: *mut ads_point,
        refstkres: *mut *mut _resbuf,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedNEntSelP@@YAHPEB_WQEA_JQEANHQEAY03NPEAPEAUresbuf@@@Z"]
    pub fn nent_sel_p(
        str_: *const ACHAR,
        entres: *mut i64,
        ptres: *mut ads_real,
        pickflag: c_int,
        xformres: *mut [ads_real; 4usize],
        refstkres: *mut *mut _resbuf,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSSGet@@YAHPEB_WPEBX1PEBUresbuf@@QEA_J@Z"]
    pub fn ss_get(
        str_: *const ACHAR,
        pt1: *const c_void,
        pt2: *const c_void,
        filter: *const _resbuf,
        ss: *mut i64,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSSGet@@YAHPEB_WPEBX1PEBUresbuf@@QEA_JPEAVAcSelectionPreview@@@Z"]
    pub fn ss_get1(
        str_: *const ACHAR,
        pt1: *const c_void,
        pt2: *const c_void,
        filter: *const _resbuf,
        ss: *mut i64,
        pSelectionPreview: *mut AcSelectionPreview,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSSGetFirst@@YAHPEAPEAUresbuf@@0@Z"]
    pub fn ss_get_first(gset: *mut *mut _resbuf, pset: *mut *mut _resbuf) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSSSetFirst@@YAHQEB_J0@Z"]
    pub fn ss_set_first(pset: *mut i64, unused: *mut i64) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSSFree@@YAHQEB_J@Z"]
    pub fn ss_free(sname: *mut i64) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSSLength@@YAHQEB_JPEAH@Z"]
    pub fn ss_length(sname: *mut i64, len: *mut Int32) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSSAdd@@YAHQEB_J0QEA_J@Z"]
    pub fn ss_add(ename: *mut i64, sname: *mut i64, result: *mut i64) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSSDel@@YAHQEB_J0@Z"]
    pub fn ss_del(ename: *mut i64, ss: *mut i64) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSSMemb@@YAHQEB_J0@Z"]
    pub fn ss_memb(ename: *mut i64, ss: *mut i64) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSSName@@YAHQEB_JHQEA_J@Z"]
    pub fn ss_name(
        ss: *mut i64,
        i: c_int,
        entres: *mut i64,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSSNameX@@YAHPEAPEAUresbuf@@QEB_JH@Z"]
    pub fn ss_name_x
    (
        rbpp: *mut *mut _resbuf,
        ss: *mut i64,
        i: c_int,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSSNameXEx@@YAHPEAPEAUresbuf@@QEB_JHI@Z"]
    pub fn ss_name_xex(
        rbpp: *mut *mut _resbuf,
        ss: *mut i64,
        i: c_int,
        flags: c_uint,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSSGetKwordCallbackPtr@@YAHPEAP6APEAUresbuf@@PEB_W@Z@Z"]
    pub fn ss_get_kword_callback_ptr(
        pFunc: *mut ::std::option::Option<unsafe extern "C" fn(arg1: *const ACHAR) -> *mut _resbuf>,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSSSetKwordCallbackPtr@@YAHP6APEAUresbuf@@PEB_W@Z@Z"]
    pub fn ss_set_kword_callback_ptr(
        pFunc: ::std::option::Option<unsafe extern "C" fn(arg1: *const ACHAR) -> *mut _resbuf>,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSSGetOtherCallbackPtr@@YAHPEAP6APEAUresbuf@@PEB_W@Z@Z"]
    pub fn ss_get_other_callback_ptr(
        pFunc: *mut ::std::option::Option<unsafe extern "C" fn(arg1: *const ACHAR) -> *mut _resbuf>,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSSSetOtherCallbackPtr@@YAHP6APEAUresbuf@@PEB_W@Z@Z"]
    pub fn ss_set_other_callback_ptr(
        pFunc: ::std::option::Option<unsafe extern "C" fn(arg1: *const ACHAR) -> *mut _resbuf>,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedTrans@@YAHQEBNPEBUresbuf@@1HQEAN@Z"]
    pub fn trans(
        pt: *mut ads_real,
        from: *const _resbuf,
        to: *const _resbuf,
        disp: c_int,
        result: *mut ads_real,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSetVar@@YAHPEB_WPEBUresbuf@@@Z"]
    pub fn set_var(sym: *const ACHAR, val: *const _resbuf) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedInitGet@@YAHHPEB_W@Z"]
    pub fn init_get(val: c_int, kwl: *const ACHAR) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGetSym@@YAHPEB_WPEAPEAUresbuf@@@Z"]
    pub fn get_sym(sname: *const ACHAR, value: *mut *mut _resbuf) -> c_int;
}


extern "C" {
    #[link_name = "\u{1}?acedPutSym@@YAHPEB_WPEAUresbuf@@@Z"]
    pub fn put_sym(sname: *const ACHAR, value: *mut _resbuf) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedHelp@@YAHPEB_W0H@Z"]
    pub fn help(
        szFilename: *const ACHAR,
        szTopic: *const ACHAR,
        iCmd: c_int,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedHelpForExternal@@YAHPEB_W@Z"]
    pub fn help_for_external(pszFunctionName: *const ACHAR) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedFNSplit@@YAHPEB_WPEA_W_K1212@Z"]
    pub fn fn_split(
        pathToSplit: *const ACHAR,
        prebuf: *mut ACHAR,
        nPreBufLen: size_t,
        namebuf: *mut ACHAR,
        nNameBufLen: size_t,
        extbuf: *mut ACHAR,
        nExtBufLen: size_t,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedArxLoaded@@YAPEAUresbuf@@XZ"]
    pub fn arx_loaded() -> *mut _resbuf;
}
extern "C" {
    #[link_name = "\u{1}?acedArxLoad@@YAHPEB_W@Z"]
    pub fn arx_load(app: *const ACHAR) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedArxUnload@@YAHPEB_W@Z"]
    pub fn arx_unload(app: *const ACHAR) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedInvoke@@YAHPEBUresbuf@@PEAPEAU1@@Z"]
    pub fn invoke(args: *const _resbuf, result: *mut *mut _resbuf) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGetVar@@YAHPEB_WPEAUresbuf@@@Z"]
    pub fn get_var(sym: *const ACHAR, result: *mut _resbuf) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedFindFile@@YAHPEB_WPEA_W_K@Z"]
    pub fn find_file(
        fname: *const ACHAR,
        result: *mut ACHAR,
        nBufLen: size_t,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedFindTrustedFile@@YAHPEB_WPEA_W_K@Z"]
    pub fn find_trusted_file(
        fname: *const ACHAR,
        result: *mut ACHAR,
        nBufLen: size_t,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGetEnv@@YAHPEB_WPEA_W_K@Z"]
    pub fn get_env(sym: *const ACHAR, var: *mut ACHAR, nBufLen: size_t)
                        -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSetEnv@@YAHPEB_W0@Z"]
    pub fn set_env(sym: *const ACHAR, val: *const ACHAR) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGetCfg@@YAHPEB_WPEA_W_K@Z"]
    pub fn get_cfg(sym: *const ACHAR, var: *mut ACHAR, len: size_t) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSetCfg@@YAHPEB_W0@Z"]
    pub fn set_cfg(sym: *const ACHAR, val: *const ACHAR) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGetString@@YAHHPEB_WPEA_W_K@Z"]
    pub fn get_string(
        cronly: c_int,
        prompt: *const ACHAR,
        result: *mut ACHAR,
        bufsize: size_t,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedMenuCmd@@YAHPEB_W@Z"]
    pub fn menu_cmd(str_: *const ACHAR) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedPrompt@@YAHPEB_W@Z"]
    pub fn prompt(str_: *const ACHAR) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedAlert@@YAHPEB_W@Z"]
    pub fn alert(str_: *const ACHAR) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGetAngle@@YAHQEBNPEB_WPEAN@Z"]
    pub fn get_angle(
        pt: *mut ads_real,
        prompt: *const ACHAR,
        result: *mut ads_real,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGetCorner@@YAHQEBNPEB_WQEAN@Z"]
    pub fn get_corner(
        pt: *mut ads_real,
        prompt: *const ACHAR,
        result: *mut ads_real,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGetDist@@YAHQEBNPEB_WPEAN@Z"]
    pub fn get_dist(
        pt: *mut ads_real,
        prompt: *const ACHAR,
        result: *mut ads_real,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGetOrient@@YAHQEBNPEB_WPEAN@Z"]
    pub fn get_orient(
        pt: *mut ads_real,
        prompt: *const ACHAR,
        result: *mut ads_real,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGetPoint@@YAHQEBNPEB_WQEAN@Z"]
    pub fn get_point(
        pt: *mut ads_real,
        prompt: *const ACHAR,
        result: *mut ads_real,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGetInt@@YAHPEB_WPEAH@Z"]
    pub fn get_int(
        prompt: *const ACHAR,
        result: *mut c_int,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGetKword@@YAHPEB_WPEA_W_K@Z"]
    pub fn get_kword(
        prompt: *const ACHAR,
        result: *mut ACHAR,
        nBufLen: size_t,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGetReal@@YAHPEB_WPEAN@Z"]
    pub fn get_real(prompt: *const ACHAR, result: *mut ads_real) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGetInput@@YAHPEA_W_K@Z"]
    pub fn get_input(str_: *mut ACHAR, nBufLen: size_t) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedVports@@YAHPEAPEAUresbuf@@@Z"]
    pub fn vports(vlist: *mut *mut _resbuf) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedTextScr@@YAHXZ"]
    pub fn text_scr() -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGraphScr@@YAHXZ"]
    pub fn graph_scr() -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedTextPage@@YAHXZ"]
    pub fn text_page() -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedRedraw@@YAHQEB_JH@Z"]
    pub fn redraw(ent: *mut i64, mode: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedOsnap@@YAHQEBNPEB_WQEAN@Z"]
    pub fn osnap(
        pt: *mut ads_real,
        mode: *const ACHAR,
        result: *mut ads_real,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGrRead@@YAHHPEAHPEAUresbuf@@@Z"]
    pub fn gr_read(
        track: c_int,
        type_: *mut c_int,
        result: *mut _resbuf,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGrText@@YAHHPEB_WH@Z"]
    pub fn gr_text(
        box_: c_int,
        text: *const ACHAR,
        hl: c_int,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGrDraw@@YAHQEBN0HH@Z"]
    pub fn gr_draw(
        from: *mut ads_real,
        to: *mut ads_real,
        color: c_int,
        hl: c_int,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGrVecs@@YAHPEBUresbuf@@QEAY03N@Z"]
    pub fn gr_vecs(vlist: *const _resbuf, mat: *mut [ads_real; 4usize]) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedXformSS@@YAHQEB_JQEAY03N@Z"]
    pub fn xform_ss(ssname: *mut i64, genmat: *mut [ads_real; 4usize]) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedDragGen@@YAHQEB_JPEB_WHP6AHQEANQEAY03N@Z2@Z"]
    pub fn drag_gen(
        ss: *mut i64,
        pmt: *const ACHAR,
        cursor: c_int,
        scnf: ::std::option::Option<
            unsafe extern "C" fn(
                pt: *mut ads_real,
                mt: *mut [ads_real; 4usize],
            ) -> c_int,
        >,
        p: *mut ads_real,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedSetView@@YAHPEBUresbuf@@H@Z"]
    pub fn set_view(view: *const _resbuf, vport: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGetFileD@@YAHPEB_W00HPEAUresbuf@@@Z"]
    pub fn get_filed(
        title: *const ACHAR,
        defawlt: *const ACHAR,
        ext: *const ACHAR,
        flags: c_int,
        result: *mut _resbuf,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGetFileNavDialog@@YAHPEB_W000HPEAPEAUresbuf@@@Z"]
    pub fn get_file_nav_dialog(
        title: *const ACHAR,
        defawlt: *const ACHAR,
        ext: *const ACHAR,
        dlgname: *const ACHAR,
        flags: c_int,
        result: *mut *mut _resbuf,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedTextBox@@YAHPEBUresbuf@@QEAN1@Z"]
    pub fn textbox(
        args: *const _resbuf,
        pt1: *mut ads_real,
        pt2: *mut ads_real,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedTablet@@YAHPEBUresbuf@@PEAPEAU1@@Z"]
    pub fn tablet(args: *const _resbuf, result: *mut *mut _resbuf) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedGetCName@@YAHPEB_WPEAPEA_W@Z"]
    pub fn get_cname(cmd: *const ACHAR, result: *mut *mut ACHAR) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}?acedEatCommandThroat@@YAHXZ"]
    pub fn eat_command_throat() -> c_int;
}