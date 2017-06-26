// This file was generated by gir (3c73dd9) from gir-files (???)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMHTMLHRElement(Object<ffi::WebKitDOMHTMLHRElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_hr_element_get_type(),
    }
}

pub trait DOMHTMLHRElementExt {
    fn get_align(&self) -> Option<String>;

    fn get_no_shade(&self) -> bool;

    fn get_size(&self) -> Option<String>;

    fn get_width(&self) -> Option<String>;

    fn set_align(&self, value: &str);

    fn set_no_shade(&self, value: bool);

    fn set_size(&self, value: &str);

    fn set_width(&self, value: &str);
}

impl<O: IsA<DOMHTMLHRElement>> DOMHTMLHRElementExt for O {
    fn get_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_hr_element_get_align(self.to_glib_none().0))
        }
    }

    fn get_no_shade(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_hr_element_get_no_shade(self.to_glib_none().0))
        }
    }

    fn get_size(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_hr_element_get_size(self.to_glib_none().0))
        }
    }

    fn get_width(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_hr_element_get_width(self.to_glib_none().0))
        }
    }

    fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_hr_element_set_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_no_shade(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_hr_element_set_no_shade(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_size(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_hr_element_set_size(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_width(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_hr_element_set_width(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
