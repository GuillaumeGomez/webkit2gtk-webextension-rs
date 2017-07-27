// This file was generated by gir (0f7cd61) from gir-files (0bcaef9)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLCollection;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use Error;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLTableSectionElement(Object<ffi::WebKitDOMHTMLTableSectionElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_table_section_element_get_type(),
    }
}

pub trait DOMHTMLTableSectionElementExt {
    fn delete_row(&self, index: libc::c_long) -> Result<(), Error>;

    fn get_align(&self) -> Option<String>;

    fn get_ch(&self) -> Option<String>;

    fn get_ch_off(&self) -> Option<String>;

    fn get_rows(&self) -> Option<DOMHTMLCollection>;

    fn get_v_align(&self) -> Option<String>;

    fn insert_row(&self, index: libc::c_long) -> Result<DOMHTMLElement, Error>;

    fn set_align(&self, value: &str);

    fn set_ch(&self, value: &str);

    fn set_ch_off(&self, value: &str);

    fn set_v_align(&self, value: &str);
}

impl<O: IsA<DOMHTMLTableSectionElement>> DOMHTMLTableSectionElementExt for O {
    fn delete_row(&self, index: libc::c_long) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_table_section_element_delete_row(self.to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_section_element_get_align(self.to_glib_none().0))
        }
    }

    fn get_ch(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_section_element_get_ch(self.to_glib_none().0))
        }
    }

    fn get_ch_off(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_section_element_get_ch_off(self.to_glib_none().0))
        }
    }

    fn get_rows(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_section_element_get_rows(self.to_glib_none().0))
        }
    }

    fn get_v_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_section_element_get_v_align(self.to_glib_none().0))
        }
    }

    fn insert_row(&self, index: libc::c_long) -> Result<DOMHTMLElement, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_html_table_section_element_insert_row(self.to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_section_element_set_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_ch(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_section_element_set_ch(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_ch_off(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_section_element_set_ch_off(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_v_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_section_element_set_v_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
