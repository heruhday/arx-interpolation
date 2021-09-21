use std::marker::PhantomData;
use crate::arx::resbuf::resbuf;
use crate::arx::string::ToWideChars;
use crate::arx::{malloc, memcpy, acut};
use crate::arx::acdb::dxf_code::*;
use crate::error::ArxError;
use crate::arx::acdb::dxf_code;
use crate::arx::string::ToString;
use crate::error;


pub struct XData<'a> {
    head: resbuf,
    cur: resbuf,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> XData<'a> {

    pub fn from_array_bytes(app: &[&str], v: &[&[u8]] ) -> Self {
        unsafe{
            let ptr =  acut::new_rb(kDxfRegAppName);
            (*ptr).resval.rstring = app[0].to_mut_wide();
            let mut c_rb = ptr;

            for i in 1 .. app.len() {
                (*c_rb).rbnext =  acut::new_rb(kDxfRegAppName);
                c_rb = (*c_rb).rbnext;
                (*c_rb).resval.rstring = app[i].to_mut_wide();
                let vi = v[i];
                for chunk in vi.chunks(127){
                    (*c_rb).rbnext = acut::new_rb(kDxfXdBinaryChunk);
                    c_rb = (*c_rb).rbnext;
                    (*c_rb).resval.rbinary.buf = malloc(chunk.len() ) ;
                    memcpy( (*c_rb).resval.rbinary.buf ,  chunk.as_ptr() , chunk.len() );
                    (*c_rb).resval.rbinary.clen = chunk.len() as _;
                }
            }

            XData {
                head: ptr,
                cur: c_rb,
                lifetime: PhantomData,
            }

        }
    }

    pub fn from_bytes(app: &str, v: &[u8] ) -> Self {
        unsafe {
            let ptr =  acut::new_rb(kDxfRegAppName);
            (*ptr).resval.rstring = app.to_mut_wide();
            let mut c_rb = ptr;
            for chunk in v.chunks(127){
                (*c_rb).rbnext = acut::new_rb(kDxfXdBinaryChunk);
                c_rb = (*c_rb).rbnext;
                (*c_rb).resval.rbinary.buf = malloc(chunk.len() as _) as _;
                memcpy( (*c_rb).resval.rbinary.buf as _,  chunk.as_ptr() as _, chunk.len() as _);
                (*c_rb).resval.rbinary.clen = chunk.len() as _;
            }
            XData {
                head: ptr,
                cur: c_rb,
                lifetime: PhantomData,
            }
        }
    }

    pub fn from(app: &str) -> Self {
        unsafe {
            let ptr =  acut::new_rb(kDxfRegAppName);
            (*ptr).resval.rstring = app.to_mut_wide();
            XData {
                head: ptr,
                cur: ptr,
                lifetime: PhantomData,
            }
        }
    }

    pub fn append_appname(&mut self, v: &str) {
        unsafe {
            let mut ptr = self.cur;
            (*ptr).rbnext = acut::new_rb(kDxfRegAppName);
            (*ptr).resval.rstring = v.to_mut_wide();
            self.cur = ptr;
        }
    }

    pub fn append_string(&mut self, v: &str) {
        unsafe {
            let mut ptr = self.cur;
            (*ptr).rbnext = acut::new_rb(kDxfXdAsciiString);
            (*ptr).resval.rstring = v.to_mut_wide();
            self.cur = ptr;
        }
    }

    pub fn append_control_string(&mut self, v: &str)  {
        unsafe {
            let mut ptr = self.cur;
            (*ptr).rbnext = acut::new_rb(kDxfXdControlString);
            (*ptr).resval.rstring = v.to_mut_wide();
            self.cur = ptr;
        }
    }

    pub fn append_real(&mut self, v: f64) {
        unsafe {
            let mut ptr = self.cur;
            (*ptr).rbnext = acut::new_rb(kDxfXdReal);
            (*ptr).resval.rreal = v;
            self.cur = ptr;
        }
    }

    pub fn append_int(&mut self, v: i16) {
        unsafe {
            let mut ptr = self.cur;
            (*ptr).rbnext = acut::new_rb(kDxfXdInteger16);
            (*ptr).resval.rint = v;
            self.cur = ptr;
        }
    }

    pub fn append_long(&mut self, v: i32) {
        unsafe {
            let mut ptr = self.cur;
            (*ptr).rbnext = acut::new_rb(kDxfXdInteger32);
            (*ptr).resval.rlong = v;
            self.cur = ptr;
        }
    }

    pub fn append_bytes(&mut self, v: &[u8]) -> error::Result<()> {
        unsafe {
            let mut c_rb = self.cur;
            for chunk in v.chunks(127){
                (*c_rb).rbnext = acut::new_rb(kDxfXdBinaryChunk);
                c_rb = (*c_rb).rbnext;
                (*c_rb).resval.rbinary.buf = malloc(chunk.len() as _) as _;
                memcpy( (*c_rb).resval.rbinary.buf as _,  chunk.as_ptr() as _, chunk.len() as _);
                (*c_rb).resval.rbinary.clen = chunk.len() as _;
            }
            self.cur = c_rb;
            Ok(())
        }
    }

    pub fn from_raw(ptr: resbuf) -> error::Result<Self> {
        if ptr.is_null() {
            return Err(Box::new(ArxError::XDataErr("null resbuf")));
        }
        Ok(XData {
            head: ptr,
            cur: ptr,
            lifetime: PhantomData,
        })
    }

    pub fn next(&mut self) -> error::Result<()> {
        unsafe {
            if (*self.cur).rbnext.is_null() {
                return Err(Box::new(ArxError::XDataErr("null next resbuf")));
            }
            self.cur = (*self.cur).rbnext;
            Ok(())
        }
    }

    pub fn get_real(&self) -> error::Result<f64> {
        unsafe {
            let rb = self.cur;
            let rbtype = (*rb).restype;
            let res;

            match rbtype {
                dxf_code::kDxfXdReal => {
                    res = (*rb).resval.rreal;
                }
                _ => {
                    return Err(Box::new(ArxError::XDataErr("must be real number")));
                }
            }
            Ok(res)
        }
    }

    pub fn get_int(&self) -> error::Result<i16> {
        unsafe {
            let rb = self.cur;
            let rbtype = (*rb).restype;
            let res;
            match rbtype {
                kDxfXdInteger16 => {
                    res = (*rb).resval.rint;
                }
                _ => {
                    return Err(Box::new(ArxError::XDataErr("must be a i16")));
                }
            }
            Ok(res)
        }
    }

    pub fn get_long(&self) -> error::Result<i32> {
        unsafe {
            let rb = self.cur;
            let rbtype = (*rb).restype;
            let res;
            match rbtype {
                kDxfXdInteger32=> {
                    res = (*rb).resval.rlong;
                }
                _ => {
                    return Err(Box::new(ArxError::XDataErr("must be a i32")));
                }
            }
            Ok(res)
        }
    }

    pub fn get_string(&self) -> error::Result<String> {
        unsafe {
            let rb = self.cur;
            let rbtype = (*rb).restype;
            let res;
            match rbtype {
                kDxfXdAsciiString=> {
                    res = (*rb).resval.rstring;
                }
                _ => {
                    return Err(Box::new(ArxError::XDataErr("must be a string")));
                }
            }
            Ok(res.to_string())
        }
    }

    pub fn get_appname(&self) -> error::Result<String> {
        unsafe {
            let rb = self.cur;
            let rbtype = (*rb).restype;
            let res;
            match rbtype {
                kDxfRegAppName=> {
                    res = (*rb).resval.rstring;
                }
                _ => {
                    return Err(Box::new(ArxError::XDataErr("must be a string")));
                }
            }
            Ok(res.to_string())
        }
    }

    pub fn get_control_string(&self) -> error::Result<String> {
        unsafe {
            let rb = self.cur;
            let rbtype = (*rb).restype;
            let res;
            match rbtype {
                kDxfXdControlString=> {
                    res = (*rb).resval.rstring;
                }
                _ => {
                    return Err(Box::new(ArxError::XDataErr("must be a string")));
                }
            }
            Ok(res.to_string())
        }
    }

    pub fn get_byte_buffer(&self) -> error::Result<Vec<u8>> {
        unsafe {
            let mut v = vec![];
            let mut rb = self.cur;
            // Get xdata vector<u8>
            while (!rb.is_null())  && ((*rb).restype  == kDxfXdBinaryChunk as i16) {
                let slice = std::slice::from_raw_parts(
                    (*rb).resval.rbinary.buf,
                    (*rb).resval.rbinary.clen as _) ;
                v.extend_from_slice(slice);
                rb = (*rb).rbnext;
            }
            Ok(v)
        }
    }

    pub fn into_inner(&self) -> resbuf {
        self.head
    }

    /// Creates a new null object
    pub fn null() -> XData<'a> {
        (::std::ptr::null_mut() as resbuf).into()
    }
}

impl<'a> std::ops::Deref for XData<'a> {
    type Target = resbuf;
    fn deref(&self) -> &Self::Target {
        &self.head
    }
}

impl<'a> From<resbuf> for XData<'a> {
    fn from(other: resbuf) -> Self {
        let mut last = other;
        while !last.is_null() {
            last = unsafe {(*last).rbnext};
        }
        XData {
            head: other,
            cur: last,
            lifetime: PhantomData,
        }
    }
}

impl<'a> Drop for XData<'a> {
    fn drop(&mut self) {
        unsafe {acut::rel_rb(self.head)};
    }
}
