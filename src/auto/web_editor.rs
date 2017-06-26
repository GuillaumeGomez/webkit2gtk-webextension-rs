// This file was generated by gir (3c73dd9) from gir-files (???)
// DO NOT EDIT

#[cfg(feature = "v2_10")]
use WebPage;
use ffi;
use glib;
#[cfg(feature = "v2_10")]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v2_10")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v2_10")]
use glib_ffi;
#[cfg(feature = "v2_10")]
use std::boxed::Box as Box_;
#[cfg(feature = "v2_10")]
use std::mem::transmute;

glib_wrapper! {
    pub struct WebEditor(Object<ffi::WebKitWebEditor>);

    match fn {
        get_type => || ffi::webkit_web_editor_get_type(),
    }
}

pub trait WebEditorExt {
    #[cfg(feature = "v2_10")]
    fn get_page(&self) -> Option<WebPage>;

    #[cfg(feature = "v2_10")]
    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<WebEditor> + IsA<glib::object::Object>> WebEditorExt for O {
    #[cfg(feature = "v2_10")]
    fn get_page(&self) -> Option<WebPage> {
        unsafe {
            from_glib_none(ffi::webkit_web_editor_get_page(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_10")]
    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "selection-changed",
                transmute(selection_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v2_10")]
unsafe extern "C" fn selection_changed_trampoline<P>(this: *mut ffi::WebKitWebEditor, f: glib_ffi::gpointer)
where P: IsA<WebEditor> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&WebEditor::from_glib_none(this).downcast_unchecked())
}
