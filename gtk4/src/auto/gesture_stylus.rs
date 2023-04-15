// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{EventController, Gesture, GestureSingle, PropagationLimit, PropagationPhase};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem, mem::transmute, ptr};

glib::wrapper! {
    #[doc(alias = "GtkGestureStylus")]
    pub struct GestureStylus(Object<ffi::GtkGestureStylus, ffi::GtkGestureStylusClass>) @extends GestureSingle, Gesture, EventController;

    match fn {
        type_ => || ffi::gtk_gesture_stylus_get_type(),
    }
}

impl GestureStylus {
    #[doc(alias = "gtk_gesture_stylus_new")]
    pub fn new() -> GestureStylus {
        assert_initialized_main_thread!();
        unsafe { Gesture::from_glib_full(ffi::gtk_gesture_stylus_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`GestureStylus`] objects.
    ///
    /// This method returns an instance of [`GestureStylusBuilder`](crate::builders::GestureStylusBuilder) which can be used to create [`GestureStylus`] objects.
    pub fn builder() -> GestureStylusBuilder {
        GestureStylusBuilder::new()
    }

    #[doc(alias = "gtk_gesture_stylus_get_axis")]
    #[doc(alias = "get_axis")]
    pub fn axis(&self, axis: gdk::AxisUse) -> Option<f64> {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_gesture_stylus_get_axis(
                self.to_glib_none().0,
                axis.into_glib(),
                value.as_mut_ptr(),
            ));
            if ret {
                Some(value.assume_init())
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_gesture_stylus_get_backlog")]
    #[doc(alias = "get_backlog")]
    pub fn backlog(&self) -> Option<Vec<gdk::TimeCoord>> {
        unsafe {
            let mut backlog = ptr::null_mut();
            let mut n_elems = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_gesture_stylus_get_backlog(
                self.to_glib_none().0,
                &mut backlog,
                n_elems.as_mut_ptr(),
            ));
            if ret {
                Some(FromGlibContainer::from_glib_full_num(
                    backlog,
                    n_elems.assume_init() as _,
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_gesture_stylus_get_device_tool")]
    #[doc(alias = "get_device_tool")]
    pub fn device_tool(&self) -> Option<gdk::DeviceTool> {
        unsafe {
            from_glib_none(ffi::gtk_gesture_stylus_get_device_tool(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_gesture_stylus_get_stylus_only")]
    #[doc(alias = "get_stylus_only")]
    pub fn is_stylus_only(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_stylus_get_stylus_only(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_gesture_stylus_set_stylus_only")]
    pub fn set_stylus_only(&self, stylus_only: bool) {
        unsafe {
            ffi::gtk_gesture_stylus_set_stylus_only(self.to_glib_none().0, stylus_only.into_glib());
        }
    }

    #[doc(alias = "down")]
    pub fn connect_down<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn down_trampoline<F: Fn(&GestureStylus, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureStylus,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"down\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    down_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "motion")]
    pub fn connect_motion<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn motion_trampoline<F: Fn(&GestureStylus, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureStylus,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"motion\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    motion_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "proximity")]
    pub fn connect_proximity<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn proximity_trampoline<F: Fn(&GestureStylus, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureStylus,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"proximity\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    proximity_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "up")]
    pub fn connect_up<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn up_trampoline<F: Fn(&GestureStylus, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureStylus,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"up\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    up_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "stylus-only")]
    pub fn connect_stylus_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_stylus_only_trampoline<F: Fn(&GestureStylus) + 'static>(
            this: *mut ffi::GtkGestureStylus,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::stylus-only\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_stylus_only_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for GestureStylus {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`GestureStylus`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct GestureStylusBuilder {
    builder: glib::object::ObjectBuilder<'static, GestureStylus>,
}

impl GestureStylusBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    pub fn stylus_only(self, stylus_only: bool) -> Self {
        Self {
            builder: self.builder.property("stylus-only", stylus_only),
        }
    }

    pub fn button(self, button: u32) -> Self {
        Self {
            builder: self.builder.property("button", button),
        }
    }

    pub fn exclusive(self, exclusive: bool) -> Self {
        Self {
            builder: self.builder.property("exclusive", exclusive),
        }
    }

    pub fn touch_only(self, touch_only: bool) -> Self {
        Self {
            builder: self.builder.property("touch-only", touch_only),
        }
    }

    pub fn n_points(self, n_points: u32) -> Self {
        Self {
            builder: self.builder.property("n-points", n_points),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn propagation_limit(self, propagation_limit: PropagationLimit) -> Self {
        Self {
            builder: self
                .builder
                .property("propagation-limit", propagation_limit),
        }
    }

    pub fn propagation_phase(self, propagation_phase: PropagationPhase) -> Self {
        Self {
            builder: self
                .builder
                .property("propagation-phase", propagation_phase),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`GestureStylus`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> GestureStylus {
        self.builder.build()
    }
}

impl fmt::Display for GestureStylus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GestureStylus")
    }
}
