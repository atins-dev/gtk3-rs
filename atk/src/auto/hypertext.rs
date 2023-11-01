// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Hyperlink;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "AtkHypertext")]
    pub struct Hypertext(Interface<ffi::AtkHypertext, ffi::AtkHypertextIface>);

    match fn {
        type_ => || ffi::atk_hypertext_get_type(),
    }
}

impl Hypertext {
    pub const NONE: Option<&'static Hypertext> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Hypertext>> Sealed for T {}
}

pub trait HypertextExt: IsA<Hypertext> + sealed::Sealed + 'static {
    #[doc(alias = "atk_hypertext_get_link")]
    #[doc(alias = "get_link")]
    fn link(&self, link_index: i32) -> Option<Hyperlink> {
        unsafe {
            from_glib_none(ffi::atk_hypertext_get_link(
                self.as_ref().to_glib_none().0,
                link_index,
            ))
        }
    }

    #[doc(alias = "atk_hypertext_get_link_index")]
    #[doc(alias = "get_link_index")]
    fn link_index(&self, char_index: i32) -> i32 {
        unsafe { ffi::atk_hypertext_get_link_index(self.as_ref().to_glib_none().0, char_index) }
    }

    #[doc(alias = "atk_hypertext_get_n_links")]
    #[doc(alias = "get_n_links")]
    fn n_links(&self) -> i32 {
        unsafe { ffi::atk_hypertext_get_n_links(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "link-selected")]
    fn connect_link_selected<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn link_selected_trampoline<
            P: IsA<Hypertext>,
            F: Fn(&P, i32) + 'static,
        >(
            this: *mut ffi::AtkHypertext,
            arg1: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Hypertext::from_glib_borrow(this).unsafe_cast_ref(), arg1)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"link-selected\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    link_selected_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Hypertext>> HypertextExt for O {}
