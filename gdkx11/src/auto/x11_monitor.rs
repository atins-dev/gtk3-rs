// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{prelude::*, translate::*};
use x11::xlib;

glib::wrapper! {
    #[doc(alias = "GdkX11Monitor")]
    pub struct X11Monitor(Object<ffi::GdkX11Monitor, ffi::GdkX11MonitorClass>) @extends gdk::Monitor;

    match fn {
        type_ => || ffi::gdk_x11_monitor_get_type(),
    }
}

impl X11Monitor {
    #[doc(alias = "gdk_x11_monitor_get_output")]
    #[doc(alias = "get_output")]
    pub fn output(monitor: &impl IsA<gdk::Monitor>) -> xlib::XID {
        assert_initialized_main_thread!();
        unsafe { ffi::gdk_x11_monitor_get_output(monitor.as_ref().to_glib_none().0) }
    }
}
