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
    pub struct DOMHTMLFrameSetElement(Object<ffi::WebKitDOMHTMLFrameSetElement, ffi::WebKitDOMHTMLFrameSetElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_html_frame_set_element_get_type(),
    }
}

pub const NONE_DOMHTML_FRAME_SET_ELEMENT: Option<&DOMHTMLFrameSetElement> = None;

pub trait DOMHTMLFrameSetElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_frame_set_element_get_cols")]
    fn cols(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_frame_set_element_get_rows")]
    fn rows(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_frame_set_element_set_cols")]
    fn set_cols(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_frame_set_element_set_rows")]
    fn set_rows(&self, value: &str);

    fn connect_property_cols_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rows_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLFrameSetElement>> DOMHTMLFrameSetElementExt for O {
    fn cols(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_set_element_get_cols(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn rows(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_set_element_get_rows(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_cols(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_frame_set_element_set_cols(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_rows(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_frame_set_element_set_rows(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn connect_property_cols_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_cols_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMHTMLFrameSetElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DOMHTMLFrameSetElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLFrameSetElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::cols\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_cols_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_rows_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rows_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMHTMLFrameSetElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DOMHTMLFrameSetElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLFrameSetElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::rows\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_rows_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMHTMLFrameSetElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLFrameSetElement")
    }
}
