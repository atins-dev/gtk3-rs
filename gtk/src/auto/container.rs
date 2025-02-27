// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Adjustment;
use crate::Buildable;
use crate::ResizeMode;
use crate::Widget;
use crate::WidgetPath;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkContainer")]
    pub struct Container(Object<ffi::GtkContainer, ffi::GtkContainerClass>) @extends Widget, @implements Buildable;

    match fn {
        type_ => || ffi::gtk_container_get_type(),
    }
}

impl Container {
    pub const NONE: Option<&'static Container> = None;
}

pub trait ContainerExt: 'static {
    #[doc(alias = "gtk_container_add")]
    fn add(&self, widget: &impl IsA<Widget>);

    //#[doc(alias = "gtk_container_add_with_properties")]
    //fn add_with_properties(&self, widget: &impl IsA<Widget>, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[doc(alias = "gtk_container_check_resize")]
    fn check_resize(&self);

    //#[doc(alias = "gtk_container_child_get")]
    //fn child_get(&self, child: &impl IsA<Widget>, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //#[doc(alias = "gtk_container_child_get_valist")]
    //fn child_get_valist(&self, child: &impl IsA<Widget>, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    #[doc(alias = "gtk_container_child_notify")]
    fn child_notify(&self, child: &impl IsA<Widget>, child_property: &str);

    #[doc(alias = "gtk_container_child_notify_by_pspec")]
    fn child_notify_by_pspec(&self, child: &impl IsA<Widget>, pspec: impl AsRef<glib::ParamSpec>);

    //#[doc(alias = "gtk_container_child_set")]
    //fn child_set(&self, child: &impl IsA<Widget>, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //#[doc(alias = "gtk_container_child_set_valist")]
    //fn child_set_valist(&self, child: &impl IsA<Widget>, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    #[doc(alias = "gtk_container_child_type")]
    fn child_type(&self) -> glib::types::Type;

    #[doc(alias = "gtk_container_forall")]
    fn forall<P: FnMut(&Widget)>(&self, callback: P);

    #[doc(alias = "gtk_container_foreach")]
    fn foreach<P: FnMut(&Widget)>(&self, callback: P);

    #[doc(alias = "gtk_container_get_border_width")]
    #[doc(alias = "get_border_width")]
    fn border_width(&self) -> u32;

    #[doc(alias = "gtk_container_get_children")]
    #[doc(alias = "get_children")]
    fn children(&self) -> Vec<Widget>;

    //#[cfg_attr(feature = "v3_24", deprecated = "Since 3.24")]
    //#[doc(alias = "gtk_container_get_focus_chain")]
    //#[doc(alias = "get_focus_chain")]
    //fn focus_chain(&self, focusable_widgets: /*Unimplemented*/Vec<Widget>) -> bool;

    #[doc(alias = "gtk_container_get_focus_child")]
    #[doc(alias = "get_focus_child")]
    fn focus_child(&self) -> Option<Widget>;

    #[doc(alias = "gtk_container_get_focus_hadjustment")]
    #[doc(alias = "get_focus_hadjustment")]
    fn focus_hadjustment(&self) -> Option<Adjustment>;

    #[doc(alias = "gtk_container_get_focus_vadjustment")]
    #[doc(alias = "get_focus_vadjustment")]
    fn focus_vadjustment(&self) -> Option<Adjustment>;

    #[doc(alias = "gtk_container_get_path_for_child")]
    #[doc(alias = "get_path_for_child")]
    fn path_for_child(&self, child: &impl IsA<Widget>) -> Option<WidgetPath>;

    #[doc(alias = "gtk_container_propagate_draw")]
    fn propagate_draw(&self, child: &impl IsA<Widget>, cr: &cairo::Context);

    #[doc(alias = "gtk_container_remove")]
    fn remove(&self, widget: &impl IsA<Widget>);

    #[doc(alias = "gtk_container_set_border_width")]
    fn set_border_width(&self, border_width: u32);

    #[cfg_attr(feature = "v3_24", deprecated = "Since 3.24")]
    #[doc(alias = "gtk_container_set_focus_chain")]
    fn set_focus_chain(&self, focusable_widgets: &[Widget]);

    #[doc(alias = "gtk_container_set_focus_child")]
    fn set_focus_child(&self, child: Option<&impl IsA<Widget>>);

    #[doc(alias = "gtk_container_set_focus_hadjustment")]
    fn set_focus_hadjustment(&self, adjustment: &impl IsA<Adjustment>);

    #[doc(alias = "gtk_container_set_focus_vadjustment")]
    fn set_focus_vadjustment(&self, adjustment: &impl IsA<Adjustment>);

    #[cfg_attr(feature = "v3_24", deprecated = "Since 3.24")]
    #[doc(alias = "gtk_container_unset_focus_chain")]
    fn unset_focus_chain(&self);

    fn set_child<P: IsA<Widget>>(&self, child: Option<&P>);

    #[doc(alias = "resize-mode")]
    fn resize_mode(&self) -> ResizeMode;

    #[doc(alias = "resize-mode")]
    fn set_resize_mode(&self, resize_mode: ResizeMode);

    #[doc(alias = "add")]
    fn connect_add<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "check-resize")]
    fn connect_check_resize<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "remove")]
    fn connect_remove<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "set-focus-child")]
    fn connect_set_focus_child<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "border-width")]
    fn connect_border_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "child")]
    fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "resize-mode")]
    fn connect_resize_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Container>> ContainerExt for O {
    fn add(&self, widget: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_container_add(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            );
        }
    }

    //fn add_with_properties(&self, widget: &impl IsA<Widget>, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:gtk_container_add_with_properties() }
    //}

    fn check_resize(&self) {
        unsafe {
            ffi::gtk_container_check_resize(self.as_ref().to_glib_none().0);
        }
    }

    //fn child_get(&self, child: &impl IsA<Widget>, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:gtk_container_child_get() }
    //}

    //fn child_get_valist(&self, child: &impl IsA<Widget>, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:gtk_container_child_get_valist() }
    //}

    fn child_notify(&self, child: &impl IsA<Widget>, child_property: &str) {
        unsafe {
            ffi::gtk_container_child_notify(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                child_property.to_glib_none().0,
            );
        }
    }

    fn child_notify_by_pspec(&self, child: &impl IsA<Widget>, pspec: impl AsRef<glib::ParamSpec>) {
        unsafe {
            ffi::gtk_container_child_notify_by_pspec(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                pspec.as_ref().to_glib_none().0,
            );
        }
    }

    //fn child_set(&self, child: &impl IsA<Widget>, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:gtk_container_child_set() }
    //}

    //fn child_set_valist(&self, child: &impl IsA<Widget>, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:gtk_container_child_set_valist() }
    //}

    fn child_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gtk_container_child_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn forall<P: FnMut(&Widget)>(&self, callback: P) {
        let callback_data: P = callback;
        unsafe extern "C" fn callback_func<P: FnMut(&Widget)>(
            widget: *mut ffi::GtkWidget,
            data: glib::ffi::gpointer,
        ) {
            let widget = from_glib_borrow(widget);
            let callback: *mut P = data as *const _ as usize as *mut P;
            (*callback)(&widget);
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: &P = &callback_data;
        unsafe {
            ffi::gtk_container_forall(
                self.as_ref().to_glib_none().0,
                callback,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    fn foreach<P: FnMut(&Widget)>(&self, callback: P) {
        let callback_data: P = callback;
        unsafe extern "C" fn callback_func<P: FnMut(&Widget)>(
            widget: *mut ffi::GtkWidget,
            data: glib::ffi::gpointer,
        ) {
            let widget = from_glib_borrow(widget);
            let callback: *mut P = data as *const _ as usize as *mut P;
            (*callback)(&widget);
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: &P = &callback_data;
        unsafe {
            ffi::gtk_container_foreach(
                self.as_ref().to_glib_none().0,
                callback,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    fn border_width(&self) -> u32 {
        unsafe { ffi::gtk_container_get_border_width(self.as_ref().to_glib_none().0) }
    }

    fn children(&self) -> Vec<Widget> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_container_get_children(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn focus_chain(&self, focusable_widgets: /*Unimplemented*/Vec<Widget>) -> bool {
    //    unsafe { TODO: call ffi:gtk_container_get_focus_chain() }
    //}

    fn focus_child(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_container_get_focus_child(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn focus_hadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_container_get_focus_hadjustment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn focus_vadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_container_get_focus_vadjustment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn path_for_child(&self, child: &impl IsA<Widget>) -> Option<WidgetPath> {
        unsafe {
            from_glib_full(ffi::gtk_container_get_path_for_child(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    fn propagate_draw(&self, child: &impl IsA<Widget>, cr: &cairo::Context) {
        unsafe {
            ffi::gtk_container_propagate_draw(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                mut_override(cr.to_glib_none().0),
            );
        }
    }

    fn remove(&self, widget: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_container_remove(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_border_width(&self, border_width: u32) {
        unsafe {
            ffi::gtk_container_set_border_width(self.as_ref().to_glib_none().0, border_width);
        }
    }

    fn set_focus_chain(&self, focusable_widgets: &[Widget]) {
        unsafe {
            ffi::gtk_container_set_focus_chain(
                self.as_ref().to_glib_none().0,
                focusable_widgets.to_glib_none().0,
            );
        }
    }

    fn set_focus_child(&self, child: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_container_set_focus_child(
                self.as_ref().to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_focus_hadjustment(&self, adjustment: &impl IsA<Adjustment>) {
        unsafe {
            ffi::gtk_container_set_focus_hadjustment(
                self.as_ref().to_glib_none().0,
                adjustment.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_focus_vadjustment(&self, adjustment: &impl IsA<Adjustment>) {
        unsafe {
            ffi::gtk_container_set_focus_vadjustment(
                self.as_ref().to_glib_none().0,
                adjustment.as_ref().to_glib_none().0,
            );
        }
    }

    fn unset_focus_chain(&self) {
        unsafe {
            ffi::gtk_container_unset_focus_chain(self.as_ref().to_glib_none().0);
        }
    }

    fn set_child<P: IsA<Widget>>(&self, child: Option<&P>) {
        glib::ObjectExt::set_property(self.as_ref(), "child", &child)
    }

    fn resize_mode(&self) -> ResizeMode {
        glib::ObjectExt::property(self.as_ref(), "resize-mode")
    }

    fn set_resize_mode(&self, resize_mode: ResizeMode) {
        glib::ObjectExt::set_property(self.as_ref(), "resize-mode", &resize_mode)
    }

    fn connect_add<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn add_trampoline<P: IsA<Container>, F: Fn(&P, &Widget) + 'static>(
            this: *mut ffi::GtkContainer,
            object: *mut ffi::GtkWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Container::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"add\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    add_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_check_resize<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn check_resize_trampoline<P: IsA<Container>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkContainer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Container::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"check-resize\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    check_resize_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_remove<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn remove_trampoline<P: IsA<Container>, F: Fn(&P, &Widget) + 'static>(
            this: *mut ffi::GtkContainer,
            object: *mut ffi::GtkWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Container::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"remove\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    remove_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_set_focus_child<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn set_focus_child_trampoline<
            P: IsA<Container>,
            F: Fn(&P, &Widget) + 'static,
        >(
            this: *mut ffi::GtkContainer,
            object: *mut ffi::GtkWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Container::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"set-focus-child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    set_focus_child_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_border_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_border_width_trampoline<
            P: IsA<Container>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkContainer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Container::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::border-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_border_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<P: IsA<Container>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkContainer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Container::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_child_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_resize_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resize_mode_trampoline<
            P: IsA<Container>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkContainer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Container::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::resize-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resize_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Container {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Container")
    }
}
