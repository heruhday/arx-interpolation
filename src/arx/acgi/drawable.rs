use crate::arx::acrx;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Drawable {
    pub _base: acrx::Object,
    pub m_pAccessory: *mut Accessory,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Accessory {
    pub _address: u8,
}