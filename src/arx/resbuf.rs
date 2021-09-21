#![allow(unreachable_patterns)]

use std::os::raw::{c_short, c_void, c_ulonglong, c_uchar, c_int};
use crate::arx::{ads_real, ACHAR, RTNORM, acut, RTT, RTNIL};
use std::marker::PhantomData;
use std::ptr::null_mut;
use crate::arx::aced;
use crate::arx::string::ToWideChars;
use crate::error::ArxError;
use crate::arx::rlname::{RlName, EntityKind};
use crate::arx::string::ToString;
use crate::{arx, error};
use crate::qprintln;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _resbuf {
    pub rbnext: *mut _resbuf,
    pub restype: c_short,
    pub resval: ads_u_val,
}
pub type resbuf = *mut _resbuf;
pub type pResbuf = *mut _resbuf;
pub type kpResbuf = *const _resbuf;
pub type std_nullptr_t = *const c_void;
pub type std_align_val_t = c_ulonglong;
pub type rl_name = *mut i64;

#[repr(C)]
#[derive(Copy, Clone)]
pub union ads_u_val {
    pub rreal: ads_real,
    pub rpoint: [ads_real; 3usize],
    pub rint: c_short,
    pub rstring: *mut ACHAR,
    pub rlname: [i64; 2usize],
    pub mnLongPtr: i64,
    pub rlong: i32,
    pub mnInt64: i64,
    pub rbinary: ads_binary,
    pub ihandle: [c_uchar; 8usize],
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ads_binary {
    pub clen: c_short,
    pub buf: *mut c_uchar,
}



#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Resbuf<'a> {
    internal: *mut _resbuf,
    lifetime: PhantomData<&'a ()>,
}


impl<'a> Resbuf<'a> {
    pub fn from_raw(ptr: *mut _resbuf) -> error::Result<Self> {
        if ptr.is_null() {
            return Err(Box::new(ArxError::ResbufErr("null resbuf")));
        }
        Ok(Resbuf {
            internal: ptr,
            lifetime: PhantomData,
        })
    }
    pub fn get_sym(s: &str) -> error::Result<Self> {
        unsafe {
            let ptr = null_mut();
            let rs = aced::get_sym(s.to_wide(), ptr);
            if rs != RTNORM {
                return Err(Box::new(ArxError::ResbufErr("get sym: not found")));
            }
            Ok(Resbuf {
                internal: *ptr,
                lifetime: PhantomData,
            })
        }
    }

    pub fn put_sym(&self, s: &str) -> error::Result<()> {
        unsafe {
            let ptr = self.internal;
            let rs = aced::put_sym(s.to_wide(), ptr);
            if rs != RTNORM {
                return Err(Box::new(ArxError::ResbufErr("put sym: failed")));
            }
        }
        Ok(())
    }

    pub fn next(&mut self) -> error::Result<()> {
        unsafe {
            if self.is_null() {
                return Err(Box::new(ArxError::ResbufErr("null resbuf")));
            }
            self.internal = (*self.internal).rbnext;
            Ok(())
        }
    }

    
    pub fn print_code(&mut self)  -> error::Result<()> {
        unsafe {
            if self.is_null() {
                return Err(Box::new(ArxError::ResbufErr("null resbuf")));
            }
            loop {
                let rb = self.internal;
                let rbtype = (*rb).restype as i32;
                unsafe {qprintln!("code:{}", rbtype);}
                self.internal = (*self.internal).rbnext;

                if self.internal.is_null(){
                    break;
                }
            }
            
            Ok(())
        }
    }
    pub fn get_codes(&mut self)  -> error::Result<resbuf> {
        unsafe {
            if self.is_null() {
                return Err(Box::new(ArxError::ResbufErr("null resbuf")));
            }
            let rbres = acut::new_rb_(arx::RTLB);
            let mut rb_cur = rbres;
 
            loop {
                let rb = self.internal;
                let rbtype = (*rb).restype as i32;
                unsafe {qprintln!("code:{}", rbtype);}
                self.internal = (*self.internal).rbnext;
                
                (*rb_cur).rbnext = acut::new_rb_(arx::RTSHORT);
                rb_cur = (*rb_cur).rbnext;
                (*rb_cur).resval.rint = rbtype as _;


                if self.internal.is_null(){
                    (*rb_cur).rbnext = acut::new_rb_(arx::RTLE);
                    break;
                }
            }
            
            Ok(rbres)
        }
    }

    pub fn get_points(&mut self) -> error::Result<Vec<(f64, f64)>> {
        unsafe{
            if self.is_null() {
                return Err(Box::new(ArxError::ResbufErr("null resbuf")));
            }
            let rb = self.internal;
            if ((*rb).restype as i32) != arx::RTLB {
                return Err(Box::new(ArxError::ResbufErr("error start points list, must be RTLB at start")));
            }
            self.internal = (*self.internal).rbnext;
            if self.internal.is_null(){
                return Err(Box::new(ArxError::ResbufErr("error end resbuf, must be RPOINT and RTLE at end")));
            }

            let mut v = vec![];
            loop {
                let rb = self.internal;
                let rbtype = (*rb).restype as i32;
                qprintln!("code: {}", rbtype);
                match rbtype {
                    arx::RTPOINT => {
                        let res = (*rb).resval.rpoint;
                        v.push((res[0], res[1]));
                        
                    }
                    arx::RTLE => {
                        self.internal = (*self.internal).rbnext;
                        break;
                    }
                    _ => {
                        return Err(Box::new(ArxError::ResbufErr("must RTPOINT, or RTLE")));
                    }
                }
                self.internal = (*self.internal).rbnext;
                if self.internal.is_null(){
                    return Err(Box::new(ArxError::ResbufErr("error end resbuf, must be RTLE at end")));
                }
                
                
            }
            
            Ok(v)
        }
        
    }

    pub fn new_real(code: c_int, v: f64) -> error::Result<Self> {
        unsafe {
            let ptr = acut::new_rb_(code);
            (*ptr).resval.rreal = v;
            Ok(Resbuf {
                internal: ptr,
                lifetime: PhantomData,
            })
        }
    }

    pub fn new_i16(code: c_int, v: i16) -> error::Result<Self> {
        unsafe {
            let ptr = acut::new_rb_(code);
            (*ptr).resval.rint = v;
            Ok(Resbuf {
                internal: ptr,
                lifetime: PhantomData,
            })
        }
    }

    pub fn new_i32(code: c_int, v: i32) -> error::Result<Self> {
        unsafe {
            let ptr = acut::new_rb_(code);
            (*ptr).resval.rlong = v;
            Ok(Resbuf {
                internal: ptr,
                lifetime: PhantomData,
            })
        }
    }

    pub fn new_i64(code: c_int, v: i64) -> error::Result<Self> {
        unsafe {
            let ptr = acut::new_rb_(code);
            (*ptr).resval.mnInt64 = v;
            Ok(Resbuf {
                internal: ptr,
                lifetime: PhantomData,
            })
        }
    }

    pub fn new_point(code: c_int, v: &[f64])-> error::Result<Self> {
        unsafe {
            let ptr = acut::new_rb_(code);
            (*ptr).resval.rpoint.clone_from_slice(v);
            Ok(Resbuf {
                internal: ptr,
                lifetime: PhantomData,
            })
        }
    }

    pub fn new_ename(code: c_int, v: *const i64)-> error::Result<Self> {
        unsafe {
            let ptr = acut::new_rb_(code);
            (*ptr).resval.rlname.clone_from_slice(std::slice::from_raw_parts(v, 2));
            Ok(Resbuf {
                internal: ptr,
                lifetime: PhantomData,
            })
        }
    }

    pub fn new_string(code: c_int, v: &str) -> error::Result<Self> {
        unsafe {
            let ptr = acut::new_rb_(code);
            (*ptr).resval.rstring = v.to_mut_wide();
            Ok(Resbuf {
                internal: ptr,
                lifetime: PhantomData,
            })
        }
    }

    pub fn new_T() -> error::Result<Self> {
        unsafe {
            let ptr = acut::new_rb_(RTT);
            Ok(Resbuf {
                internal: ptr,
                lifetime: PhantomData,
            })
        }
    }

    pub fn new_Nil() ->  error::Result<Self> {
        unsafe {
            let ptr = acut::new_rb_(RTNIL);
            Ok(Resbuf {
                internal: ptr,
                lifetime: PhantomData,
            })
        }
    }

    //////////////////////////////
    pub fn get_real(&self) -> error::Result<f64> {
        unsafe {
            let rb = self.internal;
            let rbtype = (*rb).restype as i32;
            let res;
            match rbtype {
                arx::RTREAL=> {
                    res = (*rb).resval.rreal;
                }
                _ => {
                    return Err(Box::new(ArxError::InvalidResType("must be real number")));
                }
            }
            Ok(res)
        }
    }

    pub fn get_int(&self) -> error::Result<i16> {
        unsafe {
            let rb = self.internal;
            let rbtype = (*rb).restype as i32;
            let res;
            match rbtype {
                arx::RTSHORT=> {
                    res = (*rb).resval.rint;
                }
                _ => {
                    return Err(Box::new(ArxError::InvalidResType("must be a i16")));
                }
            }
            Ok(res)
        }
    }

    pub fn get_long(&self) -> error::Result<i32> {
        unsafe {
            let rb = self.internal;
            let rbtype = (*rb).restype as i32;
            let res;
            match rbtype {
                arx::RTLONG => {
                    res = (*rb).resval.rlong;
                }
                _ => {
                    return Err(Box::new(ArxError::InvalidResType("must be a i32")));
                }
            }
            Ok(res)
        }
    }

    pub fn get_integer(&self) -> error::Result<i32> {
        unsafe {
            let rb = self.internal;
            let rbtype = (*rb).restype as i32;
            let res;
            match rbtype {
                arx::RTSHORT=> {
                    res = (*rb).resval.rint as _;
                }

                arx::RTLONG => {
                    res = (*rb).resval.rlong as _;
                }
                arx::RTREAL=> {
                    res = (*rb).resval.rreal as _;
                }
                _ => {
                    return Err(Box::new(ArxError::InvalidResType("must be a i32")));
                }
            }
            Ok(res)
        }
    }

    pub fn get_double(&self) -> error::Result<f64> {
        unsafe {
            let rb = self.internal;
            let rbtype = (*rb).restype as i32;
            let res;
            match rbtype {
                arx::RTSHORT=> {
                    res = (*rb).resval.rint as _;
                }

                arx::RTLONG => {
                    res = (*rb).resval.rlong as _;
                }
                arx::RTREAL=> {
                    res = (*rb).resval.rreal as _;
                }
                _ => {
                    return Err(Box::new(ArxError::InvalidResType("must be a f64")));
                }
            }
            Ok(res)
        }
    }
    pub fn get_string(&self) -> error::Result<String> {
        unsafe {
            let rb = self.internal;
            let rbtype = (*rb).restype as i32;
            let res;
            match rbtype {
                arx::RTSTR=> {
                    res = (*rb).resval.rstring;
                }
                _ => {
                    return Err(Box::new(ArxError::InvalidResType("must be a string")));
                }
            }
            Ok(res.to_string())
        }
    }

    pub fn get_true(&self) -> error::Result<bool> {
        unsafe {
            let rb = self.internal;
            let rbtype = (*rb).restype as i32;
            let res;
            match rbtype {
                arx::RTT=> {
                    res = true;
                }
                arx::RTNIL=> {
                    res = false;
                }
                _ => {
                    return Err(Box::new(ArxError::InvalidResType("must be a T or Nil")));
                }
            }
            Ok(res)
        }
    }

    pub fn get_nil(&self) -> error::Result<()> {
        unsafe {
            let rb = self.internal;
            let rbtype = (*rb).restype as i32;
            match rbtype {
                arx::RTNIL=> {
                    Ok(())
                }

                _ => {
                    return Err(Box::new(ArxError::InvalidResType("must be a T or Nil")));
                }
            }
        }
    }

    pub fn get_entity(&self) -> error::Result<RlName> {
        unsafe {
            let rb = self.internal;
            let rbtype = (*rb).restype as i32;
            let res;
            match rbtype {
                arx::RTENAME => {
                    res = RlName::from_raw((*rb).resval.rlname.as_mut_ptr() as rl_name)?;
                }
                _ => {
                    return Err(Box::new(ArxError::InvalidResType("must be an ename")));
                }
            }
            Ok(res)
        }
    }

    pub fn get_selection_set(&self) -> error::Result<RlName> {
        unsafe {
            let rb = self.internal;
            let rbtype = (*rb).restype as i32;
            let res;
            match rbtype {
                arx::RTPICKS=> {
                    res = RlName::from_raw((*rb).resval.rlname.as_mut_ptr() as rl_name)?;
                }
                _ => {
                    return Err(Box::new(ArxError::InvalidResType("must be an ename")));
                }
            }
            Ok(res)
        }
    }

    pub fn get_name(&self) -> error::Result<EntityKind> {
        unsafe {
            let rb = self.internal;
            let rbtype = (*rb).restype as i32;
            let res;
            match rbtype {
                arx::RTENAME=> {
                    let rs = RlName::from_raw((*rb).resval.rlname.as_mut_ptr() as rl_name)?;
                    res = EntityKind::Ename(rs.into_inner());

                }
                arx::RTPICKS=> {
                    let rs = RlName::from_raw((*rb).resval.rlname.as_mut_ptr() as rl_name)?;
                    res = EntityKind::SSname(rs.into_inner());
                }
                _ => {
                    return Err(Box::new(ArxError::InvalidResType("must be an ename")));
                }
            }
            Ok(res)
        }
    }

    pub fn into_inner(&self) -> *mut _resbuf {
        self.internal
    }

    /// Creates a new null object
    pub fn null() -> Resbuf<'a> {
        (::std::ptr::null_mut() as pResbuf).into()
    }

    pub fn set_back(&mut self, ptr: *mut _resbuf) -> error::Result<()>   {
        if ptr.is_null() {
            return Err(Box::new(ArxError::ResbufErr("null resbuf")));
        }
        self.internal= ptr;
        Ok(())
    }

 

}

impl<'a> std::ops::Deref for Resbuf<'a> {
    type Target = *mut _resbuf;
    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a> From<pResbuf> for Resbuf<'a> {
    fn from(other: pResbuf) -> Self {
        Resbuf {
            internal: other,
            lifetime: PhantomData,
        }
    }
}

impl<'a> Drop for Resbuf<'a> {
    fn drop(&mut self) {
        unsafe {acut::rel_rb(self.internal)};
    }
}

