use crate::arx::acrx;

#[repr(C)]
#[derive(Debug)]
pub struct Transaction {
    pub _base: acrx::Object,
}