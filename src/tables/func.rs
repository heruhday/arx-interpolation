use crate::entry::{FunEntry,};

use crate::functions::{get_codes, get_msg, get_points, get_points2, print_code, set_int};

pub const FUNC_TABLE: &'static [FunEntry] =
    &[
        FunEntry {func_name: "set_int", func : set_int},
        FunEntry {func_name: "get_msg", func : get_msg},
        FunEntry {func_name: "print_code", func : print_code},
        FunEntry {func_name: "get_codes", func : get_codes},
        FunEntry {func_name: "get_points", func : get_points},
        FunEntry {func_name: "get_points2", func : get_points2},
    ];