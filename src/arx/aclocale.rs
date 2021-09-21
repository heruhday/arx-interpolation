#[repr(C)]
#[derive(Debug)]
pub struct AcLocale {
    pub m_pImpAcLocale: *mut AcLocaleImp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AcLocaleImp {
    _unused: [u8; 0],
}