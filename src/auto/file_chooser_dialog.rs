// This file was generated by gir (8cacc34) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Container;
use Dialog;
use FileChooser;
use Widget;
use Window;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct FileChooserDialog(Object<ffi::GtkFileChooserDialog>): Dialog, Window, Bin, Container, Widget, FileChooser;

    match fn {
        get_type => || ffi::gtk_file_chooser_dialog_get_type(),
    }
}

impl FileChooserDialog {
    //pub fn new<T: IsA<Window>>(title: Option<&str>, parent: Option<&T>, action: FileChooserAction, first_button_text: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> FileChooserDialog {
    //    unsafe { TODO: call ffi::gtk_file_chooser_dialog_new() }
    //}
}
