// This file was generated by gir (d50d839) from gir-files (469db10)
// DO NOT EDIT

use DOMCSSStyleSheet;
use DOMObject;
use Error;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
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
    pub struct DOMCSSRule(Object<ffi::WebKitDOMCSSRule, ffi::WebKitDOMCSSRuleClass>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_css_rule_get_type(),
    }
}

pub trait DOMCSSRuleExt {
    fn get_css_text(&self) -> Option<String>;

    fn get_parent_rule(&self) -> Option<DOMCSSRule>;

    fn get_parent_style_sheet(&self) -> Option<DOMCSSStyleSheet>;

    fn get_rule_type(&self) -> libc::c_ushort;

    fn set_css_text(&self, value: &str) -> Result<(), Error>;

    fn get_property_type(&self) -> u32;

    fn connect_property_css_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_parent_rule_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_parent_style_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMCSSRule> + IsA<glib::object::Object>> DOMCSSRuleExt for O {
    fn get_css_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_rule_get_css_text(self.to_glib_none().0))
        }
    }

    fn get_parent_rule(&self) -> Option<DOMCSSRule> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_rule_get_parent_rule(self.to_glib_none().0))
        }
    }

    fn get_parent_style_sheet(&self) -> Option<DOMCSSStyleSheet> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_rule_get_parent_style_sheet(self.to_glib_none().0))
        }
    }

    fn get_rule_type(&self) -> libc::c_ushort {
        unsafe {
            ffi::webkit_dom_css_rule_get_rule_type(self.to_glib_none().0)
        }
    }

    fn set_css_text(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_css_rule_set_css_text(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_property_type(&self) -> u32 {
        unsafe {
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, <u32 as StaticType>::static_type().to_glib());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "type".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_property_css_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::css-text",
                transmute(notify_css_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_parent_rule_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::parent-rule",
                transmute(notify_parent_rule_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_parent_style_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::parent-style-sheet",
                transmute(notify_parent_style_sheet_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::type",
                transmute(notify_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_css_text_trampoline<P>(this: *mut ffi::WebKitDOMCSSRule, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMCSSRule> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMCSSRule::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_parent_rule_trampoline<P>(this: *mut ffi::WebKitDOMCSSRule, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMCSSRule> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMCSSRule::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_parent_style_sheet_trampoline<P>(this: *mut ffi::WebKitDOMCSSRule, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMCSSRule> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMCSSRule::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_type_trampoline<P>(this: *mut ffi::WebKitDOMCSSRule, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMCSSRule> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMCSSRule::from_glib_borrow(this).downcast_unchecked())
}
