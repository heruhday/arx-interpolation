#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![allow(improper_ctypes)]

use crate::arx::{ACHAR, wchar_t};
use crate::arx::string::ToWideChars;
use crate::arx::adesk::*;

pub type FunPtr = fn();

extern "C" {
    fn _add_command(
        cmd_group_name: *const ACHAR,
        cmd_global_name: *const ACHAR,
        cmd_local_name: *const ACHAR,
        command_flags: Int32,
        function_addr: FunPtr,
    );
    fn _remove_group(groupName: *const wchar_t);
}

#[inline]
pub fn add_command(cmd_group_name: &str, cmd_global_name: &str, cmd_local_name: &str, command_flags: i32, fun: FunPtr){
    unsafe {_add_command(cmd_group_name.to_wide(), cmd_global_name.to_wide(), cmd_local_name.to_wide(), command_flags, fun )};
}

#[inline]
pub fn remove_command(group_name: &str){
    unsafe {_remove_group(group_name.to_wide())};
}