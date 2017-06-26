// This file was generated by gir (3c73dd9) from gir-files (???)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    pub struct DOMHTMLUListElement(Object<ffi::WebKitDOMHTMLUListElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_u_list_element_get_type(),
    }
}

pub trait DOMHTMLUListElementExt {
    fn get_compact(&self) -> bool;

    fn get_type_attr(&self) -> Option<String>;

    fn set_compact(&self, value: bool);

    fn set_type_attr(&self, value: &str);

    fn get_property_type(&self) -> Option<String>;

    fn set_property_type(&self, type_: Option<&str>);
}

impl<O: IsA<DOMHTMLUListElement> + IsA<glib::object::Object>> DOMHTMLUListElementExt for O {
    fn get_compact(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_u_list_element_get_compact(self.to_glib_none().0))
        }
    }

    fn get_type_attr(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_u_list_element_get_type_attr(self.to_glib_none().0))
        }
    }

    fn set_compact(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_u_list_element_set_compact(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_type_attr(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_u_list_element_set_type_attr(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn get_property_type(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "type".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_type(&self, type_: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "type".to_glib_none().0, Value::from(type_).to_glib_none().0);
        }
    }
}
