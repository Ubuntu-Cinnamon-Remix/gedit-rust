// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::Window;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GeditWindowActivatable")]
    pub struct WindowActivatable(Interface<ffi::GeditWindowActivatable, ffi::GeditWindowActivatableInterface>);

    match fn {
        type_ => || ffi::gedit_window_activatable_get_type(),
    }
}

impl WindowActivatable {
        pub const NONE: Option<&'static WindowActivatable> = None;
    
}

pub trait WindowActivatableExt: 'static {
    #[doc(alias = "gedit_window_activatable_activate")]
    fn activate(&self);

    #[doc(alias = "gedit_window_activatable_deactivate")]
    fn deactivate(&self);

    #[doc(alias = "gedit_window_activatable_update_state")]
    fn update_state(&self);

    fn window(&self) -> Option<Window>;
}

impl<O: IsA<WindowActivatable>> WindowActivatableExt for O {
    fn activate(&self) {
        unsafe {
            ffi::gedit_window_activatable_activate(self.as_ref().to_glib_none().0);
        }
    }

    fn deactivate(&self) {
        unsafe {
            ffi::gedit_window_activatable_deactivate(self.as_ref().to_glib_none().0);
        }
    }

    fn update_state(&self) {
        unsafe {
            ffi::gedit_window_activatable_update_state(self.as_ref().to_glib_none().0);
        }
    }

    fn window(&self) -> Option<Window> {
        glib::ObjectExt::property(self.as_ref(), "window")
    }
}

impl fmt::Display for WindowActivatable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WindowActivatable")
    }
}
