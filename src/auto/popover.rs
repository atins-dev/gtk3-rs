// This file was generated by gir (8cacc34) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Container;
use Object;
use PositionType;
use Rectangle;
use Widget;
use ffi;
#[cfg(feature = "v3_12")]
use gio;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct Popover(Object<ffi::GtkPopover>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_popover_get_type(),
    }
}

impl Popover {
    #[cfg(feature = "v3_12")]
    pub fn new<T: IsA<Widget>>(relative_to: Option<&T>) -> Popover {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_popover_new(relative_to.to_glib_none().0)).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn new_from_model<T: IsA<Widget>, U: IsA<gio::MenuModel>>(relative_to: Option<&T>, model: &U) -> Popover {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_popover_new_from_model(relative_to.to_glib_none().0, model.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait PopoverExt {
    #[cfg(feature = "v3_12")]
    fn bind_model<T: IsA<gio::MenuModel>>(&self, model: Option<&T>, action_namespace: Option<&str>);

    #[cfg(feature = "v3_12")]
    fn get_modal(&self) -> bool;

    fn get_pointing_to(&self) -> Option<Rectangle>;

    fn get_position(&self) -> PositionType;

    #[cfg(feature = "v3_12")]
    fn get_relative_to(&self) -> Option<Widget>;

    #[cfg(feature = "v3_16")]
    fn get_transitions_enabled(&self) -> bool;

    #[cfg(feature = "v3_12")]
    fn set_modal(&self, modal: bool);

    #[cfg(feature = "v3_12")]
    fn set_pointing_to(&self, rect: &Rectangle);

    #[cfg(feature = "v3_12")]
    fn set_position(&self, position: PositionType);

    #[cfg(feature = "v3_12")]
    fn set_relative_to<T: IsA<Widget>>(&self, relative_to: Option<&T>);

    #[cfg(feature = "v3_16")]
    fn set_transitions_enabled(&self, transitions_enabled: bool);

    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Popover> + IsA<Object>> PopoverExt for O {
    #[cfg(feature = "v3_12")]
    fn bind_model<T: IsA<gio::MenuModel>>(&self, model: Option<&T>, action_namespace: Option<&str>) {
        unsafe {
            ffi::gtk_popover_bind_model(self.to_glib_none().0, model.to_glib_none().0, action_namespace.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    fn get_modal(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_popover_get_modal(self.to_glib_none().0))
        }
    }

    fn get_pointing_to(&self) -> Option<Rectangle> {
        unsafe {
            let mut rect = Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_popover_get_pointing_to(self.to_glib_none().0, rect.to_glib_none_mut().0));
            if ret { Some(rect) } else { None }
        }
    }

    fn get_position(&self) -> PositionType {
        unsafe {
            from_glib(ffi::gtk_popover_get_position(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    fn get_relative_to(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_popover_get_relative_to(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    fn get_transitions_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_popover_get_transitions_enabled(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gtk_popover_set_modal(self.to_glib_none().0, modal.to_glib());
        }
    }

    #[cfg(feature = "v3_12")]
    fn set_pointing_to(&self, rect: &Rectangle) {
        unsafe {
            ffi::gtk_popover_set_pointing_to(self.to_glib_none().0, rect.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    fn set_position(&self, position: PositionType) {
        unsafe {
            ffi::gtk_popover_set_position(self.to_glib_none().0, position.to_glib());
        }
    }

    #[cfg(feature = "v3_12")]
    fn set_relative_to<T: IsA<Widget>>(&self, relative_to: Option<&T>) {
        unsafe {
            ffi::gtk_popover_set_relative_to(self.to_glib_none().0, relative_to.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_16")]
    fn set_transitions_enabled(&self, transitions_enabled: bool) {
        unsafe {
            ffi::gtk_popover_set_transitions_enabled(self.to_glib_none().0, transitions_enabled.to_glib());
        }
    }

    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "closed",
                transmute(closed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn closed_trampoline<T>(this: *mut ffi::GtkPopover, f: glib_ffi::gpointer)
where T: IsA<Popover> {
    callback_guard!();
    let f: &Box_<Fn(&T) + 'static> = transmute(f);
    f(&Popover::from_glib_none(this).downcast_unchecked())
}
