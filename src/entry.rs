#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use crate::arx::aced::{get_args, get_fun_code};
use crate::arx::acrx::app_msg_code::*;
use crate::arx::acrx::app_ret_code::*;
use crate::arx::acrx::{register_app_mdi_aware, unlock_application, AppMsgCode, AppRetCode};
use crate::arx::acut::rel_rb;
use crate::arx::resbuf::_resbuf;
use crate::arx::string::ToWideChars;
use crate::arx::{aced, RTERROR, RTNORM};
use crate::database::reg_appnames;
use crate::entry_functions::{add_command, remove_command};
use crate::tables::cmd::COMMAND_TABLE;
use crate::tables::func::FUNC_TABLE;
use crate::{error, qeprintln};
use std::ffi::c_void;
use std::os::raw::c_int;

pub struct CommandEntry<'a> {
    pub name: &'a str,
    pub func: fn(),
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FunEntry<'a> {
    pub func_name: &'a str,
    pub func: fn(rb: *mut _resbuf) -> error::Result<()>,
}

#[no_mangle]
pub extern "C" fn acrxEntryPoint(msg: AppMsgCode, app_id: *mut c_void) -> AppRetCode {
    unsafe {
        match msg {
            kInitAppMsg => {
                unlock_application(app_id);
                register_app_mdi_aware(app_id);
                init_app();
            }
            kInvkSubrMsg => {
                dofun();
            }
            kLoadDwgMsg => {
                reg_appnames(&["gapp", "ars", "str", "me", "info"]);
                func_load();
            }
            kUnloadAppMsg => {
                unload_app();
            }
            _ => {}
        }
        kRetOK
    }
}

pub fn init_app() {
    for i in 0..COMMAND_TABLE.len() {
        let cmd = COMMAND_TABLE[i].name;
        let fun = COMMAND_TABLE[i].func;
        add_command(
            "ASDK_QS_LIBS",
            cmd,
            &(cmd.to_string().to_uppercase()),
            1,
            fun,
        );
    }
}

pub unsafe fn dofun() -> c_int {
    let val = get_fun_code();
    if val < 0 || val >= FUNC_TABLE.len() as _ {
        return RTERROR;
    }
    let rb = get_args();
    let rs = (FUNC_TABLE[val as usize].func)(rb);
    rel_rb(rb);
    return match rs {
        Ok(_) => RTNORM,
        Err(e) => {
            qeprintln!(e.to_string());
            RTERROR
        }
    };
}

pub unsafe fn func_load() -> i32 {
    for i in 0..FUNC_TABLE.len() {
        let fname = FUNC_TABLE[i].func_name.to_mut_wide();
        let rs = aced::defun(fname, i as _);
        if rs == 0 {
            return RTERROR;
        }
    }
    RTNORM
}

use lazy_static::lazy_static;
use std::sync::Mutex;
lazy_static! {
    pub static ref MSG: Mutex<Option<()>> = Mutex::new(None);
}

pub fn unload_app() {
    remove_command("ASDK_QS_LIBS");
}
