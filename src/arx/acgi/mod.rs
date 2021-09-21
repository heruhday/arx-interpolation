pub mod drawable;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Context {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DrawableTraits {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WorldDraw {
    _unused: [u8; 0],
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DrawableAccessory {
    pub _address: u8,
}