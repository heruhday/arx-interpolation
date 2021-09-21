use crate::arx::resbuf::resbuf;
use crate::arx::string::ToWideChars;
use crate::arx::{RTNORM, aced, malloc, acdb, ACHAR};
use crate::error::ArxError;
use std::ptr::{null, null_mut};
use crate::arx::string::ToString;
use crate::error;


pub fn put_sym(sym: &str, value: resbuf) -> error::Result<()>{
    unsafe {
        let rs = aced::put_sym(sym.to_wide(), value);
        if rs != RTNORM {
            return Err(Box::new(ArxError::EditorErr("put sym")));
        }
        Ok(())
    }
}

pub fn get_sym(sym: &str) -> error::Result<resbuf>{
    unsafe {
        let rb = null_mut();
        let rs = aced::get_sym(sym.to_wide(), rb);
        if rs != RTNORM {
            return Err(Box::new(ArxError::EditorErr("put sym")));
        }
        Ok(*rb)
    }
}

pub fn get_env(sym: &str) -> error::Result<String>{
    unsafe {
        let sz = 0;
        let val = malloc(256) as *mut ACHAR;
        let rs = aced::get_env(sym.to_wide(), val, sz);
        if rs != RTNORM {
            return Err(Box::new(ArxError::EditorErr("get environment")));
        }
        Ok(val.to_string())
    }
}

pub fn set_env(sym: &str, val: &str) -> error::Result<()>{
    unsafe {
        let rs = aced::set_env(sym.to_wide(), val.to_wide());
        if rs != RTNORM {
            return Err(Box::new(ArxError::EditorErr("set environment")));
        }
        Ok(())
    }
}

pub fn get_config(sym: &str) -> error::Result<String>{
    unsafe {
        let sz = 0;
        let val = malloc(256) as *mut ACHAR;
        let rs = aced::get_cfg(sym.to_wide(), val, sz);
        if rs != RTNORM {
            return Err(Box::new(ArxError::EditorErr("get config")));
        }
        Ok(val.to_string())
    }
}

pub fn set_config(sym: &str, val: &str) -> error::Result<()>{
    unsafe {
        let rs = aced::set_cfg(sym.to_wide(), val.to_wide());
        if rs != RTNORM {
            return Err(Box::new(ArxError::EditorErr("set config")));
        }
        Ok(())
    }
}

pub fn get_string(cronly: bool, prompt: Option<&str>) -> error::Result<String>{
    unsafe {
        let prompt = match prompt {
            Some(prompt) => prompt.to_wide(),
            None => null(),
        };
        let result: *mut ACHAR = malloc(2*512) as _;
        let sz =512;
        let rs = aced::get_string(cronly as _, prompt, result, sz);
        if rs != RTNORM {
            return Err(Box::new(ArxError::EditorErr("get string")));
        }
        Ok(result.to_string())
    }
}

pub fn menu_cmd(str: Option<&str>) -> error::Result<()>{
    unsafe {
        let str = match str {
            Some(str) => str.to_wide(),
            None => null(),
        };
        let rs = aced::menu_cmd(str);
        if rs != RTNORM {
            return Err(Box::new(ArxError::EditorErr("menu cmd")));
        }
        Ok(())
    }
}

pub fn prompt(str: Option<&str>) -> error::Result<()>{
    unsafe {
        let str = match str {
            Some(str) => str.to_wide(),
            None => null(),
        };
        let rs = aced::prompt(str);
        if rs != RTNORM {
            return Err(Box::new(ArxError::EditorErr("prompt")));
        }
        Ok(())
    }
}

pub fn alert(str: Option<&str>) -> error::Result<()>{
    unsafe {
        let str = match str {
            Some(str) => str.to_wide(),
            None => null(),
        };
        let rs = aced::alert(str);
        if rs != RTNORM {
            return Err(Box::new(ArxError::EditorErr("alert")));
        }
        Ok(())
    }
}

pub fn get_angle(pt: *mut f64, prompt: Option<&str>) -> error::Result<*mut f64>{
    unsafe {
        let prompt = match prompt {
            Some(prompt) => prompt.to_wide(),
            None => null(),
        };
        let result = null_mut();
        let rs = aced::get_angle(pt, prompt, result);
        if rs != RTNORM {
            return Err(Box::new(ArxError::EditorErr("get angle")));
        }
        Ok(result)
    }
}

pub fn get_corner(pt: *mut f64, prompt: Option<&str>) -> error::Result<*mut f64>{
    unsafe {
        let prompt = match prompt {
            Some(prompt) => prompt.to_wide(),
            None => null(),
        };
        let result = null_mut();
        let rs = aced::get_corner(pt, prompt, result);
        if rs != RTNORM {
            return Err(Box::new(ArxError::EditorErr("get corner")));
        }
        Ok(result)
    }
}

pub fn get_dist(pt: *mut f64, prompt: Option<&str>) -> error::Result<*mut f64>{
    unsafe {
        let prompt = match prompt {
            Some(prompt) => prompt.to_wide(),
            None => null(),
        };
        let result = null_mut();
        let rs = aced::get_dist(pt, prompt, result);
        if rs != RTNORM {
            return Err(Box::new(ArxError::EditorErr("get dist")));
        }
        Ok(result)
    }
}

pub fn get_point(pt: *mut f64, prompt: Option<&str>) -> error::Result<*mut f64>{
    unsafe {
        let prompt = match prompt {
            Some(prompt) => prompt.to_wide(),
            None => null(),
        };
        let result = null_mut();
        let rs = aced::get_point(pt, prompt, result);
        if rs != RTNORM {
            return Err(Box::new(ArxError::EditorErr("get point")));
        }
        Ok(result)
    }
}

pub fn get_kword(sym: &str) -> error::Result<String>{
    unsafe {
        let sz = 0;
        let val = malloc(256) as *mut ACHAR;
        let rs = aced::get_kword(sym.to_wide(), val, sz);
        if rs != RTNORM {
            return Err(Box::new(ArxError::EditorErr("get kword")));
        }
        Ok(val.to_string())
    }
}

pub fn get_int(prompt: Option<&str>) -> error::Result<i32>{
    unsafe {
        let prompt = match prompt {
            Some(prompt) => prompt.to_wide(),
            None => null(),
        };
        let mut result = 0;
        let rs = aced::get_int( prompt, &mut result);
        if rs != RTNORM {
            return Err(Box::new(ArxError::EditorErr("get int")));
        }
        Ok(result)
    }
}

pub fn get_real(prompt: Option<&str>) -> error::Result<f64>{
    unsafe {
        let prompt = match prompt {
            Some(prompt) => prompt.to_wide(),
            None => null(),
        };
        let mut result = 0f64;
        let rs = aced::get_real( prompt, &mut result);
        if rs != RTNORM {
            return Err(Box::new(ArxError::EditorErr("get real")));
        }
        Ok(result)
    }
}

pub fn get_input() -> error::Result<String>{
    unsafe {

        let result: *mut ACHAR = malloc(2*512) as _;
        let sz =512;
        let rs = aced::get_input(result, sz);
        if rs != RTNORM {
            return Err(Box::new(ArxError::EditorErr("get input")));
        }
        Ok(result.to_string())
    }
}

pub fn reg_appname(appname: &str) {
    unsafe {
        acdb::reg_app(appname.to_wide());
    }
}

pub fn reg_appnames(appnames: &[&str]) {
    unsafe {
        for &app in appnames {
            acdb::reg_app(app.to_wide());
        }

    }
}