// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Buildable;
use crate::IconSize;
use crate::Orientation;
use crate::ReliefStyle;
use crate::SizeGroup;
use crate::ToolbarStyle;
use crate::Widget;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkToolShell")]
    pub struct ToolShell(Interface<ffi::GtkToolShell, ffi::GtkToolShellIface>) @requires Widget, Buildable;

    match fn {
        type_ => || ffi::gtk_tool_shell_get_type(),
    }
}

impl ToolShell {
    pub const NONE: Option<&'static ToolShell> = None;
}

pub trait ToolShellExt: 'static {
    #[doc(alias = "gtk_tool_shell_get_ellipsize_mode")]
    #[doc(alias = "get_ellipsize_mode")]
    fn ellipsize_mode(&self) -> pango::EllipsizeMode;

    #[doc(alias = "gtk_tool_shell_get_icon_size")]
    #[doc(alias = "get_icon_size")]
    fn icon_size(&self) -> IconSize;

    #[doc(alias = "gtk_tool_shell_get_orientation")]
    #[doc(alias = "get_orientation")]
    fn orientation(&self) -> Orientation;

    #[doc(alias = "gtk_tool_shell_get_relief_style")]
    #[doc(alias = "get_relief_style")]
    fn relief_style(&self) -> ReliefStyle;

    #[doc(alias = "gtk_tool_shell_get_style")]
    #[doc(alias = "get_style")]
    fn style(&self) -> ToolbarStyle;

    #[doc(alias = "gtk_tool_shell_get_text_alignment")]
    #[doc(alias = "get_text_alignment")]
    fn text_alignment(&self) -> f32;

    #[doc(alias = "gtk_tool_shell_get_text_orientation")]
    #[doc(alias = "get_text_orientation")]
    fn text_orientation(&self) -> Orientation;

    #[doc(alias = "gtk_tool_shell_get_text_size_group")]
    #[doc(alias = "get_text_size_group")]
    fn text_size_group(&self) -> Option<SizeGroup>;

    #[doc(alias = "gtk_tool_shell_rebuild_menu")]
    fn rebuild_menu(&self);
}

impl<O: IsA<ToolShell>> ToolShellExt for O {
    fn ellipsize_mode(&self) -> pango::EllipsizeMode {
        unsafe {
            from_glib(ffi::gtk_tool_shell_get_ellipsize_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn icon_size(&self) -> IconSize {
        unsafe {
            from_glib(ffi::gtk_tool_shell_get_icon_size(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_tool_shell_get_orientation(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn relief_style(&self) -> ReliefStyle {
        unsafe {
            from_glib(ffi::gtk_tool_shell_get_relief_style(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn style(&self) -> ToolbarStyle {
        unsafe {
            from_glib(ffi::gtk_tool_shell_get_style(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn text_alignment(&self) -> f32 {
        unsafe { ffi::gtk_tool_shell_get_text_alignment(self.as_ref().to_glib_none().0) }
    }

    fn text_orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_tool_shell_get_text_orientation(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn text_size_group(&self) -> Option<SizeGroup> {
        unsafe {
            from_glib_none(ffi::gtk_tool_shell_get_text_size_group(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn rebuild_menu(&self) {
        unsafe {
            ffi::gtk_tool_shell_rebuild_menu(self.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for ToolShell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ToolShell")
    }
}
