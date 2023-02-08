// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Cursor, Device, DeviceTool, Display, Event, GrabStatus, SeatCapabilities, Window};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GdkSeat")]
    pub struct Seat(Object<ffi::GdkSeat>);

    match fn {
        type_ => || ffi::gdk_seat_get_type(),
    }
}

impl Seat {
    pub const NONE: Option<&'static Seat> = None;
}

pub trait SeatExt: 'static {
    #[doc(alias = "gdk_seat_get_capabilities")]
    #[doc(alias = "get_capabilities")]
    fn capabilities(&self) -> SeatCapabilities;

    #[doc(alias = "gdk_seat_get_display")]
    #[doc(alias = "get_display")]
    fn display(&self) -> Option<Display>;

    #[doc(alias = "gdk_seat_get_keyboard")]
    #[doc(alias = "get_keyboard")]
    fn keyboard(&self) -> Option<Device>;

    #[doc(alias = "gdk_seat_get_pointer")]
    #[doc(alias = "get_pointer")]
    fn pointer(&self) -> Option<Device>;

    #[doc(alias = "gdk_seat_get_slaves")]
    #[doc(alias = "get_slaves")]
    fn slaves(&self, capabilities: SeatCapabilities) -> Vec<Device>;

    #[doc(alias = "gdk_seat_grab")]
    fn grab(
        &self,
        window: &Window,
        capabilities: SeatCapabilities,
        owner_events: bool,
        cursor: Option<&Cursor>,
        event: Option<&Event>,
        prepare_func: Option<&mut dyn (FnMut(&Seat, &Window))>,
    ) -> GrabStatus;

    #[doc(alias = "gdk_seat_ungrab")]
    fn ungrab(&self);

    #[doc(alias = "device-added")]
    fn connect_device_added<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "device-removed")]
    fn connect_device_removed<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "tool-added")]
    fn connect_tool_added<F: Fn(&Self, &DeviceTool) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "tool-removed")]
    fn connect_tool_removed<F: Fn(&Self, &DeviceTool) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Seat>> SeatExt for O {
    fn capabilities(&self) -> SeatCapabilities {
        unsafe {
            from_glib(ffi::gdk_seat_get_capabilities(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_seat_get_display(self.as_ref().to_glib_none().0)) }
    }

    fn keyboard(&self) -> Option<Device> {
        unsafe { from_glib_none(ffi::gdk_seat_get_keyboard(self.as_ref().to_glib_none().0)) }
    }

    fn pointer(&self) -> Option<Device> {
        unsafe { from_glib_none(ffi::gdk_seat_get_pointer(self.as_ref().to_glib_none().0)) }
    }

    fn slaves(&self, capabilities: SeatCapabilities) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_seat_get_slaves(
                self.as_ref().to_glib_none().0,
                capabilities.into_glib(),
            ))
        }
    }

    fn grab(
        &self,
        window: &Window,
        capabilities: SeatCapabilities,
        owner_events: bool,
        cursor: Option<&Cursor>,
        event: Option<&Event>,
        prepare_func: Option<&mut dyn (FnMut(&Seat, &Window))>,
    ) -> GrabStatus {
        let prepare_func_data: Option<&mut dyn (FnMut(&Seat, &Window))> = prepare_func;
        unsafe extern "C" fn prepare_func_func(
            seat: *mut ffi::GdkSeat,
            window: *mut ffi::GdkWindow,
            user_data: glib::ffi::gpointer,
        ) {
            let seat = from_glib_borrow(seat);
            let window = from_glib_borrow(window);
            let callback: *mut Option<&mut dyn (FnMut(&Seat, &Window))> =
                user_data as *const _ as usize as *mut Option<&mut dyn (FnMut(&Seat, &Window))>;
            if let Some(ref mut callback) = *callback {
                callback(&seat, &window)
            } else {
                panic!("cannot get closure...")
            }
        }
        let prepare_func = if prepare_func_data.is_some() {
            Some(prepare_func_func as _)
        } else {
            None
        };
        let super_callback0: &Option<&mut dyn (FnMut(&Seat, &Window))> = &prepare_func_data;
        unsafe {
            from_glib(ffi::gdk_seat_grab(
                self.as_ref().to_glib_none().0,
                window.to_glib_none().0,
                capabilities.into_glib(),
                owner_events.into_glib(),
                cursor.to_glib_none().0,
                event.to_glib_none().0,
                prepare_func,
                super_callback0 as *const _ as usize as *mut _,
            ))
        }
    }

    fn ungrab(&self) {
        unsafe {
            ffi::gdk_seat_ungrab(self.as_ref().to_glib_none().0);
        }
    }

    fn connect_device_added<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn device_added_trampoline<P: IsA<Seat>, F: Fn(&P, &Device) + 'static>(
            this: *mut ffi::GdkSeat,
            device: *mut ffi::GdkDevice,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Seat::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(device),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"device-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    device_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_device_removed<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn device_removed_trampoline<
            P: IsA<Seat>,
            F: Fn(&P, &Device) + 'static,
        >(
            this: *mut ffi::GdkSeat,
            device: *mut ffi::GdkDevice,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Seat::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(device),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"device-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    device_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_tool_added<F: Fn(&Self, &DeviceTool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn tool_added_trampoline<
            P: IsA<Seat>,
            F: Fn(&P, &DeviceTool) + 'static,
        >(
            this: *mut ffi::GdkSeat,
            tool: *mut ffi::GdkDeviceTool,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Seat::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(tool),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"tool-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    tool_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_tool_removed<F: Fn(&Self, &DeviceTool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn tool_removed_trampoline<
            P: IsA<Seat>,
            F: Fn(&P, &DeviceTool) + 'static,
        >(
            this: *mut ffi::GdkSeat,
            tool: *mut ffi::GdkDeviceTool,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Seat::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(tool),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"tool-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    tool_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Seat")
    }
}
