// This file was generated by gir (0f7cd61) from gir-files (0bcaef9)
// DO NOT EDIT

use DOMCSSRule;
use DOMObject;
use Error;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DOMCSSStyleDeclaration(Object<ffi::WebKitDOMCSSStyleDeclaration>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_css_style_declaration_get_type(),
    }
}

pub trait DOMCSSStyleDeclarationExt {
    fn get_css_text(&self) -> Option<String>;

    fn get_length(&self) -> libc::c_ulong;

    fn get_parent_rule(&self) -> Option<DOMCSSRule>;

    fn get_property_priority(&self, propertyName: &str) -> Option<String>;

    fn get_property_shorthand(&self, propertyName: &str) -> Option<String>;

    fn get_property_value(&self, propertyName: &str) -> Option<String>;

    fn is_property_implicit(&self, propertyName: &str) -> bool;

    fn item(&self, index: libc::c_ulong) -> Option<String>;

    fn remove_property(&self, propertyName: &str) -> Result<String, Error>;

    fn set_css_text(&self, value: &str) -> Result<(), Error>;

    fn set_property(&self, propertyName: &str, value: &str, priority: &str) -> Result<(), Error>;
}

impl<O: IsA<DOMCSSStyleDeclaration>> DOMCSSStyleDeclarationExt for O {
    fn get_css_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_declaration_get_css_text(self.to_glib_none().0))
        }
    }

    fn get_length(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_css_style_declaration_get_length(self.to_glib_none().0)
        }
    }

    fn get_parent_rule(&self) -> Option<DOMCSSRule> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_declaration_get_parent_rule(self.to_glib_none().0))
        }
    }

    fn get_property_priority(&self, propertyName: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_declaration_get_property_priority(self.to_glib_none().0, propertyName.to_glib_none().0))
        }
    }

    fn get_property_shorthand(&self, propertyName: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_declaration_get_property_shorthand(self.to_glib_none().0, propertyName.to_glib_none().0))
        }
    }

    fn get_property_value(&self, propertyName: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_declaration_get_property_value(self.to_glib_none().0, propertyName.to_glib_none().0))
        }
    }

    fn is_property_implicit(&self, propertyName: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_css_style_declaration_is_property_implicit(self.to_glib_none().0, propertyName.to_glib_none().0))
        }
    }

    fn item(&self, index: libc::c_ulong) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_declaration_item(self.to_glib_none().0, index))
        }
    }

    fn remove_property(&self, propertyName: &str) -> Result<String, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_css_style_declaration_remove_property(self.to_glib_none().0, propertyName.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_css_text(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_css_style_declaration_set_css_text(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_property(&self, propertyName: &str, value: &str, priority: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_css_style_declaration_set_property(self.to_glib_none().0, propertyName.to_glib_none().0, value.to_glib_none().0, priority.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}
