use crate::arx::acdb;
use std::marker::PhantomData;
use crate::error::ArxError;
use crate::arx::acdb::OpenMode;
use crate::arx::ename::EName;
use crate::transaction::Transaction;
use crate::arx::acad_app::error_status::eOk;
use crate::arx::xdata::XData;
use crate::arx::acdb::object::{set_xdata, xdata};
use crate::arx::string::ToWideChars;
use acdb::object;
use crate::error;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


pub trait TryOpenFrom<T>: Sized {
    fn try_open_from(value: T, open_mode: OpenMode) -> error::Result<Self>;
    fn trans_try_open_from(trans: &Transaction, value: T, open_mode: OpenMode) -> error::Result<Self>;
}

pub trait TryOpenFromRaw<T>: Sized {
    fn try_open_from_raw(value: T, open_mode: OpenMode) -> error::Result<Self>;
    fn trans_try_open_from_raw(trans: &Transaction, value: T, open_mode: OpenMode) -> error::Result<Self>;
}


#[repr(transparent)]
#[derive(Clone, Debug, Copy)]
pub struct Object<'a> {
    internal: *mut object::Object,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> Object<'a> {
    pub fn from_raw(ptr: *mut object::Object) -> error::Result<Self> {
        if ptr.is_null(){
            return Err(Box::new(ArxError::ObjectErr("null ptr")));
        }
        Ok(Object {
            internal: ptr,
            lifetime: PhantomData,
        })
    }

    pub fn into_inner(self) ->*mut object::Object {
        self.internal
    }

    pub fn get_byte_xdata(&self, appname: &str) -> error::Result<Vec<u8>> {
        let o = self.internal;
        let rs = unsafe {xdata(o as _, appname.to_wide())};
        let mut xd = XData::from_raw(rs)?;
        let _app = xd.get_appname()?;
        xd.next()?;
        let v = xd.get_byte_buffer()?;
        Ok(v)
    }

    pub fn set_byte_xdata(&mut self, appname: &str, data: &[u8]) -> error::Result<()> {
        unsafe {
            let o = self.internal;
            let xd = XData::from_bytes(appname, data);
            let res =  set_xdata(o as _ , xd.into_inner());

            if res != eOk{
                return Err(Box::new(ArxError::XDataErr("set byte xdata")));
            }
            Ok(())
        }
    }


    pub fn set_multi_byte_xdata(&mut self, appname: &[&str], data: &[&[u8]]) -> error::Result<()> {
        unsafe {
            let o = self.internal;
            let xd = XData::from_array_bytes(appname, data);
            let res =  set_xdata(o as _ , xd.into_inner());

            if res != eOk{
                return Err(Box::new(ArxError::ObjectErr("set multi byte xdata")));
            }
            Ok(())
        }
    }


    pub fn close(&self) ->Result<()>{
        unsafe {
            if object::close(self.internal) != eOk {
                return Err(Box::new(ArxError::ObjectErr("close object")));
            }
        }
        Ok(())

    }

}



impl<'a> std::ops::Deref for Object<'a> {
    type Target = *mut object::Object;
    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a> From<*mut object::Object> for Object<'a>  {
    fn from(other: *mut object::Object) -> Self {
        Object {
            internal: other,
            lifetime: PhantomData,
        }
    }
}


impl<'a> TryOpenFrom<EName<'a>> for Object<'a> {
    fn try_open_from(ename: EName, open_mode: OpenMode) -> error::Result<Self> {
        let oid = ename.get_object_id()?;
        let obj = oid.open_object(open_mode)?;
        Ok(Object {
            internal: obj,
            lifetime: PhantomData,
        })
    }

    fn trans_try_open_from(trans: &Transaction, ename: EName<'a>, open_mode: OpenMode) -> error::Result<Self> {
        let oid = ename.get_object_id()?;
        let obj = (*trans).get_object(oid, open_mode, false)?;
        Ok(Object {
            internal: obj,
            lifetime: PhantomData,
        })
    }
}

impl<'a> TryOpenFromRaw<*mut i64> for Object<'a> {
    fn try_open_from_raw(e: *mut i64, open_mode: OpenMode) -> error::Result<Self> {
        let oid = EName::from_raw(e)?.get_object_id()?;
        let obj = oid.open_object(open_mode)?;
        Ok(Object {
            internal: obj,
            lifetime: PhantomData,
        })
    }

    fn trans_try_open_from_raw(trans: &Transaction, e: *mut i64, open_mode: OpenMode) -> error::Result<Self> {
        let oid = EName::from_raw(e)?.get_object_id()?;

        let obj = (*trans).get_object(oid, open_mode, false)?;
        Ok(Object {
            internal: obj,
            lifetime: PhantomData,
        })
    }
}


