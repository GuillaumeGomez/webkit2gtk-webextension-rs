// This file was generated by gir (3c73dd9) from gir-files (???)
// DO NOT EDIT

use DOMObject;
use DOMStyleSheet;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use libc;

glib_wrapper! {
    pub struct DOMStyleSheetList(Object<ffi::WebKitDOMStyleSheetList>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_style_sheet_list_get_type(),
    }
}

pub trait DOMStyleSheetListExt {
    fn get_length(&self) -> libc::c_ulong;

    fn item(&self, index: libc::c_ulong) -> Option<DOMStyleSheet>;
}

impl<O: IsA<DOMStyleSheetList>> DOMStyleSheetListExt for O {
    fn get_length(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_style_sheet_list_get_length(self.to_glib_none().0)
        }
    }

    fn item(&self, index: libc::c_ulong) -> Option<DOMStyleSheet> {
        unsafe {
            from_glib_full(ffi::webkit_dom_style_sheet_list_item(self.to_glib_none().0, index))
        }
    }
}
