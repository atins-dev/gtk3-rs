// This file was generated by gir (8cacc34) from gir-files (11e0e6d)
// DO NOT EDIT

use Container;
use DirectionType;
use NotebookTab;
use PackType;
use PositionType;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct Notebook(Object<ffi::GtkNotebook>): Container, Widget;

    match fn {
        get_type => || ffi::gtk_notebook_get_type(),
    }
}

impl Notebook {
    pub fn new() -> Notebook {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_notebook_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn detach_tab<T: IsA<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_notebook_detach_tab(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    pub fn get_action_widget(&self, pack_type: PackType) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_action_widget(self.to_glib_none().0, pack_type.to_glib()))
        }
    }

    pub fn get_group_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_group_name(self.to_glib_none().0))
        }
    }

    pub fn get_menu_label<T: IsA<Widget>>(&self, child: &T) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_menu_label(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    pub fn get_menu_label_text<T: IsA<Widget>>(&self, child: &T) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_menu_label_text(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    pub fn get_scrollable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_scrollable(self.to_glib_none().0))
        }
    }

    pub fn get_show_border(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_show_border(self.to_glib_none().0))
        }
    }

    pub fn get_show_tabs(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_show_tabs(self.to_glib_none().0))
        }
    }

    pub fn get_tab_detachable<T: IsA<Widget>>(&self, child: &T) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_tab_detachable(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    pub fn get_tab_label<T: IsA<Widget>>(&self, child: &T) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_tab_label(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    pub fn get_tab_label_text<T: IsA<Widget>>(&self, child: &T) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_tab_label_text(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    pub fn get_tab_pos(&self) -> PositionType {
        unsafe {
            from_glib(ffi::gtk_notebook_get_tab_pos(self.to_glib_none().0))
        }
    }

    pub fn get_tab_reorderable<T: IsA<Widget>>(&self, child: &T) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_tab_reorderable(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    pub fn next_page(&self) {
        unsafe {
            ffi::gtk_notebook_next_page(self.to_glib_none().0);
        }
    }

    pub fn popup_disable(&self) {
        unsafe {
            ffi::gtk_notebook_popup_disable(self.to_glib_none().0);
        }
    }

    pub fn popup_enable(&self) {
        unsafe {
            ffi::gtk_notebook_popup_enable(self.to_glib_none().0);
        }
    }

    pub fn prev_page(&self) {
        unsafe {
            ffi::gtk_notebook_prev_page(self.to_glib_none().0);
        }
    }

    pub fn set_action_widget<T: IsA<Widget>>(&self, widget: &T, pack_type: PackType) {
        unsafe {
            ffi::gtk_notebook_set_action_widget(self.to_glib_none().0, widget.to_glib_none().0, pack_type.to_glib());
        }
    }

    pub fn set_group_name(&self, group_name: Option<&str>) {
        unsafe {
            ffi::gtk_notebook_set_group_name(self.to_glib_none().0, group_name.to_glib_none().0);
        }
    }

    pub fn set_menu_label<T: IsA<Widget>, U: IsA<Widget>>(&self, child: &T, menu_label: Option<&U>) {
        unsafe {
            ffi::gtk_notebook_set_menu_label(self.to_glib_none().0, child.to_glib_none().0, menu_label.to_glib_none().0);
        }
    }

    pub fn set_menu_label_text<T: IsA<Widget>>(&self, child: &T, menu_text: &str) {
        unsafe {
            ffi::gtk_notebook_set_menu_label_text(self.to_glib_none().0, child.to_glib_none().0, menu_text.to_glib_none().0);
        }
    }

    pub fn set_scrollable(&self, scrollable: bool) {
        unsafe {
            ffi::gtk_notebook_set_scrollable(self.to_glib_none().0, scrollable.to_glib());
        }
    }

    pub fn set_show_border(&self, show_border: bool) {
        unsafe {
            ffi::gtk_notebook_set_show_border(self.to_glib_none().0, show_border.to_glib());
        }
    }

    pub fn set_show_tabs(&self, show_tabs: bool) {
        unsafe {
            ffi::gtk_notebook_set_show_tabs(self.to_glib_none().0, show_tabs.to_glib());
        }
    }

    pub fn set_tab_detachable<T: IsA<Widget>>(&self, child: &T, detachable: bool) {
        unsafe {
            ffi::gtk_notebook_set_tab_detachable(self.to_glib_none().0, child.to_glib_none().0, detachable.to_glib());
        }
    }

    pub fn set_tab_label<T: IsA<Widget>, U: IsA<Widget>>(&self, child: &T, tab_label: Option<&U>) {
        unsafe {
            ffi::gtk_notebook_set_tab_label(self.to_glib_none().0, child.to_glib_none().0, tab_label.to_glib_none().0);
        }
    }

    pub fn set_tab_label_text<T: IsA<Widget>>(&self, child: &T, tab_text: &str) {
        unsafe {
            ffi::gtk_notebook_set_tab_label_text(self.to_glib_none().0, child.to_glib_none().0, tab_text.to_glib_none().0);
        }
    }

    pub fn set_tab_pos(&self, pos: PositionType) {
        unsafe {
            ffi::gtk_notebook_set_tab_pos(self.to_glib_none().0, pos.to_glib());
        }
    }

    pub fn set_tab_reorderable<T: IsA<Widget>>(&self, child: &T, reorderable: bool) {
        unsafe {
            ffi::gtk_notebook_set_tab_reorderable(self.to_glib_none().0, child.to_glib_none().0, reorderable.to_glib());
        }
    }

    pub fn connect_change_current_page<F: Fn(&Notebook, i32) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Notebook, i32) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "change-current-page",
                transmute(change_current_page_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_create_window<F: Fn(&Notebook, &Widget, i32, i32) -> Notebook + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Notebook, &Widget, i32, i32) -> Notebook + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "create-window",
                transmute(create_window_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_focus_tab<F: Fn(&Notebook, NotebookTab) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Notebook, NotebookTab) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "focus-tab",
                transmute(focus_tab_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_move_focus_out<F: Fn(&Notebook, DirectionType) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Notebook, DirectionType) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-focus-out",
                transmute(move_focus_out_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_page_added<F: Fn(&Notebook, &Widget, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Notebook, &Widget, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "page-added",
                transmute(page_added_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_page_removed<F: Fn(&Notebook, &Widget, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Notebook, &Widget, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "page-removed",
                transmute(page_removed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_page_reordered<F: Fn(&Notebook, &Widget, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Notebook, &Widget, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "page-reordered",
                transmute(page_reordered_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_reorder_tab<F: Fn(&Notebook, DirectionType, bool) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Notebook, DirectionType, bool) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "reorder-tab",
                transmute(reorder_tab_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_select_page<F: Fn(&Notebook, bool) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Notebook, bool) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "select-page",
                transmute(select_page_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_switch_page<F: Fn(&Notebook, &Widget, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Notebook, &Widget, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "switch-page",
                transmute(switch_page_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn change_current_page_trampoline(this: *mut ffi::GtkNotebook, object: libc::c_int, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&Notebook, i32) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), object).to_glib()
}

unsafe extern "C" fn create_window_trampoline(this: *mut ffi::GtkNotebook, page: *mut ffi::GtkWidget, x: libc::c_int, y: libc::c_int, f: glib_ffi::gpointer) -> *mut ffi::GtkNotebook {
    callback_guard!();
    let f: &Box_<Fn(&Notebook, &Widget, i32, i32) -> Notebook + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(page), x, y)/*Not checked*/.to_glib_none().0
}

unsafe extern "C" fn focus_tab_trampoline(this: *mut ffi::GtkNotebook, object: ffi::GtkNotebookTab, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&Notebook, NotebookTab) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(object)).to_glib()
}

unsafe extern "C" fn move_focus_out_trampoline(this: *mut ffi::GtkNotebook, object: ffi::GtkDirectionType, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Notebook, DirectionType) + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(object))
}

unsafe extern "C" fn page_added_trampoline(this: *mut ffi::GtkNotebook, child: *mut ffi::GtkWidget, page_num: libc::c_uint, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Notebook, &Widget, u32) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(child), page_num)
}

unsafe extern "C" fn page_removed_trampoline(this: *mut ffi::GtkNotebook, child: *mut ffi::GtkWidget, page_num: libc::c_uint, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Notebook, &Widget, u32) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(child), page_num)
}

unsafe extern "C" fn page_reordered_trampoline(this: *mut ffi::GtkNotebook, child: *mut ffi::GtkWidget, page_num: libc::c_uint, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Notebook, &Widget, u32) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(child), page_num)
}

unsafe extern "C" fn reorder_tab_trampoline(this: *mut ffi::GtkNotebook, object: ffi::GtkDirectionType, p0: glib_ffi::gboolean, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&Notebook, DirectionType, bool) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(object), from_glib(p0)).to_glib()
}

unsafe extern "C" fn select_page_trampoline(this: *mut ffi::GtkNotebook, object: glib_ffi::gboolean, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&Notebook, bool) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(object)).to_glib()
}

unsafe extern "C" fn switch_page_trampoline(this: *mut ffi::GtkNotebook, page: *mut ffi::GtkWidget, page_num: libc::c_uint, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Notebook, &Widget, u32) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(page), page_num)
}
