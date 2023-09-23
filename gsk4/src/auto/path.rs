// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{FillRule, PathPoint, Stroke};
use glib::translate::*;
use std::{fmt, mem};

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Path(Shared<ffi::GskPath>);

    match fn {
        ref => |ptr| ffi::gsk_path_ref(ptr),
        unref => |ptr| ffi::gsk_path_unref(ptr),
        type_ => || ffi::gsk_path_get_type(),
    }
}

impl Path {
    #[doc(alias = "gsk_path_get_bounds")]
    #[doc(alias = "get_bounds")]
    pub fn bounds(&self) -> Option<graphene::Rect> {
        unsafe {
            let mut bounds = graphene::Rect::uninitialized();
            let ret = from_glib(ffi::gsk_path_get_bounds(
                self.to_glib_none().0,
                bounds.to_glib_none_mut().0,
            ));
            if ret {
                Some(bounds)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gsk_path_get_closest_point")]
    #[doc(alias = "get_closest_point")]
    pub fn closest_point(
        &self,
        point: &graphene::Point,
        threshold: f32,
    ) -> Option<(PathPoint, f32)> {
        unsafe {
            let mut result = PathPoint::uninitialized();
            let mut distance = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gsk_path_get_closest_point(
                self.to_glib_none().0,
                point.to_glib_none().0,
                threshold,
                result.to_glib_none_mut().0,
                distance.as_mut_ptr(),
            ));
            if ret {
                Some((result, distance.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gsk_path_get_end_point")]
    #[doc(alias = "get_end_point")]
    pub fn end_point(&self) -> Option<PathPoint> {
        unsafe {
            let mut result = PathPoint::uninitialized();
            let ret = from_glib(ffi::gsk_path_get_end_point(
                self.to_glib_none().0,
                result.to_glib_none_mut().0,
            ));
            if ret {
                Some(result)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gsk_path_get_start_point")]
    #[doc(alias = "get_start_point")]
    pub fn start_point(&self) -> Option<PathPoint> {
        unsafe {
            let mut result = PathPoint::uninitialized();
            let ret = from_glib(ffi::gsk_path_get_start_point(
                self.to_glib_none().0,
                result.to_glib_none_mut().0,
            ));
            if ret {
                Some(result)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gsk_path_get_stroke_bounds")]
    #[doc(alias = "get_stroke_bounds")]
    pub fn stroke_bounds(&self, stroke: &Stroke) -> Option<graphene::Rect> {
        unsafe {
            let mut bounds = graphene::Rect::uninitialized();
            let ret = from_glib(ffi::gsk_path_get_stroke_bounds(
                self.to_glib_none().0,
                stroke.to_glib_none().0,
                bounds.to_glib_none_mut().0,
            ));
            if ret {
                Some(bounds)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gsk_path_in_fill")]
    pub fn in_fill(&self, point: &graphene::Point, fill_rule: FillRule) -> bool {
        unsafe {
            from_glib(ffi::gsk_path_in_fill(
                self.to_glib_none().0,
                point.to_glib_none().0,
                fill_rule.into_glib(),
            ))
        }
    }

    #[doc(alias = "gsk_path_is_closed")]
    pub fn is_closed(&self) -> bool {
        unsafe { from_glib(ffi::gsk_path_is_closed(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_path_is_empty")]
    pub fn is_empty(&self) -> bool {
        unsafe { from_glib(ffi::gsk_path_is_empty(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_path_to_cairo")]
    pub fn to_cairo(&self, cr: &cairo::Context) {
        unsafe {
            ffi::gsk_path_to_cairo(self.to_glib_none().0, mut_override(cr.to_glib_none().0));
        }
    }

    #[doc(alias = "gsk_path_to_string")]
    #[doc(alias = "to_string")]
    pub fn to_str(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::gsk_path_to_string(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_path_parse")]
    pub fn parse(string: &str) -> Result<Path, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gsk_path_parse(string.to_glib_none().0))
                .ok_or_else(|| glib::bool_error!("Can't parse Path"))
        }
    }
}

impl fmt::Display for Path {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}
