use crate::error::ArxError;
use crate::arx::{ acdb};
use std::marker::PhantomData;
use crate::arx::resbuf::{ rl_name};
use std::alloc::{Layout, alloc_zeroed};
use std::ptr::null_mut;
use crate::arx::acdb::object_id::ObjectId;
use crate::arx::acad_app::error_status::eOk;
use crate::arx::acad_app::ErrorStatus;
use crate::error;


extern "C" {
    pub fn _get_length(ent: *mut i64, length: *mut f64) -> ErrorStatus;
    pub fn _get_area(ent: *mut i64, area: *mut f64) -> ErrorStatus;
}


#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct EName<'a> {
    internal: rl_name,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> EName<'a> {
    pub fn from_raw(ptr: rl_name) -> error::Result<Self> {
        if ptr.is_null(){
            return Err(Box::new(ArxError::ENameErr("null ptr")));
        }
        Ok(EName {
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
        EName {
            internal: ptr,
            lifetime: PhantomData,
        }
    }

    pub fn into_inner(self) -> rl_name {
        self.internal
    }

    pub fn null() -> EName<'a> {
        (::std::ptr::null_mut() as rl_name).into()
    }

    pub fn get_object_id(&self) -> error::Result<ObjectId> {
        let mut objid = ObjectId{ mId: null_mut()};
        let rs = unsafe {acdb::get_object_id(&mut objid, self.internal)};
        if rs == eOk {
            Ok(objid)
        }else {
            return Err(Box::new(ArxError::ENameErr("get object_id")));
        }

    }


}

impl<'a> std::ops::Deref for EName<'a> {
    type Target = rl_name;
    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a> From<rl_name> for EName<'a>  {
    fn from(other: rl_name) -> Self {
        EName {
            internal: other,
            lifetime: PhantomData,
        }
    }
}

