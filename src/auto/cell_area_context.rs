// This file was generated by gir (8cacc34) from gir-files (11e0e6d)
// DO NOT EDIT

use CellArea;
use ffi;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct CellAreaContext(Object<ffi::GtkCellAreaContext>);

    match fn {
        get_type => || ffi::gtk_cell_area_context_get_type(),
    }
}

impl CellAreaContext {
    pub fn allocate(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_cell_area_context_allocate(self.to_glib_none().0, width, height);
        }
    }

    pub fn get_allocation(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::gtk_cell_area_context_get_allocation(self.to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    pub fn get_area(&self) -> Option<CellArea> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_context_get_area(self.to_glib_none().0))
        }
    }

    pub fn get_preferred_height(&self) -> (i32, i32) {
        unsafe {
            let mut minimum_height = mem::uninitialized();
            let mut natural_height = mem::uninitialized();
            ffi::gtk_cell_area_context_get_preferred_height(self.to_glib_none().0, &mut minimum_height, &mut natural_height);
            (minimum_height, natural_height)
        }
    }

    pub fn get_preferred_height_for_width(&self, width: i32) -> (i32, i32) {
        unsafe {
            let mut minimum_height = mem::uninitialized();
            let mut natural_height = mem::uninitialized();
            ffi::gtk_cell_area_context_get_preferred_height_for_width(self.to_glib_none().0, width, &mut minimum_height, &mut natural_height);
            (minimum_height, natural_height)
        }
    }

    pub fn get_preferred_width(&self) -> (i32, i32) {
        unsafe {
            let mut minimum_width = mem::uninitialized();
            let mut natural_width = mem::uninitialized();
            ffi::gtk_cell_area_context_get_preferred_width(self.to_glib_none().0, &mut minimum_width, &mut natural_width);
            (minimum_width, natural_width)
        }
    }

    pub fn get_preferred_width_for_height(&self, height: i32) -> (i32, i32) {
        unsafe {
            let mut minimum_width = mem::uninitialized();
            let mut natural_width = mem::uninitialized();
            ffi::gtk_cell_area_context_get_preferred_width_for_height(self.to_glib_none().0, height, &mut minimum_width, &mut natural_width);
            (minimum_width, natural_width)
        }
    }

    pub fn push_preferred_height(&self, minimum_height: i32, natural_height: i32) {
        unsafe {
            ffi::gtk_cell_area_context_push_preferred_height(self.to_glib_none().0, minimum_height, natural_height);
        }
    }

    pub fn push_preferred_width(&self, minimum_width: i32, natural_width: i32) {
        unsafe {
            ffi::gtk_cell_area_context_push_preferred_width(self.to_glib_none().0, minimum_width, natural_width);
        }
    }

    pub fn reset(&self) {
        unsafe {
            ffi::gtk_cell_area_context_reset(self.to_glib_none().0);
        }
    }
}
