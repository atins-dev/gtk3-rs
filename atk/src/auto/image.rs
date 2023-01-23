// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::CoordType;
use glib::{prelude::*, translate::*};
use std::{fmt, mem};

glib::wrapper! {
    #[doc(alias = "AtkImage")]
    pub struct Image(Interface<ffi::AtkImage, ffi::AtkImageIface>);

    match fn {
        type_ => || ffi::atk_image_get_type(),
    }
}

impl Image {
    pub const NONE: Option<&'static Image> = None;
}

pub trait AtkImageExt: 'static {
    #[doc(alias = "atk_image_get_image_description")]
    #[doc(alias = "get_image_description")]
    fn image_description(&self) -> Option<glib::GString>;

    #[doc(alias = "atk_image_get_image_locale")]
    #[doc(alias = "get_image_locale")]
    fn image_locale(&self) -> Option<glib::GString>;

    #[doc(alias = "atk_image_get_image_position")]
    #[doc(alias = "get_image_position")]
    fn image_position(&self, coord_type: CoordType) -> (i32, i32);

    #[doc(alias = "atk_image_get_image_size")]
    #[doc(alias = "get_image_size")]
    fn image_size(&self) -> (i32, i32);

    #[doc(alias = "atk_image_set_image_description")]
    fn set_image_description(&self, description: &str) -> bool;
}

impl<O: IsA<Image>> AtkImageExt for O {
    fn image_description(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::atk_image_get_image_description(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn image_locale(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::atk_image_get_image_locale(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn image_position(&self, coord_type: CoordType) -> (i32, i32) {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            ffi::atk_image_get_image_position(
                self.as_ref().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
                coord_type.into_glib(),
            );
            (x.assume_init(), y.assume_init())
        }
    }

    fn image_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::atk_image_get_image_size(
                self.as_ref().to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            (width.assume_init(), height.assume_init())
        }
    }

    fn set_image_description(&self, description: &str) -> bool {
        unsafe {
            from_glib(ffi::atk_image_set_image_description(
                self.as_ref().to_glib_none().0,
                description.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Image")
    }
}
