// This file was generated by gir (0f7cd61) from gir-files (0bcaef9)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLDListElement(Object<ffi::WebKitDOMHTMLDListElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_d_list_element_get_type(),
    }
}

pub trait DOMHTMLDListElementExt {
    fn get_compact(&self) -> bool;

    fn set_compact(&self, value: bool);
}

impl<O: IsA<DOMHTMLDListElement>> DOMHTMLDListElementExt for O {
    fn get_compact(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_d_list_element_get_compact(self.to_glib_none().0))
        }
    }

    fn set_compact(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_d_list_element_set_compact(self.to_glib_none().0, value.to_glib());
        }
    }
}
