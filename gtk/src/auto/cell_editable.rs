// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Buildable, Widget};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkCellEditable")]
    pub struct CellEditable(Interface<ffi::GtkCellEditable, ffi::GtkCellEditableIface>) @requires Widget, Buildable;

    match fn {
        type_ => || ffi::gtk_cell_editable_get_type(),
    }
}

impl CellEditable {
    pub const NONE: Option<&'static CellEditable> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::CellEditable>> Sealed for T {}
}

pub trait CellEditableExt: IsA<CellEditable> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_cell_editable_editing_done")]
    fn editing_done(&self) {
        unsafe {
            ffi::gtk_cell_editable_editing_done(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_cell_editable_remove_widget")]
    fn remove_widget(&self) {
        unsafe {
            ffi::gtk_cell_editable_remove_widget(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_cell_editable_start_editing")]
    fn start_editing(&self, event: Option<&gdk::Event>) {
        unsafe {
            ffi::gtk_cell_editable_start_editing(
                self.as_ref().to_glib_none().0,
                mut_override(event.to_glib_none().0),
            );
        }
    }

    #[doc(alias = "editing-canceled")]
    fn is_editing_canceled(&self) -> bool {
        ObjectExt::property(self.as_ref(), "editing-canceled")
    }

    #[doc(alias = "editing-canceled")]
    fn set_editing_canceled(&self, editing_canceled: bool) {
        ObjectExt::set_property(self.as_ref(), "editing-canceled", editing_canceled)
    }

    #[doc(alias = "editing-done")]
    fn connect_editing_done<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn editing_done_trampoline<P: IsA<CellEditable>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellEditable,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellEditable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"editing-done\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    editing_done_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "remove-widget")]
    fn connect_remove_widget<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn remove_widget_trampoline<P: IsA<CellEditable>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellEditable,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellEditable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"remove-widget\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    remove_widget_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "editing-canceled")]
    fn connect_editing_canceled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_editing_canceled_trampoline<
            P: IsA<CellEditable>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellEditable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellEditable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::editing-canceled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_editing_canceled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<CellEditable>> CellEditableExt for O {}
