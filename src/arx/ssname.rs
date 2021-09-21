use crate::arx::resbuf::{rl_name, resbuf};
use std::marker::PhantomData;
use crate::error::ArxError;
use std::alloc::{Layout, alloc_zeroed};
use crate::arx::rlname::RlName;
use crate::arx::{RTNORM, aced, malloc};
use crate::arx::string::ToWideChars;
use std::ptr::{null, null_mut};
use crate::arx::acdb::free;
use crate::error;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct SSName<'a> {
    internal: rl_name,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> SSName<'a> {
    pub fn from_raw(ptr: rl_name) -> error::Result<Self> {
        if ptr.is_null(){
            return Err(Box::new(ArxError::ENameErr("null ptr")));
        }
        Ok(SSName {
            internal: ptr,
            lifetime: PhantomData,
        })
    }

    pub fn to_string(&self) -> String {
        unsafe{
            let v = *(self.internal as *const u128);
            format!("{:0x}",v)
        }
    }

    pub fn new() -> Self {
        let layout = Layout::new::<[i64; 2]>();
        let ptr =  unsafe {alloc_zeroed(layout) as *mut i64};
        SSName {
            internal: ptr,
            lifetime: PhantomData,
        }
    }

    pub fn into_inner(self) -> rl_name {
        self.internal
    }

    pub fn null() -> SSName<'a> {
        (::std::ptr::null_mut() as rl_name).into()
    }


    pub fn set_first(&self, ename: rl_name) -> error::Result<()> {
        let rs = unsafe {aced::ss_add(ename, self.internal, self.internal)};
        if rs == RTNORM {
            Ok(())
        }else {
            return Err(Box::new(ArxError::SSNameErr("add")));
        }
    }

    pub fn add(&self, ename: rl_name) -> error::Result<()> {
        let rs = unsafe {aced::ss_add(ename, self.internal, self.internal)};
        if rs == RTNORM {
            Ok(())
        }else {
            return Err(Box::new(ArxError::SSNameErr("add")));
        }
    }

    pub fn del(&self, ename: rl_name) -> error::Result<()> {
        let rs = unsafe {aced::ss_del(ename, self.internal)};
        if rs == RTNORM {
            Ok(())
        }else {
            return Err(Box::new(ArxError::SSNameErr("del")));
        }
    }

    pub fn member(&self, ename: rl_name) -> error::Result<()> {
        let rs = unsafe {aced::ss_memb(ename, self.internal)};
        if rs == RTNORM {
            Ok(())
        }else {
            return Err(Box::new(ArxError::SSNameErr("member")));
        }
    }

    pub fn name(&self, i: usize) -> error::Result<rl_name> {
        let entres = RlName::new().into_inner();
        let ss = self.internal;
        let rs = unsafe {aced::ss_name(ss, i as _, entres)};
        if rs == RTNORM {
            Ok(entres)
        }else {
            return Err(Box::new(ArxError::SSNameErr("name")));
        }
    }


    pub fn free(&self) -> error::Result<()>{
        let rs = unsafe {aced::ss_free(self.internal)};
        if rs == RTNORM {
            Ok(())
        }else {
            return Err(Box::new(ArxError::SSNameErr("free")));
        }
    }

    pub fn len(&self)  -> error::Result<usize> {
        let mut sz = 0;
        let rs = unsafe {aced::ss_length(self.internal, &mut sz)};
        if rs == RTNORM {
            Ok(sz as _)
        }else {
            return Err(Box::new(ArxError::SSNameErr("len")));
        }
    }

    pub fn get(str: Option<&str>, pt1: Option<&[f64]>, pt2: Option<&[f64]>, filter: Option<resbuf>) -> error::Result<Self> {
        let _str = match str {
            Some(s) => s.to_wide(),
            None => null(),
        };

        let _pt1 = match pt1 {
            Some(pt1) => {
                pt1.as_ptr()
            },
            None => {
                null_mut()
            }};
        let _pt2 = match pt2 {
            Some(pt2) => {
                pt2.as_ptr()
            },
            None => {
                null_mut()
            }};
        let _filter = match filter {
            Some(fltr) => {
                fltr
            },
            None => {
                null_mut()
            }};

        let ss_ptr: *mut i64 = unsafe {malloc(16) as _};
        let rs  = unsafe {aced::ss_get(_str, _pt1 as _, _pt2 as _, _filter, ss_ptr)};
        if rs == RTNORM {
            Ok(SSName {
                internal: ss_ptr,
                lifetime: PhantomData,
            })
        }else {
            unsafe {free(ss_ptr as _)};
            return Err(Box::new(ArxError::SSNameErr("get")));
        }

    }


}

impl<'a> std::ops::Deref for SSName<'a> {
    type Target = rl_name;
    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a> From<rl_name> for SSName<'a>  {
    fn from(other: rl_name) -> Self {
        SSName {
            internal: other,
            lifetime: PhantomData,
        }
    }
}

