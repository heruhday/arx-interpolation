use crate::arx::acad_app::ErrorStatus;
use crate::arx::point3d;
use crate::arx::acdb::Entity;
use crate::arx::acdb::object::Object;
#[repr(C)]
#[derive(Debug)]
pub struct Curve {
    pub _base: Entity,
}

extern "C" {
    pub fn acdb_object_cast_to_acdb_curve(obj: *mut Object, pCurve: *mut *mut Curve) -> bool;
    pub fn acdb_curve_is_closed(pCurve: *mut Curve) -> bool;
    pub fn acdb_curve_get_start_param(pCurve: *mut Curve, param: &mut f64) -> ErrorStatus;
    pub fn acdb_curve_get_end_param(pCurve: *mut Curve, param: &mut f64) -> ErrorStatus;
    pub fn acdb_curve_get_start_point(pCurve: *mut Curve, point: &mut point3d) -> ErrorStatus;
    pub fn acdb_curve_get_end_point(pCurve: *mut Curve, point: &mut point3d) -> ErrorStatus;
    pub fn acdb_curve_get_point_at_param(pCurve: *mut Curve, param: f64, point: &mut point3d) -> ErrorStatus;
    pub fn acdb_curve_get_param_at_point(pCurve: *mut Curve, point: &point3d, param: &mut f64,) -> ErrorStatus;
    pub fn acdb_curve_get_dist_at_param(pCurve: *mut Curve, param: f64, dist: &mut f64) -> ErrorStatus;
    pub fn acdb_curve_get_param_at_dist(pCurve: *mut Curve, dist: f64, param: &mut f64) -> ErrorStatus;
    pub fn acdb_curve_get_point_at_dist(pCurve: *mut Curve, dist: f64, point: &mut point3d) -> ErrorStatus;
    pub fn acdb_curve_get_dist_at_point(pCurve: *mut Curve, point: &point3d, dist: &mut f64) -> ErrorStatus;
    pub fn acdb_curve_get_closest_point_to(pCurve: *mut Curve, givenPoint: &point3d, pointOnCurve: &mut point3d, extend: bool) -> ErrorStatus;
    pub fn acdb_curve_get_area(pCurve: *mut Curve, area: &mut f64) -> ErrorStatus;
    pub fn acdb_curve_get_length(pCurve: *mut Curve, length: &mut f64) -> ErrorStatus;
    pub fn acdb_curve_get_area_length(pCurve: *mut Curve, area: &mut f64, length: &mut f64) -> ErrorStatus;

}