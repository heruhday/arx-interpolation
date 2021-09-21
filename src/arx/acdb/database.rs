use crate::arx::acrx::Object;
use crate::arx::acdb::ImpDatabase;

#[repr(C)]
#[derive(Debug)]
pub struct Database {
    pub _base: Object,
    pub mpImpDb: *mut ImpDatabase,
}