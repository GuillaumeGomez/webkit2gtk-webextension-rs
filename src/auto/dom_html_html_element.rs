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
    pub struct DOMHTMLHtmlElement(Object<ffi::WebKitDOMHTMLHtmlElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_html_element_get_type(),
    }
}

pub trait DOMHTMLHtmlElementExt {
    fn get_version(&self) -> Option<String>;

    fn set_version(&self, value: &str);
}

impl<O: IsA<DOMHTMLHtmlElement>> DOMHTMLHtmlElementExt for O {
    fn get_version(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_html_element_get_version(self.to_glib_none().0))
        }
    }

    fn set_version(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_html_element_set_version(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
