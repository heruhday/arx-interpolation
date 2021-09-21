use std::marker::PhantomData;
use crate::error::ArxError;
use crate::arx::acad_app::ErrorStatus;
use crate::arx::{transaction};
use crate::arx::acdb::object_id::ObjectId;
use crate::arx::acdb::object::Object;
use crate::arx::acdb::OpenMode;
use crate::arx::acad_app::error_status::eOk;
use std::ptr::null_mut;
use crate::error;


#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Transaction<'a> {
    internal: *mut transaction::Transaction,
    lifetime: PhantomData<&'a ()>,
}

extern "C" {
    pub fn start_transaction() -> *mut transaction::Transaction;
    pub fn end_transaction() -> ErrorStatus;
    pub fn abort_transaction() -> ErrorStatus;
    pub fn transaction_get_object(this: *mut transaction::Transaction, obj: *mut *mut Object, id: ObjectId, mode: OpenMode, openErasedObj: bool) -> ErrorStatus;
}

impl<'a> Transaction<'a> {
    pub fn from_raw(ptr: *mut transaction::Transaction) -> error::Result<Self> {
        Ok(Transaction {
            internal: ptr,
            lifetime: PhantomData,
        })
    }
    pub fn start() -> Self {
        unsafe {
            let ptr = start_transaction();
            Transaction {
                internal: ptr,
                lifetime: PhantomData,
            }
        }
    }
    pub fn end() -> error::Result<()>{
        unsafe {
            let rs = end_transaction();
            if rs != eOk {
                return Err(Box::new(ArxError::TransactionErr("end")));
            }
        }
        Ok(())
    }
    pub fn abort() -> error::Result<()>{
        unsafe {
            let rs = abort_transaction();
            if rs != eOk {
                return Err(Box::new(ArxError::TransactionErr("end")));
            }
        }
        Ok(())
    }
    pub fn get_object (&self, id: ObjectId, mode: OpenMode, openErasedObj: bool) -> error::Result <*mut Object> {
        unsafe {
            let x = self.internal;
            let mut obj = null_mut();
            let res = transaction_get_object(x, &mut obj, id, mode, openErasedObj);
            if res != eOk {
                return Err(Box::new(ArxError::TransactionErr("get object")));
            }
            Ok(obj)
        }

    }
}


impl<'a> std::ops::Deref for Transaction<'a> {
    type Target = *mut transaction::Transaction;
    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a> From<*mut transaction::Transaction> for Transaction<'a>{
    fn from(other: *mut transaction::Transaction) -> Self {
        Transaction {
            internal: other,
            lifetime: PhantomData,
        }
    }
}
