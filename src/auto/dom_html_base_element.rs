// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::DOMElement;
use crate::DOMEventTarget;
use crate::DOMHTMLElement;
use crate::DOMNode;
use crate::DOMObject;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct DOMHTMLBaseElement(Object<ffi::WebKitDOMHTMLBaseElement, ffi::WebKitDOMHTMLBaseElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_html_base_element_get_type(),
    }
}

pub const NONE_DOMHTML_BASE_ELEMENT: Option<&DOMHTMLBaseElement> = None;

pub trait DOMHTMLBaseElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_base_element_get_href")]
    fn href(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_base_element_get_target")]
    fn target(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_base_element_set_href")]
    fn set_href(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_base_element_set_target")]
    fn set_target(&self, value: &str);

    fn connect_property_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLBaseElement>> DOMHTMLBaseElementExt for O {
    fn href(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_base_element_get_href(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn target(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_base_element_get_target(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_href(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_base_element_set_href(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_target(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_base_element_set_target(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn connect_property_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_href_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMHTMLBaseElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DOMHTMLBaseElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLBaseElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::href\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_href_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_target_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMHTMLBaseElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DOMHTMLBaseElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLBaseElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::target\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_target_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMHTMLBaseElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLBaseElement")
    }
}
