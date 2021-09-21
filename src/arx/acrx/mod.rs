
#![allow(dead_code)]
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![allow(improper_ctypes)]

pub mod object;

use std::ffi::c_void;
use std::os::raw::c_int;

use crate::arx::ACHAR;
use crate::arx::acarray::Array;
use crate::arx::aclocale::AcLocale;
use crate::arx::acad_app::{LoadReasons, ErrorStatus};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AcRx {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct Class {
    pub _base: Object,
    pub m_pImp: *mut ImpClass,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImpClass {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Object {
    pub vtable_: *const bindgen_vtable,
}
#[repr(C)]
pub struct bindgen_vtable(c_void);

pub type FcnPtr = ::std::option::Option<unsafe extern "C" fn()>;
pub type DictIterType = c_int;
pub type Ordering = c_int;
pub type AppMsgCode = c_int;
pub type AppRetCode = c_int;

pub mod dict_iter_type{
    use crate::arx::acrx::DictIterType;
    pub const kDictSorted: DictIterType = 0;
    pub const kDictCollated: DictIterType = 1;
}
pub mod ordering {
    use crate::arx::acrx::Ordering;
    pub const kLessThan: Ordering = -1;
    pub const kEqual: Ordering = 0;
    pub const kGreaterThan: Ordering = 1;
    pub const kNotOrderable: Ordering = 2;
}
pub mod app_msg_code {
    use crate::arx::acrx::AppMsgCode;

    pub const kNullMsg: AppMsgCode = 0;
    pub const kInitAppMsg: AppMsgCode = 1;
    pub const kUnloadAppMsg: AppMsgCode = 2;
    pub const kLoadDwgMsg: AppMsgCode = 3;
    pub const kUnloadDwgMsg: AppMsgCode = 4;
    pub const kInvkSubrMsg: AppMsgCode = 5;
    pub const kCfgMsg: AppMsgCode = 6;
    pub const kEndMsg: AppMsgCode = 7;
    pub const kQuitMsg: AppMsgCode = 8;
    pub const kSaveMsg: AppMsgCode = 9;
    pub const kDependencyMsg: AppMsgCode = 10;
    pub const kNoDependencyMsg: AppMsgCode = 11;
    pub const kOleUnloadAppMsg: AppMsgCode = 12;
    pub const kPreQuitMsg: AppMsgCode = 13;
    pub const kInitDialogMsg: AppMsgCode = 14;
    pub const kEndDialogMsg: AppMsgCode = 15;
    pub const kSuspendMsg: AppMsgCode = 16;
    pub const kInitTabGroupMsg: AppMsgCode = 17;
    pub const kEndTabGroupMsg: AppMsgCode = 18;
}
pub mod app_ret_code{
    use crate::arx::acrx::AppRetCode;
    pub const kRetOK: AppRetCode = 0;
    pub const kRetError: AppRetCode = 3;
}


extern "C" {
    #[link_name = "\u{1}?acrxUnlockApplication@@YA_NPEAX@Z"]
    pub fn unlock_application(app_id: *mut c_void) -> bool;

    #[link_name = "\u{1}?acrxApplicationIsLocked@@YA_NPEB_W@Z"]
    pub fn application_is_locked(module_name: *const ACHAR) -> bool;

    #[link_name = "\u{1}?acrxLockApplication@@YA_NPEAX@Z"]
    pub fn lock_application(app_id: *mut c_void) -> bool;

    #[link_name = "\u{1}?acrxFindAcRxClass@@YAPEAVAcRxClass@@PEB_W@Z"]
    pub fn find_acrx_class(p_class_name: *const ACHAR) -> *mut Class;

    pub fn abort(format: *const ACHAR, ...);

    #[link_name = "\u{1}?acrxLoadModule@@YA_NPEB_W_N1@Z"]
    pub fn load_module(module_name: *const ACHAR, print_it: bool, as_cmd: bool) -> bool;

    #[link_name = "\u{1}?acrxLoadApp@@YA_NPEB_W_N@Z"]
    pub fn load_app(appname: *const ACHAR, as_cmd: bool) -> bool;

    #[link_name = "\u{1}?acrxUnloadModule@@YA_NPEB_W_N@Z"]
    pub fn unload_module(module_name: *const ACHAR, as_cmd: bool) -> bool;

    #[link_name = "\u{1}?acrxUnloadApp@@YA_NPEB_W_N@Z"]
    pub fn unload_app(app_name: *const ACHAR, as_cmd: bool) -> bool;

    #[link_name = "\u{1}?acrxLoadedApps@@YAPEAV?$AcArray@PEA_WV?$AcArrayMemCopyReallocator@PEA_W@@@@XZ"]
    pub fn loaded_apps() -> *mut Array<*mut u16>;

    #[link_name = "\u{1}?acrxLoadAutoloadApps@@YA_NPEB_W@Z"]
    pub fn load_autoload_apps(appname: *const ACHAR) -> bool;

    #[link_name = "\u{1}?acrxUnloadAutoloadApps@@YA_NPEB_W@Z"]
    pub fn unload_autoload_apps(appname: *const ACHAR) -> bool;

    #[link_name = "\u{1}?acrxAppIsLoaded@@YA_NPEB_W@Z"]
    pub fn app_is_loaded(pAppName: *const ACHAR) -> bool;

    #[link_name = "\u{1}?acrxObjectDBXRegistryKey@@YAPEB_WXZ"]
    pub fn object_dbx_registry_key() -> *const ACHAR;

    #[link_name = "\u{1}?acrxProductLocale@@YA?AVAcLocale@@XZ"]
    pub fn product_locale() -> AcLocale;

    #[link_name = "\u{1}?acrxRegisterApp@@YA?AW4ErrorStatus@AcadApp@@W4LoadReasons@2@PEB_W@Z"]
    pub fn register_app(
        alr: LoadReasons,
        logicalName: *const ACHAR,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?acrxUnregisterApp@@YA?AW4ErrorStatus@AcadApp@@PEB_W@Z"]
    pub fn unregister_app(logicalname: *const ACHAR) -> ErrorStatus;

    #[link_name = "\u{1}?acrxGetServiceSymbolAddr@@YAPEAXPEB_W0@Z"]
    pub fn get_service_symbol_addr(
        serviceName: *const ACHAR,
        symbol: *const ACHAR,
    ) -> *mut c_void;

    #[link_name = "\u{1}?acrxRegisterService@@YAPEAXPEB_W@Z"]
    pub fn register_service(serviceName: *const ACHAR) -> *mut c_void;

    #[link_name = "\u{1}?acrxServiceIsRegistered@@YA_NPEB_W@Z"]
    pub fn service_is_registered(serviceName: *const ACHAR) -> bool;

    #[link_name = "\u{1}?acrxIsAppMDIAware@@YA_NPEB_W@Z"]
    pub fn is_app_mdi_aware(moduleName: *const ACHAR) -> bool;

    #[link_name = "\u{1}?acrxRegisterAppMDIAware@@YA_NPEAX@Z"]
    pub fn register_app_mdi_aware(appId: *mut c_void) -> bool;

    #[link_name = "\u{1}?acrxRegisterAppNotMDIAware@@YA_NPEAX@Z"]
    pub fn register_app_not_mdi_aware(appId: *mut c_void) -> bool;
}