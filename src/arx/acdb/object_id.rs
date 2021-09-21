use crate::arx::acdb::{Stub, OpenMode};
use crate::arx::adesk::int_db_id;
use crate::arx::acdb::database::Database;
use crate::arx::acdb::handle::Handle;
use crate::arx::{acrx, acdb};
use std::ptr::null_mut;
use crate::arx::acad_app::error_status::eOk;
use crate::error::ArxError;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ObjectId {
    pub mId: *mut Stub,
}

extern "C" {
    #[link_name = "\u{1}?kNull@AcDbObjectId@@2V1@B"]
    pub static knull: ObjectId;
}
extern "C" {
    #[link_name = "\u{1}?isNull@AcDbObjectId@@QEBA_NXZ"]
    pub fn isnull(this: *const ObjectId) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?setNull@AcDbObjectId@@QEAAXXZ"]
    pub fn set_null(this: *mut ObjectId);
}
extern "C" {
    #[link_name = "\u{1}?asOldId@AcDbObjectId@@QEBA_JXZ"]
    pub fn as_old_id(this: *const ObjectId) -> int_db_id;
}
extern "C" {
    #[link_name = "\u{1}?setFromOldId@AcDbObjectId@@QEAAAEAV1@_J@Z"]
    pub fn set_from_old_id(
        this: *mut ObjectId,
        oldId: int_db_id,
    ) -> *mut ObjectId;
}
extern "C" {
    #[link_name = "\u{1}?isValid@AcDbObjectId@@QEBA_NXZ"]
    pub fn is_valid(this: *const ObjectId) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?isWellBehaved@AcDbObjectId@@QEBA_NXZ"]
    pub fn is_well_behaved(this: *const ObjectId) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?database@AcDbObjectId@@QEBAPEAVAcDbDatabase@@XZ"]
    pub fn database(this: *const ObjectId) -> *mut Database;
}
extern "C" {
    #[link_name = "\u{1}?originalDatabase@AcDbObjectId@@QEBAPEAVAcDbDatabase@@XZ"]
    pub fn original_database(this: *const ObjectId) -> *mut Database;
}
extern "C" {
    #[link_name = "\u{1}?convertToRedirectedId@AcDbObjectId@@QEAA_NXZ"]
    pub fn convert_to_redirected_id(this: *mut ObjectId) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?isErased@AcDbObjectId@@QEBA_NXZ"]
    pub fn is_erased(this: *const ObjectId) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?isEffectivelyErased@AcDbObjectId@@QEBA_NXZ"]
    pub fn AcDbObjectId_isEffectivelyErased(this: *const ObjectId) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?isResident@AcDbObjectId@@QEBA_NXZ"]
    pub fn AcDbObjectId_isResident(this: *const ObjectId) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?objectLeftOnDisk@AcDbObjectId@@QEBA_NXZ"]
    pub fn object_left_on_disk(this: *const ObjectId) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?handle@AcDbObjectId@@QEBA?AVAcDbHandle@@XZ"]
    pub fn handle(this: *const ObjectId) -> Handle;
}
extern "C" {
    #[link_name = "\u{1}?nonForwardedHandle@AcDbObjectId@@QEBA?AVAcDbHandle@@XZ"]
    pub fn non_forwarded_handle(this: *const ObjectId) -> Handle;
}
extern "C" {
    #[link_name = "\u{1}?objectClass@AcDbObjectId@@QEBAPEAVAcRxClass@@XZ"]
    pub fn object_class(this: *const ObjectId) -> *mut acrx::Class;
}
extern "C" {
    #[link_name = "\u{1}??0AcDbObjectId@@QEAA@XZ"]
    pub fn object_id(this: *mut ObjectId);
}
extern "C" {
    #[link_name = "\u{1}??0AcDbObjectId@@QEAA@PEBVAcDbStub@@@Z"]
    pub fn object_id1(this: *mut ObjectId, arg1: *const Stub);
}

impl ObjectId {
    #[inline]
    pub fn open_object(&self, mode: OpenMode) -> Result<*mut acdb::Object> {
        unsafe {
            let mut pobj = null_mut();
            let rs = acdb::open_object(&mut pobj, *self, mode, false, null_mut() );
            if rs == eOk{
                Ok(pobj)
            }else{
                return Err(Box::new(ArxError::ObjectIdErr("open object")));
            }
        }
    }
}