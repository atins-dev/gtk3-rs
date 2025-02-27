// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::TextBuffer;
use glib::object::IsA;
use glib::translate::*;
use std::mem;
use std::ptr;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SelectionData(Boxed<ffi::GtkSelectionData>);

    match fn {
        copy => |ptr| ffi::gtk_selection_data_copy(ptr),
        free => |ptr| ffi::gtk_selection_data_free(ptr),
        type_ => || ffi::gtk_selection_data_get_type(),
    }
}

impl SelectionData {
    #[doc(alias = "gtk_selection_data_get_data_type")]
    #[doc(alias = "get_data_type")]
    pub fn data_type(&self) -> gdk::Atom {
        unsafe { from_glib_none(ffi::gtk_selection_data_get_data_type(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_selection_data_get_display")]
    #[doc(alias = "get_display")]
    pub fn display(&self) -> Option<gdk::Display> {
        unsafe { from_glib_none(ffi::gtk_selection_data_get_display(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_selection_data_get_format")]
    #[doc(alias = "get_format")]
    pub fn format(&self) -> i32 {
        unsafe { ffi::gtk_selection_data_get_format(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_selection_data_get_length")]
    #[doc(alias = "get_length")]
    pub fn length(&self) -> i32 {
        unsafe { ffi::gtk_selection_data_get_length(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_selection_data_get_pixbuf")]
    #[doc(alias = "get_pixbuf")]
    pub fn pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe { from_glib_full(ffi::gtk_selection_data_get_pixbuf(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_selection_data_get_selection")]
    #[doc(alias = "get_selection")]
    pub fn selection(&self) -> gdk::Atom {
        unsafe { from_glib_none(ffi::gtk_selection_data_get_selection(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_selection_data_get_target")]
    #[doc(alias = "get_target")]
    pub fn target(&self) -> gdk::Atom {
        unsafe { from_glib_none(ffi::gtk_selection_data_get_target(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_selection_data_get_targets")]
    #[doc(alias = "get_targets")]
    pub fn targets(&self) -> Option<Vec<gdk::Atom>> {
        unsafe {
            let mut targets = ptr::null_mut();
            let mut n_atoms = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_selection_data_get_targets(
                self.to_glib_none().0,
                &mut targets,
                n_atoms.as_mut_ptr(),
            ));
            if ret {
                Some(FromGlibContainer::from_glib_container_num(
                    targets,
                    n_atoms.assume_init() as usize,
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_selection_data_get_text")]
    #[doc(alias = "get_text")]
    pub fn text(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::gtk_selection_data_get_text(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_selection_data_get_uris")]
    #[doc(alias = "get_uris")]
    pub fn uris(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_selection_data_get_uris(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_selection_data_set")]
    pub fn set(&self, type_: &gdk::Atom, format: i32, data: &[u8]) {
        let length = data.len() as i32;
        unsafe {
            ffi::gtk_selection_data_set(
                mut_override(self.to_glib_none().0),
                type_.to_glib_none().0,
                format,
                data.to_glib_none().0,
                length,
            );
        }
    }

    #[doc(alias = "gtk_selection_data_set_pixbuf")]
    pub fn set_pixbuf(&self, pixbuf: &gdk_pixbuf::Pixbuf) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_data_set_pixbuf(
                mut_override(self.to_glib_none().0),
                pixbuf.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_selection_data_set_text")]
    pub fn set_text(&self, str: &str) -> bool {
        let len = str.len() as i32;
        unsafe {
            from_glib(ffi::gtk_selection_data_set_text(
                mut_override(self.to_glib_none().0),
                str.to_glib_none().0,
                len,
            ))
        }
    }

    #[doc(alias = "gtk_selection_data_set_uris")]
    pub fn set_uris(&self, uris: &[&str]) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_data_set_uris(
                mut_override(self.to_glib_none().0),
                uris.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_selection_data_targets_include_image")]
    pub fn targets_include_image(&self, writable: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_data_targets_include_image(
                self.to_glib_none().0,
                writable.into_glib(),
            ))
        }
    }

    #[doc(alias = "gtk_selection_data_targets_include_rich_text")]
    pub fn targets_include_rich_text(&self, buffer: &impl IsA<TextBuffer>) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_data_targets_include_rich_text(
                self.to_glib_none().0,
                buffer.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_selection_data_targets_include_text")]
    pub fn targets_include_text(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_data_targets_include_text(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_selection_data_targets_include_uri")]
    pub fn targets_include_uri(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_data_targets_include_uri(
                self.to_glib_none().0,
            ))
        }
    }
}
