// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

#[cfg(feature = "v2_12")]
use ConsoleMessage;
#[cfg(feature = "v2_8")]
use ContextMenu;
use DOMDocument;
#[cfg(feature = "v2_2")]
use Frame;
use URIRequest;
use URIResponse;
#[cfg(feature = "v2_10")]
use WebEditor;
#[cfg(feature = "v2_8")]
use WebHitTestResult;
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
    pub struct WebPage(Object<ffi::WebKitWebPage>);

    match fn {
        get_type => || ffi::webkit_web_page_get_type(),
    }
}

pub trait WebPageExt {
    fn get_dom_document(&self) -> Option<DOMDocument>;

    #[cfg(feature = "v2_10")]
    fn get_editor(&self) -> Option<WebEditor>;

    fn get_id(&self) -> u64;

    #[cfg(feature = "v2_2")]
    fn get_main_frame(&self) -> Option<Frame>;

    fn get_uri(&self) -> Option<String>;

    #[cfg(feature = "v2_12")]
    fn connect_console_message_sent<F: Fn(&Self, &ConsoleMessage) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(feature = "v2_8")]
    fn connect_context_menu<F: Fn(&Self, &ContextMenu, &WebHitTestResult) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_document_loaded<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    //#[cfg(feature = "v2_16")]
    //fn connect_form_controls_associated<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_send_request<F: Fn(&Self, &URIRequest, &Option<URIResponse>) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<WebPage> + IsA<glib::object::Object>> WebPageExt for O {
    fn get_dom_document(&self) -> Option<DOMDocument> {
        unsafe {
            from_glib_none(ffi::webkit_web_page_get_dom_document(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_10")]
    fn get_editor(&self) -> Option<WebEditor> {
        unsafe {
            from_glib_none(ffi::webkit_web_page_get_editor(self.to_glib_none().0))
        }
    }

    fn get_id(&self) -> u64 {
        unsafe {
            ffi::webkit_web_page_get_id(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_2")]
    fn get_main_frame(&self) -> Option<Frame> {
        unsafe {
            from_glib_none(ffi::webkit_web_page_get_main_frame(self.to_glib_none().0))
        }
    }

    fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_web_page_get_uri(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_12")]
    fn connect_console_message_sent<F: Fn(&Self, &ConsoleMessage) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &ConsoleMessage) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "console-message-sent",
                transmute(console_message_sent_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v2_8")]
    fn connect_context_menu<F: Fn(&Self, &ContextMenu, &WebHitTestResult) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &ContextMenu, &WebHitTestResult) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "context-menu",
                transmute(context_menu_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_document_loaded<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "document-loaded",
                transmute(document_loaded_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    //#[cfg(feature = "v2_16")]
    //fn connect_form_controls_associated<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Empty ctype elements: *.PtrArray TypeId { ns_id: 1, id: 12 }
    //}

    fn connect_send_request<F: Fn(&Self, &URIRequest, &Option<URIResponse>) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &URIRequest, &Option<URIResponse>) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "send-request",
                transmute(send_request_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::uri",
                transmute(notify_uri_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v2_12")]
unsafe extern "C" fn console_message_sent_trampoline<P>(this: *mut ffi::WebKitWebPage, console_message: *mut ffi::WebKitConsoleMessage, f: glib_ffi::gpointer)
where P: IsA<WebPage> {
    callback_guard!();
    let f: &&(Fn(&P, &ConsoleMessage) + 'static) = transmute(f);
    f(&WebPage::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(console_message))
}

#[cfg(feature = "v2_8")]
unsafe extern "C" fn context_menu_trampoline<P>(this: *mut ffi::WebKitWebPage, context_menu: *mut ffi::WebKitContextMenu, hit_test_result: *mut ffi::WebKitWebHitTestResult, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<WebPage> {
    callback_guard!();
    let f: &&(Fn(&P, &ContextMenu, &WebHitTestResult) -> bool + 'static) = transmute(f);
    f(&WebPage::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(context_menu), &from_glib_borrow(hit_test_result)).to_glib()
}

unsafe extern "C" fn document_loaded_trampoline<P>(this: *mut ffi::WebKitWebPage, f: glib_ffi::gpointer)
where P: IsA<WebPage> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&WebPage::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn send_request_trampoline<P>(this: *mut ffi::WebKitWebPage, request: *mut ffi::WebKitURIRequest, redirected_response: *mut ffi::WebKitURIResponse, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<WebPage> {
    callback_guard!();
    let f: &&(Fn(&P, &URIRequest, &Option<URIResponse>) -> bool + 'static) = transmute(f);
    f(&WebPage::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(request), &from_glib_borrow(redirected_response)).to_glib()
}

unsafe extern "C" fn notify_uri_trampoline<P>(this: *mut ffi::WebKitWebPage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<WebPage> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&WebPage::from_glib_borrow(this).downcast_unchecked())
}
