// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Path, PathDirection};
use glib::translate::*;
use std::cmp;

glib::wrapper! {
    pub struct PathPoint(BoxedInline<ffi::GskPathPoint>);

    match fn {
        copy => |ptr| ffi::gsk_path_point_copy(mut_override(ptr)),
        free => |ptr| ffi::gsk_path_point_free(ptr),
        type_ => || ffi::gsk_path_point_get_type(),
    }
}

impl PathPoint {
    #[doc(alias = "gsk_path_point_compare")]
    fn compare(&self, point2: &PathPoint) -> i32 {
        unsafe { ffi::gsk_path_point_compare(self.to_glib_none().0, point2.to_glib_none().0) }
    }

    #[doc(alias = "gsk_path_point_equal")]
    fn equal(&self, point2: &PathPoint) -> bool {
        unsafe {
            from_glib(ffi::gsk_path_point_equal(
                self.to_glib_none().0,
                point2.to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "gsk_path_point_get_distance")]
    //#[doc(alias = "get_distance")]
    //pub fn distance(&self, measure: /*Ignored*/&PathMeasure) -> f32 {
    //    unsafe { TODO: call ffi:gsk_path_point_get_distance() }
    //}

    #[doc(alias = "gsk_path_point_get_position")]
    #[doc(alias = "get_position")]
    pub fn position(&self, path: &Path) -> graphene::Point {
        unsafe {
            let mut position = graphene::Point::uninitialized();
            ffi::gsk_path_point_get_position(
                self.to_glib_none().0,
                path.to_glib_none().0,
                position.to_glib_none_mut().0,
            );
            position
        }
    }

    #[doc(alias = "gsk_path_point_get_rotation")]
    #[doc(alias = "get_rotation")]
    pub fn rotation(&self, path: &Path, direction: PathDirection) -> f32 {
        unsafe {
            ffi::gsk_path_point_get_rotation(
                self.to_glib_none().0,
                path.to_glib_none().0,
                direction.into_glib(),
            )
        }
    }

    #[doc(alias = "gsk_path_point_get_tangent")]
    #[doc(alias = "get_tangent")]
    pub fn tangent(&self, path: &Path, direction: PathDirection) -> graphene::Vec2 {
        unsafe {
            let mut tangent = graphene::Vec2::uninitialized();
            ffi::gsk_path_point_get_tangent(
                self.to_glib_none().0,
                path.to_glib_none().0,
                direction.into_glib(),
                tangent.to_glib_none_mut().0,
            );
            tangent
        }
    }
}

impl PartialOrd for PathPoint {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.compare(other).partial_cmp(&0)
    }
}

impl Ord for PathPoint {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.compare(other).cmp(&0)
    }
}

impl PartialEq for PathPoint {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for PathPoint {}
