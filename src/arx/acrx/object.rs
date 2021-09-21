use std::os::raw::c_void;
use crate::arx::acad_app::ErrorStatus;
use crate::arx::acrx::Class;
use crate::arx::acrx;

pub struct bindgen_vtable(c_void);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Object {
    pub vtable_: *const bindgen_vtable,
}

extern "C" {
    #[link_name = "\u{1}?desc@AcRxObject@@SAPEAVAcRxClass@@XZ"]
    pub fn desc() -> *mut Class;

    #[link_name = "\u{1}?queryX@AcRxObject@@QEBAPEAV1@PEBVAcRxClass@@@Z"]
    pub fn query_x(
        this: *const Object,
        protocol_class: *const Class,
    ) -> *mut Object;

    #[link_name = "\u{1}?x@AcRxObject@@QEBAPEAV1@PEBVAcRxClass@@@Z"]
    pub fn x(
        this: *const Object,
        protocol_class: *const Class,
    ) -> *mut Object;

    #[link_name = "\u{1}??0AcRxObject@@IEAA@XZ"]
    pub fn Object(this: *mut Object);

    #[link_name = "\u{1}??_DAcRxObject@@QEAAXXZ"]
    pub fn destructor(this: *mut Object);
    
    #[link_name = "\u{1}?isA@AcRxObject@@UEBAPEAVAcRxClass@@XZ"]
    pub fn is_a(this: *mut c_void) -> *mut Class;

    #[link_name = "\u{1}?clone@AcRxObject@@UEBAPEAV1@XZ"]
    pub fn clone(this: *mut c_void) -> *mut Object;

    #[link_name = "\u{1}?copyFrom@AcRxObject@@UEAA?AW4ErrorStatus@Acad@@PEBV1@@Z"]
    pub fn copy_from(
        this: *mut c_void,
        other: *const Object,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?isEqualTo@AcRxObject@@UEBA_NPEBV1@@Z"]
    pub fn is_equal_to(
        this: *mut c_void,
        other: *const Object,
    ) -> bool;

    #[link_name = "\u{1}?comparedTo@AcRxObject@@UEBA?AW4Ordering@AcRx@@PEBV1@@Z"]
    pub fn compared_to(
        this: *mut c_void,
        other: *const Object,
    ) -> acrx::Ordering;

    #[link_name = "\u{1}?subQueryX@AcRxObject@@MEBAPEAV1@PEBVAcRxClass@@@Z"]
    pub fn sub_query_x(
        this: *mut c_void,
        protocol_class: *const Class,
    ) -> *mut Object;
}

impl Object {
    #[inline]
    pub unsafe fn desc() -> *mut Class {
        desc()
    }
    #[inline]
    pub unsafe fn query_x(&self, protocol_class: *const Class) -> *mut Object {
        query_x(self, protocol_class)
    }
    #[inline]
    pub unsafe fn x(&self, protocol_class: *const Class) -> *mut Object {
        x(self, protocol_class)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        Object(bindgen_tmp.as_mut_ptr());
        bindgen_tmp.assume_init()
    }
}
