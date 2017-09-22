// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLFontElement(Object<ffi::WebKitDOMHTMLFontElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_font_element_get_type(),
    }
}

pub trait DOMHTMLFontElementExt {
    fn get_color(&self) -> Option<String>;

    fn get_face(&self) -> Option<String>;

    fn get_size(&self) -> Option<String>;

    fn set_color(&self, value: &str);

    fn set_face(&self, value: &str);

    fn set_size(&self, value: &str);

    fn connect_property_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_face_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLFontElement> + IsA<glib::object::Object>> DOMHTMLFontElementExt for O {
    fn get_color(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_font_element_get_color(self.to_glib_none().0))
        }
    }

    fn get_face(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_font_element_get_face(self.to_glib_none().0))
        }
    }

    fn get_size(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_font_element_get_size(self.to_glib_none().0))
        }
    }

    fn set_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_font_element_set_color(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_face(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_font_element_set_face(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_size(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_font_element_set_size(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::color",
                transmute(notify_color_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_face_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::face",
                transmute(notify_face_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::size",
                transmute(notify_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_color_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFontElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFontElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFontElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_face_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFontElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFontElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFontElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_size_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFontElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFontElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFontElement::from_glib_borrow(this).downcast_unchecked())
}
