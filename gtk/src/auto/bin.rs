// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Buildable, Container, Widget};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GtkBin")]
    pub struct Bin(Object<ffi::GtkBin, ffi::GtkBinClass>) @extends Container, Widget, @implements Buildable;

    match fn {
        type_ => || ffi::gtk_bin_get_type(),
    }
}

impl Bin {
    pub const NONE: Option<&'static Bin> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Bin>> Sealed for T {}
}

pub trait BinExt: IsA<Bin> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_bin_get_child")]
    #[doc(alias = "get_child")]
    fn child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_bin_get_child(self.as_ref().to_glib_none().0)) }
    }
}

impl<O: IsA<Bin>> BinExt for O {}
