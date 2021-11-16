// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkX11DeviceCore")]
    pub struct X11DeviceCore(Object<ffi::GdkX11DeviceCore, ffi::GdkX11DeviceCoreClass>) @extends gdk::Device;

    match fn {
        type_ => || ffi::gdk_x11_device_core_get_type(),
    }
}

impl fmt::Display for X11DeviceCore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("X11DeviceCore")
    }
}
