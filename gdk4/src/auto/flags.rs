// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use bitflags::bitflags;
use glib::{translate::*, value::FromValue, value::ToValue, StaticType, Type};
use std::fmt;

bitflags! {
    #[doc(alias = "GdkAnchorHints")]
    pub struct AnchorHints: u32 {
        #[doc(alias = "GDK_ANCHOR_FLIP_X")]
        const FLIP_X = ffi::GDK_ANCHOR_FLIP_X as _;
        #[doc(alias = "GDK_ANCHOR_FLIP_Y")]
        const FLIP_Y = ffi::GDK_ANCHOR_FLIP_Y as _;
        #[doc(alias = "GDK_ANCHOR_SLIDE_X")]
        const SLIDE_X = ffi::GDK_ANCHOR_SLIDE_X as _;
        #[doc(alias = "GDK_ANCHOR_SLIDE_Y")]
        const SLIDE_Y = ffi::GDK_ANCHOR_SLIDE_Y as _;
        #[doc(alias = "GDK_ANCHOR_RESIZE_X")]
        const RESIZE_X = ffi::GDK_ANCHOR_RESIZE_X as _;
        #[doc(alias = "GDK_ANCHOR_RESIZE_Y")]
        const RESIZE_Y = ffi::GDK_ANCHOR_RESIZE_Y as _;
        #[doc(alias = "GDK_ANCHOR_FLIP")]
        const FLIP = ffi::GDK_ANCHOR_FLIP as _;
        #[doc(alias = "GDK_ANCHOR_SLIDE")]
        const SLIDE = ffi::GDK_ANCHOR_SLIDE as _;
        #[doc(alias = "GDK_ANCHOR_RESIZE")]
        const RESIZE = ffi::GDK_ANCHOR_RESIZE as _;
    }
}

impl fmt::Display for AnchorHints {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for AnchorHints {
    type GlibType = ffi::GdkAnchorHints;

    #[inline]
    fn into_glib(self) -> ffi::GdkAnchorHints {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkAnchorHints> for AnchorHints {
    #[inline]
    unsafe fn from_glib(value: ffi::GdkAnchorHints) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for AnchorHints {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_anchor_hints_get_type()) }
    }
}

impl glib::HasParamSpec for AnchorHints {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name| Self::ParamSpec::builder(name)
    }
}

impl glib::value::ValueType for AnchorHints {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for AnchorHints {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for AnchorHints {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<AnchorHints> for glib::Value {
    #[inline]
    fn from(v: AnchorHints) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "GdkAxisFlags")]
    pub struct AxisFlags: u32 {
        #[doc(alias = "GDK_AXIS_FLAG_X")]
        const X = ffi::GDK_AXIS_FLAG_X as _;
        #[doc(alias = "GDK_AXIS_FLAG_Y")]
        const Y = ffi::GDK_AXIS_FLAG_Y as _;
        #[doc(alias = "GDK_AXIS_FLAG_DELTA_X")]
        const DELTA_X = ffi::GDK_AXIS_FLAG_DELTA_X as _;
        #[doc(alias = "GDK_AXIS_FLAG_DELTA_Y")]
        const DELTA_Y = ffi::GDK_AXIS_FLAG_DELTA_Y as _;
        #[doc(alias = "GDK_AXIS_FLAG_PRESSURE")]
        const PRESSURE = ffi::GDK_AXIS_FLAG_PRESSURE as _;
        #[doc(alias = "GDK_AXIS_FLAG_XTILT")]
        const XTILT = ffi::GDK_AXIS_FLAG_XTILT as _;
        #[doc(alias = "GDK_AXIS_FLAG_YTILT")]
        const YTILT = ffi::GDK_AXIS_FLAG_YTILT as _;
        #[doc(alias = "GDK_AXIS_FLAG_WHEEL")]
        const WHEEL = ffi::GDK_AXIS_FLAG_WHEEL as _;
        #[doc(alias = "GDK_AXIS_FLAG_DISTANCE")]
        const DISTANCE = ffi::GDK_AXIS_FLAG_DISTANCE as _;
        #[doc(alias = "GDK_AXIS_FLAG_ROTATION")]
        const ROTATION = ffi::GDK_AXIS_FLAG_ROTATION as _;
        #[doc(alias = "GDK_AXIS_FLAG_SLIDER")]
        const SLIDER = ffi::GDK_AXIS_FLAG_SLIDER as _;
    }
}

impl fmt::Display for AxisFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for AxisFlags {
    type GlibType = ffi::GdkAxisFlags;

    #[inline]
    fn into_glib(self) -> ffi::GdkAxisFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkAxisFlags> for AxisFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GdkAxisFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for AxisFlags {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_axis_flags_get_type()) }
    }
}

impl glib::HasParamSpec for AxisFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name| Self::ParamSpec::builder(name)
    }
}

impl glib::value::ValueType for AxisFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for AxisFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for AxisFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<AxisFlags> for glib::Value {
    #[inline]
    fn from(v: AxisFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "GdkDragAction")]
    pub struct DragAction: u32 {
        #[doc(alias = "GDK_ACTION_COPY")]
        const COPY = ffi::GDK_ACTION_COPY as _;
        #[doc(alias = "GDK_ACTION_MOVE")]
        const MOVE = ffi::GDK_ACTION_MOVE as _;
        #[doc(alias = "GDK_ACTION_LINK")]
        const LINK = ffi::GDK_ACTION_LINK as _;
        #[doc(alias = "GDK_ACTION_ASK")]
        const ASK = ffi::GDK_ACTION_ASK as _;
    }
}

impl DragAction {
    #[doc(alias = "gdk_drag_action_is_unique")]
    pub fn is_unique(self) -> bool {
        assert_initialized_main_thread!();
        unsafe { from_glib(ffi::gdk_drag_action_is_unique(self.into_glib())) }
    }
}

impl fmt::Display for DragAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for DragAction {
    type GlibType = ffi::GdkDragAction;

    #[inline]
    fn into_glib(self) -> ffi::GdkDragAction {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkDragAction> for DragAction {
    #[inline]
    unsafe fn from_glib(value: ffi::GdkDragAction) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for DragAction {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_drag_action_get_type()) }
    }
}

impl glib::HasParamSpec for DragAction {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name| Self::ParamSpec::builder(name)
    }
}

impl glib::value::ValueType for DragAction {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for DragAction {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for DragAction {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<DragAction> for glib::Value {
    #[inline]
    fn from(v: DragAction) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "GdkFrameClockPhase")]
    pub struct FrameClockPhase: u32 {
        #[doc(alias = "GDK_FRAME_CLOCK_PHASE_NONE")]
        const NONE = ffi::GDK_FRAME_CLOCK_PHASE_NONE as _;
        #[doc(alias = "GDK_FRAME_CLOCK_PHASE_FLUSH_EVENTS")]
        const FLUSH_EVENTS = ffi::GDK_FRAME_CLOCK_PHASE_FLUSH_EVENTS as _;
        #[doc(alias = "GDK_FRAME_CLOCK_PHASE_BEFORE_PAINT")]
        const BEFORE_PAINT = ffi::GDK_FRAME_CLOCK_PHASE_BEFORE_PAINT as _;
        #[doc(alias = "GDK_FRAME_CLOCK_PHASE_UPDATE")]
        const UPDATE = ffi::GDK_FRAME_CLOCK_PHASE_UPDATE as _;
        #[doc(alias = "GDK_FRAME_CLOCK_PHASE_LAYOUT")]
        const LAYOUT = ffi::GDK_FRAME_CLOCK_PHASE_LAYOUT as _;
        #[doc(alias = "GDK_FRAME_CLOCK_PHASE_PAINT")]
        const PAINT = ffi::GDK_FRAME_CLOCK_PHASE_PAINT as _;
        #[doc(alias = "GDK_FRAME_CLOCK_PHASE_RESUME_EVENTS")]
        const RESUME_EVENTS = ffi::GDK_FRAME_CLOCK_PHASE_RESUME_EVENTS as _;
        #[doc(alias = "GDK_FRAME_CLOCK_PHASE_AFTER_PAINT")]
        const AFTER_PAINT = ffi::GDK_FRAME_CLOCK_PHASE_AFTER_PAINT as _;
    }
}

impl fmt::Display for FrameClockPhase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for FrameClockPhase {
    type GlibType = ffi::GdkFrameClockPhase;

    #[inline]
    fn into_glib(self) -> ffi::GdkFrameClockPhase {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkFrameClockPhase> for FrameClockPhase {
    #[inline]
    unsafe fn from_glib(value: ffi::GdkFrameClockPhase) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for FrameClockPhase {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_frame_clock_phase_get_type()) }
    }
}

impl glib::HasParamSpec for FrameClockPhase {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name| Self::ParamSpec::builder(name)
    }
}

impl glib::value::ValueType for FrameClockPhase {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for FrameClockPhase {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for FrameClockPhase {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<FrameClockPhase> for glib::Value {
    #[inline]
    fn from(v: FrameClockPhase) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[cfg(feature = "v4_6")]
bitflags! {
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[doc(alias = "GdkGLAPI")]
    pub struct GLAPI: u32 {
        #[doc(alias = "GDK_GL_API_GL")]
        const GL = ffi::GDK_GL_API_GL as _;
        #[doc(alias = "GDK_GL_API_GLES")]
        const GLES = ffi::GDK_GL_API_GLES as _;
    }
}

#[cfg(feature = "v4_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
impl fmt::Display for GLAPI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(feature = "v4_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
#[doc(hidden)]
impl IntoGlib for GLAPI {
    type GlibType = ffi::GdkGLAPI;

    #[inline]
    fn into_glib(self) -> ffi::GdkGLAPI {
        self.bits()
    }
}

#[cfg(feature = "v4_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
#[doc(hidden)]
impl FromGlib<ffi::GdkGLAPI> for GLAPI {
    #[inline]
    unsafe fn from_glib(value: ffi::GdkGLAPI) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(feature = "v4_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
impl StaticType for GLAPI {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_gl_api_get_type()) }
    }
}

#[cfg(feature = "v4_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
impl glib::HasParamSpec for GLAPI {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name| Self::ParamSpec::builder(name)
    }
}

#[cfg(feature = "v4_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
impl glib::value::ValueType for GLAPI {
    type Type = Self;
}

#[cfg(feature = "v4_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
unsafe impl<'a> FromValue<'a> for GLAPI {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(feature = "v4_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
impl ToValue for GLAPI {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(feature = "v4_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
impl From<GLAPI> for glib::Value {
    #[inline]
    fn from(v: GLAPI) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "GdkModifierType")]
    pub struct ModifierType: u32 {
        #[doc(alias = "GDK_SHIFT_MASK")]
        const SHIFT_MASK = ffi::GDK_SHIFT_MASK as _;
        #[doc(alias = "GDK_LOCK_MASK")]
        const LOCK_MASK = ffi::GDK_LOCK_MASK as _;
        #[doc(alias = "GDK_CONTROL_MASK")]
        const CONTROL_MASK = ffi::GDK_CONTROL_MASK as _;
        #[doc(alias = "GDK_ALT_MASK")]
        const ALT_MASK = ffi::GDK_ALT_MASK as _;
        #[doc(alias = "GDK_BUTTON1_MASK")]
        const BUTTON1_MASK = ffi::GDK_BUTTON1_MASK as _;
        #[doc(alias = "GDK_BUTTON2_MASK")]
        const BUTTON2_MASK = ffi::GDK_BUTTON2_MASK as _;
        #[doc(alias = "GDK_BUTTON3_MASK")]
        const BUTTON3_MASK = ffi::GDK_BUTTON3_MASK as _;
        #[doc(alias = "GDK_BUTTON4_MASK")]
        const BUTTON4_MASK = ffi::GDK_BUTTON4_MASK as _;
        #[doc(alias = "GDK_BUTTON5_MASK")]
        const BUTTON5_MASK = ffi::GDK_BUTTON5_MASK as _;
        #[doc(alias = "GDK_SUPER_MASK")]
        const SUPER_MASK = ffi::GDK_SUPER_MASK as _;
        #[doc(alias = "GDK_HYPER_MASK")]
        const HYPER_MASK = ffi::GDK_HYPER_MASK as _;
        #[doc(alias = "GDK_META_MASK")]
        const META_MASK = ffi::GDK_META_MASK as _;
    }
}

impl fmt::Display for ModifierType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for ModifierType {
    type GlibType = ffi::GdkModifierType;

    #[inline]
    fn into_glib(self) -> ffi::GdkModifierType {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkModifierType> for ModifierType {
    #[inline]
    unsafe fn from_glib(value: ffi::GdkModifierType) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for ModifierType {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_modifier_type_get_type()) }
    }
}

impl glib::HasParamSpec for ModifierType {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name| Self::ParamSpec::builder(name)
    }
}

impl glib::value::ValueType for ModifierType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for ModifierType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for ModifierType {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<ModifierType> for glib::Value {
    #[inline]
    fn from(v: ModifierType) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "GdkPaintableFlags")]
    pub struct PaintableFlags: u32 {
        #[doc(alias = "GDK_PAINTABLE_STATIC_SIZE")]
        const SIZE = ffi::GDK_PAINTABLE_STATIC_SIZE as _;
        #[doc(alias = "GDK_PAINTABLE_STATIC_CONTENTS")]
        const CONTENTS = ffi::GDK_PAINTABLE_STATIC_CONTENTS as _;
    }
}

impl fmt::Display for PaintableFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for PaintableFlags {
    type GlibType = ffi::GdkPaintableFlags;

    #[inline]
    fn into_glib(self) -> ffi::GdkPaintableFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkPaintableFlags> for PaintableFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GdkPaintableFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for PaintableFlags {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_paintable_flags_get_type()) }
    }
}

impl glib::HasParamSpec for PaintableFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name| Self::ParamSpec::builder(name)
    }
}

impl glib::value::ValueType for PaintableFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PaintableFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for PaintableFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<PaintableFlags> for glib::Value {
    #[inline]
    fn from(v: PaintableFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "GdkSeatCapabilities")]
    pub struct SeatCapabilities: u32 {
        #[doc(alias = "GDK_SEAT_CAPABILITY_NONE")]
        const NONE = ffi::GDK_SEAT_CAPABILITY_NONE as _;
        #[doc(alias = "GDK_SEAT_CAPABILITY_POINTER")]
        const POINTER = ffi::GDK_SEAT_CAPABILITY_POINTER as _;
        #[doc(alias = "GDK_SEAT_CAPABILITY_TOUCH")]
        const TOUCH = ffi::GDK_SEAT_CAPABILITY_TOUCH as _;
        #[doc(alias = "GDK_SEAT_CAPABILITY_TABLET_STYLUS")]
        const TABLET_STYLUS = ffi::GDK_SEAT_CAPABILITY_TABLET_STYLUS as _;
        #[doc(alias = "GDK_SEAT_CAPABILITY_KEYBOARD")]
        const KEYBOARD = ffi::GDK_SEAT_CAPABILITY_KEYBOARD as _;
        #[doc(alias = "GDK_SEAT_CAPABILITY_TABLET_PAD")]
        const TABLET_PAD = ffi::GDK_SEAT_CAPABILITY_TABLET_PAD as _;
        #[doc(alias = "GDK_SEAT_CAPABILITY_ALL_POINTING")]
        const ALL_POINTING = ffi::GDK_SEAT_CAPABILITY_ALL_POINTING as _;
        #[doc(alias = "GDK_SEAT_CAPABILITY_ALL")]
        const ALL = ffi::GDK_SEAT_CAPABILITY_ALL as _;
    }
}

impl fmt::Display for SeatCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for SeatCapabilities {
    type GlibType = ffi::GdkSeatCapabilities;

    #[inline]
    fn into_glib(self) -> ffi::GdkSeatCapabilities {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkSeatCapabilities> for SeatCapabilities {
    #[inline]
    unsafe fn from_glib(value: ffi::GdkSeatCapabilities) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for SeatCapabilities {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_seat_capabilities_get_type()) }
    }
}

impl glib::HasParamSpec for SeatCapabilities {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name| Self::ParamSpec::builder(name)
    }
}

impl glib::value::ValueType for SeatCapabilities {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for SeatCapabilities {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for SeatCapabilities {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<SeatCapabilities> for glib::Value {
    #[inline]
    fn from(v: SeatCapabilities) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "GdkToplevelState")]
    pub struct ToplevelState: u32 {
        #[doc(alias = "GDK_TOPLEVEL_STATE_MINIMIZED")]
        const MINIMIZED = ffi::GDK_TOPLEVEL_STATE_MINIMIZED as _;
        #[doc(alias = "GDK_TOPLEVEL_STATE_MAXIMIZED")]
        const MAXIMIZED = ffi::GDK_TOPLEVEL_STATE_MAXIMIZED as _;
        #[doc(alias = "GDK_TOPLEVEL_STATE_STICKY")]
        const STICKY = ffi::GDK_TOPLEVEL_STATE_STICKY as _;
        #[doc(alias = "GDK_TOPLEVEL_STATE_FULLSCREEN")]
        const FULLSCREEN = ffi::GDK_TOPLEVEL_STATE_FULLSCREEN as _;
        #[doc(alias = "GDK_TOPLEVEL_STATE_ABOVE")]
        const ABOVE = ffi::GDK_TOPLEVEL_STATE_ABOVE as _;
        #[doc(alias = "GDK_TOPLEVEL_STATE_BELOW")]
        const BELOW = ffi::GDK_TOPLEVEL_STATE_BELOW as _;
        #[doc(alias = "GDK_TOPLEVEL_STATE_FOCUSED")]
        const FOCUSED = ffi::GDK_TOPLEVEL_STATE_FOCUSED as _;
        #[doc(alias = "GDK_TOPLEVEL_STATE_TILED")]
        const TILED = ffi::GDK_TOPLEVEL_STATE_TILED as _;
        #[doc(alias = "GDK_TOPLEVEL_STATE_TOP_TILED")]
        const TOP_TILED = ffi::GDK_TOPLEVEL_STATE_TOP_TILED as _;
        #[doc(alias = "GDK_TOPLEVEL_STATE_TOP_RESIZABLE")]
        const TOP_RESIZABLE = ffi::GDK_TOPLEVEL_STATE_TOP_RESIZABLE as _;
        #[doc(alias = "GDK_TOPLEVEL_STATE_RIGHT_TILED")]
        const RIGHT_TILED = ffi::GDK_TOPLEVEL_STATE_RIGHT_TILED as _;
        #[doc(alias = "GDK_TOPLEVEL_STATE_RIGHT_RESIZABLE")]
        const RIGHT_RESIZABLE = ffi::GDK_TOPLEVEL_STATE_RIGHT_RESIZABLE as _;
        #[doc(alias = "GDK_TOPLEVEL_STATE_BOTTOM_TILED")]
        const BOTTOM_TILED = ffi::GDK_TOPLEVEL_STATE_BOTTOM_TILED as _;
        #[doc(alias = "GDK_TOPLEVEL_STATE_BOTTOM_RESIZABLE")]
        const BOTTOM_RESIZABLE = ffi::GDK_TOPLEVEL_STATE_BOTTOM_RESIZABLE as _;
        #[doc(alias = "GDK_TOPLEVEL_STATE_LEFT_TILED")]
        const LEFT_TILED = ffi::GDK_TOPLEVEL_STATE_LEFT_TILED as _;
        #[doc(alias = "GDK_TOPLEVEL_STATE_LEFT_RESIZABLE")]
        const LEFT_RESIZABLE = ffi::GDK_TOPLEVEL_STATE_LEFT_RESIZABLE as _;
    }
}

impl fmt::Display for ToplevelState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for ToplevelState {
    type GlibType = ffi::GdkToplevelState;

    #[inline]
    fn into_glib(self) -> ffi::GdkToplevelState {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkToplevelState> for ToplevelState {
    #[inline]
    unsafe fn from_glib(value: ffi::GdkToplevelState) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for ToplevelState {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_toplevel_state_get_type()) }
    }
}

impl glib::HasParamSpec for ToplevelState {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name| Self::ParamSpec::builder(name)
    }
}

impl glib::value::ValueType for ToplevelState {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for ToplevelState {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for ToplevelState {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<ToplevelState> for glib::Value {
    #[inline]
    fn from(v: ToplevelState) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
