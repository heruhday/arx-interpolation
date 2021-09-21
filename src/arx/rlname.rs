use crate::arx::resbuf::rl_name;
use std::marker::PhantomData;
use crate::error::ArxError;
use crate::arx::acdb::object_id::{ObjectId};
use std::alloc::{Layout, alloc_zeroed};
use crate::arx::acdb;
use crate::arx::acad_app::error_status::eOk;
use crate::error;


#[derive(Clone, Debug)]
pub enum EntityKind {
    Ename(rl_name),
    SSname(rl_name)
}


#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct RlName<'a> {
    internal: rl_name,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> RlName<'a> {
    pub fn from_raw(ptr: rl_name) -> error::Result<Self> {
        if ptr.is_null() {
            return Err(Box::new(ArxError::ENameErr("rlname null")));
        }
        Ok(RlName {
            internal: ptr,
            lifetime: PhantomData,
        })
    }
    pub fn new() -> Self {
        let layout = Layout::new::<[i64; 2]>();
        let ptr =  unsafe {alloc_zeroed(layout) as *mut i64};
        RlName {
            internal: ptr,
            lifetime: PhantomData,
        }
    }

    pub fn into_inner(self) -> rl_name {
        self.internal
    }

    /// Creates a new null object
    pub fn null() -> RlName<'a> {
        (::std::ptr::null_mut() as rl_name).into()
    }

    pub fn get_object_id(self) -> error::Result<ObjectId>{
        let mut obj_id = ObjectId {mId: std::ptr::null_mut()};
        let ename = self.into_inner();
        let rs = unsafe{acdb::get_object_id(&mut obj_id, ename)};
        if rs != eOk {
            return Err(Box::new(ArxError::ObjectIdErr("get_object_id")));
        }
        Ok(obj_id)
    }
}

impl<'a> std::ops::Deref for RlName<'a> {
    type Target = rl_name;
    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a> From<rl_name> for RlName<'a>  {
    fn from(other: rl_name) -> Self {
        RlName {
            internal: other,
            lifetime: PhantomData,
        }
    }
}
