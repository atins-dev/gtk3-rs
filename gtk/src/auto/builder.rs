// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Application;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    pub struct Builder(Object<ffi::GtkBuilder, ffi::GtkBuilderClass>);

    match fn {
        type_ => || ffi::gtk_builder_get_type(),
    }
}

impl Builder {
    #[doc(alias = "gtk_builder_new")]
    pub fn new() -> Builder {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_builder_new()) }
    }

    #[doc(alias = "gtk_builder_new_from_resource")]
    #[doc(alias = "new_from_resource")]
    pub fn from_resource(resource_path: &str) -> Builder {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_builder_new_from_resource(
                resource_path.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_builder_new_from_string")]
    #[doc(alias = "new_from_string")]
    pub fn from_string(string: &str) -> Builder {
        assert_initialized_main_thread!();
        let length = string.len() as isize;
        unsafe {
            from_glib_full(ffi::gtk_builder_new_from_string(
                string.to_glib_none().0,
                length,
            ))
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_BUILDER: Option<&Builder> = None;

pub trait BuilderExt: 'static {
    //#[doc(alias = "gtk_builder_add_callback_symbol")]
    //fn add_callback_symbol<P: FnOnce() + 'static>(&self, callback_name: &str, callback_symbol: P);

    //#[doc(alias = "gtk_builder_add_callback_symbols")]
    //fn add_callback_symbols<P: FnOnce() + 'static>(&self, first_callback_name: &str, first_callback_symbol: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[doc(alias = "gtk_builder_add_from_resource")]
    fn add_from_resource(&self, resource_path: &str) -> Result<(), glib::Error>;

    #[doc(alias = "gtk_builder_add_from_string")]
    fn add_from_string(&self, buffer: &str) -> Result<(), glib::Error>;

    #[doc(alias = "gtk_builder_add_objects_from_resource")]
    fn add_objects_from_resource(
        &self,
        resource_path: &str,
        object_ids: &[&str],
    ) -> Result<(), glib::Error>;

    #[doc(alias = "gtk_builder_add_objects_from_string")]
    fn add_objects_from_string(&self, buffer: &str, object_ids: &[&str])
        -> Result<(), glib::Error>;

    //#[doc(alias = "gtk_builder_connect_signals")]
    //fn connect_signals(&self, user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    #[doc(alias = "gtk_builder_expose_object")]
    fn expose_object<P: IsA<glib::Object>>(&self, name: &str, object: &P);

    #[doc(alias = "gtk_builder_extend_with_template")]
    fn extend_with_template<P: IsA<Widget>>(
        &self,
        widget: &P,
        template_type: glib::types::Type,
        buffer: &str,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "gtk_builder_get_application")]
    #[doc(alias = "get_application")]
    fn application(&self) -> Option<Application>;

    #[doc(alias = "gtk_builder_get_objects")]
    #[doc(alias = "get_objects")]
    fn objects(&self) -> Vec<glib::Object>;

    #[doc(alias = "gtk_builder_get_translation_domain")]
    #[doc(alias = "get_translation_domain")]
    fn translation_domain(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_builder_get_type_from_name")]
    #[doc(alias = "get_type_from_name")]
    fn type_from_name(&self, type_name: &str) -> glib::types::Type;

    //#[doc(alias = "gtk_builder_lookup_callback_symbol")]
    //fn lookup_callback_symbol(&self, callback_name: &str) -> Option<Box_<dyn Fn() + 'static>>;

    #[doc(alias = "gtk_builder_set_application")]
    fn set_application<P: IsA<Application>>(&self, application: &P);

    #[doc(alias = "gtk_builder_set_translation_domain")]
    fn set_translation_domain(&self, domain: Option<&str>);

    #[doc(alias = "gtk_builder_value_from_string")]
    fn value_from_string(
        &self,
        pspec: &glib::ParamSpec,
        string: &str,
    ) -> Result<glib::Value, glib::Error>;

    #[doc(alias = "gtk_builder_value_from_string_type")]
    fn value_from_string_type(
        &self,
        type_: glib::types::Type,
        string: &str,
    ) -> Result<glib::Value, glib::Error>;

    #[doc(alias = "translation-domain")]
    fn connect_translation_domain_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Builder>> BuilderExt for O {
    //fn add_callback_symbol<P: FnOnce() + 'static>(&self, callback_name: &str, callback_symbol: P) {
    //    unsafe { TODO: call ffi:gtk_builder_add_callback_symbol() }
    //}

    //fn add_callback_symbols<P: FnOnce() + 'static>(&self, first_callback_name: &str, first_callback_symbol: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:gtk_builder_add_callback_symbols() }
    //}

    fn add_from_resource(&self, resource_path: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_builder_add_from_resource(
                self.as_ref().to_glib_none().0,
                resource_path.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn add_from_string(&self, buffer: &str) -> Result<(), glib::Error> {
        let length = buffer.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_builder_add_from_string(
                self.as_ref().to_glib_none().0,
                buffer.to_glib_none().0,
                length,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn add_objects_from_resource(
        &self,
        resource_path: &str,
        object_ids: &[&str],
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_builder_add_objects_from_resource(
                self.as_ref().to_glib_none().0,
                resource_path.to_glib_none().0,
                object_ids.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn add_objects_from_string(
        &self,
        buffer: &str,
        object_ids: &[&str],
    ) -> Result<(), glib::Error> {
        let length = buffer.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_builder_add_objects_from_string(
                self.as_ref().to_glib_none().0,
                buffer.to_glib_none().0,
                length,
                object_ids.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //fn connect_signals(&self, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi:gtk_builder_connect_signals() }
    //}

    fn expose_object<P: IsA<glib::Object>>(&self, name: &str, object: &P) {
        unsafe {
            ffi::gtk_builder_expose_object(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                object.as_ref().to_glib_none().0,
            );
        }
    }

    fn extend_with_template<P: IsA<Widget>>(
        &self,
        widget: &P,
        template_type: glib::types::Type,
        buffer: &str,
    ) -> Result<(), glib::Error> {
        let length = buffer.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_builder_extend_with_template(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                template_type.into_glib(),
                buffer.to_glib_none().0,
                length,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn application(&self) -> Option<Application> {
        unsafe {
            from_glib_none(ffi::gtk_builder_get_application(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn objects(&self) -> Vec<glib::Object> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_builder_get_objects(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn translation_domain(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_builder_get_translation_domain(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn type_from_name(&self, type_name: &str) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gtk_builder_get_type_from_name(
                self.as_ref().to_glib_none().0,
                type_name.to_glib_none().0,
            ))
        }
    }

    //fn lookup_callback_symbol(&self, callback_name: &str) -> Option<Box_<dyn Fn() + 'static>> {
    //    unsafe { TODO: call ffi:gtk_builder_lookup_callback_symbol() }
    //}

    fn set_application<P: IsA<Application>>(&self, application: &P) {
        unsafe {
            ffi::gtk_builder_set_application(
                self.as_ref().to_glib_none().0,
                application.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_translation_domain(&self, domain: Option<&str>) {
        unsafe {
            ffi::gtk_builder_set_translation_domain(
                self.as_ref().to_glib_none().0,
                domain.to_glib_none().0,
            );
        }
    }

    fn value_from_string(
        &self,
        pspec: &glib::ParamSpec,
        string: &str,
    ) -> Result<glib::Value, glib::Error> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_builder_value_from_string(
                self.as_ref().to_glib_none().0,
                pspec.to_glib_none().0,
                string.to_glib_none().0,
                value.to_glib_none_mut().0,
                &mut error,
            );
            if error.is_null() {
                Ok(value)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn value_from_string_type(
        &self,
        type_: glib::types::Type,
        string: &str,
    ) -> Result<glib::Value, glib::Error> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_builder_value_from_string_type(
                self.as_ref().to_glib_none().0,
                type_.into_glib(),
                string.to_glib_none().0,
                value.to_glib_none_mut().0,
                &mut error,
            );
            if error.is_null() {
                Ok(value)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "translation-domain")]
    fn connect_translation_domain_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_translation_domain_trampoline<
            P: IsA<Builder>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkBuilder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Builder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::translation-domain\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_translation_domain_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Builder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Builder")
    }
}
