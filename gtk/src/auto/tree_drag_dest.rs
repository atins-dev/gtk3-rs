// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::SelectionData;
use crate::TreePath;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkTreeDragDest")]
    pub struct TreeDragDest(Interface<ffi::GtkTreeDragDest, ffi::GtkTreeDragDestIface>);

    match fn {
        type_ => || ffi::gtk_tree_drag_dest_get_type(),
    }
}

impl TreeDragDest {
    pub const NONE: Option<&'static TreeDragDest> = None;
}

pub trait TreeDragDestExt: 'static {
    #[doc(alias = "gtk_tree_drag_dest_drag_data_received")]
    fn drag_data_received(&self, dest: &mut TreePath, selection_data: &mut SelectionData) -> bool;

    #[doc(alias = "gtk_tree_drag_dest_row_drop_possible")]
    fn row_drop_possible(
        &self,
        dest_path: &mut TreePath,
        selection_data: &mut SelectionData,
    ) -> bool;
}

impl<O: IsA<TreeDragDest>> TreeDragDestExt for O {
    fn drag_data_received(&self, dest: &mut TreePath, selection_data: &mut SelectionData) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_drag_dest_drag_data_received(
                self.as_ref().to_glib_none().0,
                dest.to_glib_none_mut().0,
                selection_data.to_glib_none_mut().0,
            ))
        }
    }

    fn row_drop_possible(
        &self,
        dest_path: &mut TreePath,
        selection_data: &mut SelectionData,
    ) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_drag_dest_row_drop_possible(
                self.as_ref().to_glib_none().0,
                dest_path.to_glib_none_mut().0,
                selection_data.to_glib_none_mut().0,
            ))
        }
    }
}

impl fmt::Display for TreeDragDest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TreeDragDest")
    }
}
