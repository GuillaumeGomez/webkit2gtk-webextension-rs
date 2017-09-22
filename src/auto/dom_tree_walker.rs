// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use DOMNode;
use DOMObject;
use Error;
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
    pub struct DOMTreeWalker(Object<ffi::WebKitDOMTreeWalker>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_tree_walker_get_type(),
    }
}

pub trait DOMTreeWalkerExt {
    fn first_child(&self) -> Option<DOMNode>;

    fn get_current_node(&self) -> Option<DOMNode>;

    fn get_expand_entity_references(&self) -> bool;

    //fn get_filter(&self) -> /*Ignored*/Option<DOMNodeFilter>;

    fn get_root(&self) -> Option<DOMNode>;

    fn get_what_to_show(&self) -> libc::c_ulong;

    fn last_child(&self) -> Option<DOMNode>;

    fn next_node(&self) -> Option<DOMNode>;

    fn next_sibling(&self) -> Option<DOMNode>;

    fn parent_node(&self) -> Option<DOMNode>;

    fn previous_node(&self) -> Option<DOMNode>;

    fn previous_sibling(&self) -> Option<DOMNode>;

    fn set_current_node<P: IsA<DOMNode>>(&self, value: &P) -> Result<(), Error>;

    fn connect_property_current_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_root_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_what_to_show_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMTreeWalker> + IsA<glib::object::Object>> DOMTreeWalkerExt for O {
    fn first_child(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_tree_walker_first_child(self.to_glib_none().0))
        }
    }

    fn get_current_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_tree_walker_get_current_node(self.to_glib_none().0))
        }
    }

    fn get_expand_entity_references(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_tree_walker_get_expand_entity_references(self.to_glib_none().0))
        }
    }

    //fn get_filter(&self) -> /*Ignored*/Option<DOMNodeFilter> {
    //    unsafe { TODO: call ffi::webkit_dom_tree_walker_get_filter() }
    //}

    fn get_root(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_tree_walker_get_root(self.to_glib_none().0))
        }
    }

    fn get_what_to_show(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_tree_walker_get_what_to_show(self.to_glib_none().0)
        }
    }

    fn last_child(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_tree_walker_last_child(self.to_glib_none().0))
        }
    }

    fn next_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_tree_walker_next_node(self.to_glib_none().0))
        }
    }

    fn next_sibling(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_tree_walker_next_sibling(self.to_glib_none().0))
        }
    }

    fn parent_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_tree_walker_parent_node(self.to_glib_none().0))
        }
    }

    fn previous_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_tree_walker_previous_node(self.to_glib_none().0))
        }
    }

    fn previous_sibling(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_tree_walker_previous_sibling(self.to_glib_none().0))
        }
    }

    fn set_current_node<P: IsA<DOMNode>>(&self, value: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_tree_walker_set_current_node(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_property_current_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::current-node",
                transmute(notify_current_node_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::filter",
                transmute(notify_filter_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_root_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::root",
                transmute(notify_root_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_what_to_show_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::what-to-show",
                transmute(notify_what_to_show_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_current_node_trampoline<P>(this: *mut ffi::WebKitDOMTreeWalker, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMTreeWalker> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMTreeWalker::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_filter_trampoline<P>(this: *mut ffi::WebKitDOMTreeWalker, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMTreeWalker> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMTreeWalker::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_root_trampoline<P>(this: *mut ffi::WebKitDOMTreeWalker, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMTreeWalker> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMTreeWalker::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_what_to_show_trampoline<P>(this: *mut ffi::WebKitDOMTreeWalker, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMTreeWalker> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMTreeWalker::from_glib_borrow(this).downcast_unchecked())
}
