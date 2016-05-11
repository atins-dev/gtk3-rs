// This file was generated by gir (8cacc34) from gir-files (11e0e6d)
// DO NOT EDIT

use Error;
use RecentData;
use RecentInfo;
use ffi;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct RecentManager(Object<ffi::GtkRecentManager>);

    match fn {
        get_type => || ffi::gtk_recent_manager_get_type(),
    }
}

impl RecentManager {
    pub fn new() -> RecentManager {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_recent_manager_new())
        }
    }

    pub fn add_full(&self, uri: &str, recent_data: &RecentData) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_manager_add_full(self.to_glib_none().0, uri.to_glib_none().0, recent_data.to_glib_none().0))
        }
    }

    pub fn add_item(&self, uri: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_manager_add_item(self.to_glib_none().0, uri.to_glib_none().0))
        }
    }

    pub fn get_items(&self) -> Vec<RecentInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_recent_manager_get_items(self.to_glib_none().0))
        }
    }

    pub fn has_item(&self, uri: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_manager_has_item(self.to_glib_none().0, uri.to_glib_none().0))
        }
    }

    pub fn lookup_item(&self, uri: &str) -> Result<RecentInfo, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_recent_manager_lookup_item(self.to_glib_none().0, uri.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn move_item(&self, uri: &str, new_uri: Option<&str>) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_recent_manager_move_item(self.to_glib_none().0, uri.to_glib_none().0, new_uri.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn purge_items(&self) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_recent_manager_purge_items(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn remove_item(&self, uri: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_recent_manager_remove_item(self.to_glib_none().0, uri.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_default() -> Option<RecentManager> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_recent_manager_get_default())
        }
    }

    pub fn connect_changed<F: Fn(&RecentManager) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&RecentManager) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_trampoline(this: *mut ffi::GtkRecentManager, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&RecentManager) + 'static> = transmute(f);
    f(&from_glib_none(this))
}
