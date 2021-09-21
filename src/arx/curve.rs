use crate::arx::{point3d};
use std::marker::PhantomData;
use crate::arx::acdb::curve::*;
use crate::error::ArxError;
use crate::arx::acad_app::error_status::eOk;
use crate::arx::acdb::curve;
use crate::arx::acdb;
use std::ptr::null_mut;
use crate::arx::object::Object;
use crate::qprintln;
use crate::error;
use crate::arx::string::ToWideChars;

pub trait TryFrom<T>: Sized {
    fn try_from(value: T) -> error::Result<Self>;
}

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Curve<'a> {
    internal: *mut curve::Curve,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> Curve<'a> {
    pub fn get_area(&self) -> error::Result<f64> {
        let mut area = 0f64;
        let rs = unsafe {acdb_curve_get_area(self.internal, &mut area)};

        if rs != eOk {
            return Err(Box::new(ArxError::CurveErr("get area")));
        }
        Ok(area)
    }

    pub fn get_area_length(&self) -> error::Result<(f64, f64)> {
        let mut area = 0f64;
        let mut len = 0f64;
        let rs = unsafe {acdb_curve_get_area_length(self.internal, &mut area, &mut len)};

        if rs != eOk {
            return Err(Box::new(ArxError::CurveErr("get area")));
        }

        Ok((area,len))

    }

    pub fn get_length(&self) -> error::Result<f64> {
        let mut len = 0f64;
        let rs = unsafe {acdb_curve_get_length(self.internal, &mut len)};
        if rs != eOk {
            return Err(Box::new(ArxError::CurveErr("get length")));
        }
        Ok(len)
    }

    pub fn is_closed(&self) -> bool {
        unsafe {acdb_curve_is_closed(self.internal)}
    }

    pub fn get_start_param(&self) -> error::Result<f64> {
        unsafe {
            let mut param = 0f64;
            let rs = acdb_curve_get_start_param(self.internal, &mut param);
            if rs != eOk {
                return Err(Box::new(ArxError::CurveErr("get start param")));
            }
            Ok(param)
        }
    }

    pub fn get_end_param(&self) -> error::Result<f64> {
        unsafe {
            let mut param = 0f64;
            let rs = acdb_curve_get_end_param(self.internal, &mut param);
            if rs != eOk {
                return Err(Box::new(ArxError::CurveErr("get start param")));
            }
            Ok(param)
        }
    }

    pub fn get_start_point(&self) -> error::Result<point3d> {
        unsafe {
            let mut pt = std::mem::zeroed();
            let rs = acdb_curve_get_start_point(self.internal, &mut pt);
            if rs != eOk {
                return Err(Box::new(ArxError::CurveErr("get start param")));
            }
            Ok(pt)
        }
    }

    pub fn get_end_point(&self) -> error::Result<point3d> {
        unsafe {
            let mut pt = std::mem::zeroed();
            let rs = acdb_curve_get_end_point(self.internal, &mut pt);
            if rs != eOk {
                return Err(Box::new(ArxError::CurveErr("get end param")));
            }
            Ok(pt)
        }
    }

    pub fn get_point_at_param(&self, param: f64) -> error::Result<point3d> {
        unsafe {
            let mut pt = std::mem::zeroed();
            let rs = acdb_curve_get_point_at_param(self.internal, param, &mut pt);
            if rs != eOk {
                return Err(Box::new(ArxError::CurveErr("get point at param")));
            }
            Ok(pt)
        }
    }

    pub fn get_param_at_point(&self, pt: &point3d) -> error::Result<f64> {
        unsafe {
            let mut param = 0f64;
            let rs = acdb_curve_get_param_at_point(self.internal, pt, &mut param);
            if rs != eOk {
                return Err(Box::new(ArxError::CurveErr("get param at point")));
            }
            Ok(param)
        }
    }

    pub fn get_param_at_dist(&self, dist: f64) -> error::Result<f64> {
        unsafe {
            let mut param = 0f64;
            let rs = acdb_curve_get_param_at_dist(self.internal, dist, &mut param);
            if rs != eOk {
                return Err(Box::new(ArxError::CurveErr("get param at dist")));
            }
            Ok(param)
        }
    }

    pub fn get_dist_at_param(&self, param: f64) -> error::Result<f64> {
        unsafe {
            let mut dist = 0f64;
            let rs = acdb_curve_get_param_at_dist(self.internal, param, &mut dist);
            if rs != eOk {
                return Err(Box::new(ArxError::CurveErr("get dist at param")));
            }
            Ok(dist)
        }
    }

    pub fn get_point_at_dist(&self, dist: f64) -> error::Result<point3d> {
        unsafe {
            let mut pt = std::mem::zeroed();
            let rs = acdb_curve_get_point_at_dist(self.internal, dist, &mut pt);
            if rs != eOk {
                return Err(Box::new(ArxError::CurveErr("get point at dist")));
            }
            Ok(pt)
        }
    }

    pub fn get_dist_at_point(&self, pt: &point3d) -> error::Result<f64> {
        unsafe {
            let mut dist = 0f64;
            let rs = acdb_curve_get_dist_at_point(self.internal, pt, &mut dist);
            if rs != eOk {
                return Err(Box::new(ArxError::CurveErr("get dist at point")));
            }
            Ok(dist)
        }
    }

    pub fn get_closest_point_to(&self, given_point: &point3d, extend: bool) -> error::Result<point3d>{
        unsafe {
            let mut point_on_curve = std::mem::zeroed();
            let rs = acdb_curve_get_closest_point_to(self.internal, given_point, &mut point_on_curve, extend);
            if rs != eOk {
                return Err(Box::new(ArxError::CurveErr("get closest point to")));
            }
            Ok(point_on_curve)
        }
    }

    pub fn print_address(&self) {
         unsafe {qprintln!("curve address: {:p}", self.internal);}
    }
}

impl<'a> std::ops::Deref for Curve<'a> {
    type Target = *mut curve::Curve;
    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a> From<*mut curve::Curve> for Curve<'a> {
    fn from(other: *mut curve::Curve) -> Self {
        Curve {
            internal: other,
            lifetime: PhantomData,
        }
    }
}

impl<'a> TryFrom<*mut acdb::object::Object> for Curve<'a> {
    fn try_from(other: *mut acdb::object::Object) -> error::Result<Self> {
        let mut curve = null_mut();
        let res = unsafe {acdb_object_cast_to_acdb_curve(other, &mut curve)};
        if !res{
            return Err(Box::new(ArxError::CurveErr("try from object")));
        }
        Ok(Curve {
            internal: curve,
            lifetime: PhantomData,
        })
    }

}

impl<'a> TryFrom<Object<'a>> for Curve<'a> {
    fn try_from(other: Object) -> error::Result<Self> {
        let other = other.into_inner();
        let mut curve = null_mut();
        let res = unsafe {acdb_object_cast_to_acdb_curve(other, &mut curve)};
        if !res{
            return Err(Box::new(ArxError::CurveErr("try from Object")));
        }
        Ok(Curve {
            internal: curve,
            lifetime: PhantomData,
        })
    }

}

impl<'a> TryFrom<&Object<'a>> for Curve<'a> {
    fn try_from(other: &Object) -> error::Result<Self> {
        let other = (*other).into_inner();
        let mut curve = null_mut();
        let res = unsafe {acdb_object_cast_to_acdb_curve(other, &mut curve)};
        if !res{
            return Err(Box::new(ArxError::CurveErr("try from Object")));
        }
        Ok(Curve {
            internal: curve,
            lifetime: PhantomData,
        })
    }

}

