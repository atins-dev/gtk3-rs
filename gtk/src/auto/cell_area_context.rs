// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::CellArea;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkCellAreaContext")]
    pub struct CellAreaContext(Object<ffi::GtkCellAreaContext, ffi::GtkCellAreaContextClass>);

    match fn {
        type_ => || ffi::gtk_cell_area_context_get_type(),
    }
}

impl CellAreaContext {
    pub const NONE: Option<&'static CellAreaContext> = None;
}

pub trait CellAreaContextExt: 'static {
    #[doc(alias = "gtk_cell_area_context_allocate")]
    fn allocate(&self, width: i32, height: i32);

    #[doc(alias = "gtk_cell_area_context_get_allocation")]
    #[doc(alias = "get_allocation")]
    fn allocation(&self) -> (i32, i32);

    #[doc(alias = "gtk_cell_area_context_get_area")]
    #[doc(alias = "get_area")]
    fn area(&self) -> Option<CellArea>;

    #[doc(alias = "gtk_cell_area_context_get_preferred_height")]
    #[doc(alias = "get_preferred_height")]
    fn preferred_height(&self) -> (i32, i32);

    #[doc(alias = "gtk_cell_area_context_get_preferred_height_for_width")]
    #[doc(alias = "get_preferred_height_for_width")]
    fn preferred_height_for_width(&self, width: i32) -> (i32, i32);

    #[doc(alias = "gtk_cell_area_context_get_preferred_width")]
    #[doc(alias = "get_preferred_width")]
    fn preferred_width(&self) -> (i32, i32);

    #[doc(alias = "gtk_cell_area_context_get_preferred_width_for_height")]
    #[doc(alias = "get_preferred_width_for_height")]
    fn preferred_width_for_height(&self, height: i32) -> (i32, i32);

    #[doc(alias = "gtk_cell_area_context_push_preferred_height")]
    fn push_preferred_height(&self, minimum_height: i32, natural_height: i32);

    #[doc(alias = "gtk_cell_area_context_push_preferred_width")]
    fn push_preferred_width(&self, minimum_width: i32, natural_width: i32);

    #[doc(alias = "gtk_cell_area_context_reset")]
    fn reset(&self);

    #[doc(alias = "minimum-height")]
    fn minimum_height(&self) -> i32;

    #[doc(alias = "minimum-width")]
    fn minimum_width(&self) -> i32;

    #[doc(alias = "natural-height")]
    fn natural_height(&self) -> i32;

    #[doc(alias = "natural-width")]
    fn natural_width(&self) -> i32;

    #[doc(alias = "minimum-height")]
    fn connect_minimum_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "minimum-width")]
    fn connect_minimum_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "natural-height")]
    fn connect_natural_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "natural-width")]
    fn connect_natural_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellAreaContext>> CellAreaContextExt for O {
    fn allocate(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_cell_area_context_allocate(self.as_ref().to_glib_none().0, width, height);
        }
    }

    fn allocation(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::gtk_cell_area_context_get_allocation(
                self.as_ref().to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            (width.assume_init(), height.assume_init())
        }
    }

    fn area(&self) -> Option<CellArea> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_context_get_area(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn preferred_height(&self) -> (i32, i32) {
        unsafe {
            let mut minimum_height = mem::MaybeUninit::uninit();
            let mut natural_height = mem::MaybeUninit::uninit();
            ffi::gtk_cell_area_context_get_preferred_height(
                self.as_ref().to_glib_none().0,
                minimum_height.as_mut_ptr(),
                natural_height.as_mut_ptr(),
            );
            (minimum_height.assume_init(), natural_height.assume_init())
        }
    }

    fn preferred_height_for_width(&self, width: i32) -> (i32, i32) {
        unsafe {
            let mut minimum_height = mem::MaybeUninit::uninit();
            let mut natural_height = mem::MaybeUninit::uninit();
            ffi::gtk_cell_area_context_get_preferred_height_for_width(
                self.as_ref().to_glib_none().0,
                width,
                minimum_height.as_mut_ptr(),
                natural_height.as_mut_ptr(),
            );
            (minimum_height.assume_init(), natural_height.assume_init())
        }
    }

    fn preferred_width(&self) -> (i32, i32) {
        unsafe {
            let mut minimum_width = mem::MaybeUninit::uninit();
            let mut natural_width = mem::MaybeUninit::uninit();
            ffi::gtk_cell_area_context_get_preferred_width(
                self.as_ref().to_glib_none().0,
                minimum_width.as_mut_ptr(),
                natural_width.as_mut_ptr(),
            );
            (minimum_width.assume_init(), natural_width.assume_init())
        }
    }

    fn preferred_width_for_height(&self, height: i32) -> (i32, i32) {
        unsafe {
            let mut minimum_width = mem::MaybeUninit::uninit();
            let mut natural_width = mem::MaybeUninit::uninit();
            ffi::gtk_cell_area_context_get_preferred_width_for_height(
                self.as_ref().to_glib_none().0,
                height,
                minimum_width.as_mut_ptr(),
                natural_width.as_mut_ptr(),
            );
            (minimum_width.assume_init(), natural_width.assume_init())
        }
    }

    fn push_preferred_height(&self, minimum_height: i32, natural_height: i32) {
        unsafe {
            ffi::gtk_cell_area_context_push_preferred_height(
                self.as_ref().to_glib_none().0,
                minimum_height,
                natural_height,
            );
        }
    }

    fn push_preferred_width(&self, minimum_width: i32, natural_width: i32) {
        unsafe {
            ffi::gtk_cell_area_context_push_preferred_width(
                self.as_ref().to_glib_none().0,
                minimum_width,
                natural_width,
            );
        }
    }

    fn reset(&self) {
        unsafe {
            ffi::gtk_cell_area_context_reset(self.as_ref().to_glib_none().0);
        }
    }

    fn minimum_height(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "minimum-height")
    }

    fn minimum_width(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "minimum-width")
    }

    fn natural_height(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "natural-height")
    }

    fn natural_width(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "natural-width")
    }

    fn connect_minimum_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_minimum_height_trampoline<
            P: IsA<CellAreaContext>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellAreaContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellAreaContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::minimum-height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_minimum_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_minimum_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_minimum_width_trampoline<
            P: IsA<CellAreaContext>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellAreaContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellAreaContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::minimum-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_minimum_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_natural_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_natural_height_trampoline<
            P: IsA<CellAreaContext>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellAreaContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellAreaContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::natural-height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_natural_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_natural_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_natural_width_trampoline<
            P: IsA<CellAreaContext>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellAreaContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellAreaContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::natural-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_natural_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CellAreaContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellAreaContext")
    }
}
