// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_webextension_sys;
use DOMElement;
use DOMEventTarget;
use DOMHTMLCollection;
use DOMHTMLElement;
use DOMNode;
use DOMObject;

glib_wrapper! {
    pub struct DOMHTMLFormElement(Object<webkit2_webextension_sys::WebKitDOMHTMLFormElement, webkit2_webextension_sys::WebKitDOMHTMLFormElementClass, DOMHTMLFormElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_html_form_element_get_type(),
    }
}

pub const NONE_DOMHTML_FORM_ELEMENT: Option<&DOMHTMLFormElement> = None;

pub trait DOMHTMLFormElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_accept_charset(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_action(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_elements(&self) -> Option<DOMHTMLCollection>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_encoding(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_enctype(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_length(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_method(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_name(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_target(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn reset(&self);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_accept_charset(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_action(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_encoding(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_enctype(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_method(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_name(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_target(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn submit(&self);

    fn connect_property_accept_charset_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_elements_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_enctype_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_method_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLFormElement>> DOMHTMLFormElementExt for O {
    fn get_accept_charset(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_form_element_get_accept_charset(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_action(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_form_element_get_action(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_elements(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_form_element_get_elements(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_encoding(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_form_element_get_encoding(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_enctype(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_form_element_get_enctype(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_length(&self) -> libc::c_long {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_form_element_get_length(
                self.as_ref().to_glib_none().0,
            )
        }
    }

    fn get_method(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_form_element_get_method(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_form_element_get_name(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_target(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_form_element_get_target(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn reset(&self) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_form_element_reset(
                self.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_accept_charset(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_form_element_set_accept_charset(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_action(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_form_element_set_action(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_encoding(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_form_element_set_encoding(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_enctype(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_form_element_set_enctype(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_method(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_form_element_set_method(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_form_element_set_name(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_target(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_form_element_set_target(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn submit(&self) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_form_element_submit(
                self.as_ref().to_glib_none().0,
            );
        }
    }

    fn connect_property_accept_charset_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accept_charset_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLFormElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLFormElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLFormElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accept-charset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accept_charset_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_action_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLFormElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLFormElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLFormElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::action\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_action_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_elements_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_elements_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLFormElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLFormElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLFormElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::elements\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_elements_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_encoding_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLFormElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLFormElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLFormElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::encoding\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_encoding_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_enctype_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_enctype_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLFormElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLFormElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLFormElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enctype\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enctype_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_length_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLFormElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLFormElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLFormElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::length\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_length_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_method_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_method_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLFormElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLFormElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLFormElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::method\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_method_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLFormElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLFormElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLFormElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_target_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLFormElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLFormElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLFormElement::from_glib_borrow(this).unsafe_cast_ref())
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

impl fmt::Display for DOMHTMLFormElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLFormElement")
    }
}
