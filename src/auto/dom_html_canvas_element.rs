// This file was generated by gir (d50d839) from gir-files (469db10)
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
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLCanvasElement(Object<ffi::WebKitDOMHTMLCanvasElement, ffi::WebKitDOMHTMLCanvasElementClass>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_canvas_element_get_type(),
    }
}

pub trait DOMHTMLCanvasElementExt {
    fn get_height(&self) -> libc::c_long;

    fn get_width(&self) -> libc::c_long;

    fn set_height(&self, value: libc::c_long);

    fn set_width(&self, value: libc::c_long);

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLCanvasElement> + IsA<glib::object::Object>> DOMHTMLCanvasElementExt for O {
    fn get_height(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_canvas_element_get_height(self.to_glib_none().0)
        }
    }

    fn get_width(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_canvas_element_get_width(self.to_glib_none().0)
        }
    }

    fn set_height(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_canvas_element_set_height(self.to_glib_none().0, value);
        }
    }

    fn set_width(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_canvas_element_set_width(self.to_glib_none().0, value);
        }
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::height",
                transmute(notify_height_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::width",
                transmute(notify_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_height_trampoline<P>(this: *mut ffi::WebKitDOMHTMLCanvasElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLCanvasElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLCanvasElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_width_trampoline<P>(this: *mut ffi::WebKitDOMHTMLCanvasElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLCanvasElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLCanvasElement::from_glib_borrow(this).downcast_unchecked())
}
