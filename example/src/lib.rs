/*
 * Copyright (c) 2016 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#![allow(deprecated)]

extern crate glib;
#[macro_use]
extern crate webkit2gtk_webextension;
extern crate libc;

use glib::closure::Closure;
use glib::variant::Variant;
use webkit2gtk_webextension::{
    DOMDocumentExt, DOMElementExt, DOMEventTargetExt, DOMMouseEvent, DOMMouseEventExt,
    WebExtension, WebExtensionExt, WebPage, WebPageExt,
};

/*web_extension_init!();

pub fn web_extension_initialize(extension: &WebExtension) {
    extension.connect_page_created(|_, page| {
        page.connect_document_loaded(|page| {
            println!("Page {} created for {:?}", page.get_id(), page.get_uri());
            let document = page.get_dom_document().unwrap();
            println!("URL: {:?}", document.get_url());
            println!("Title: {:?}", document.get_title());
            document.set_title("My Web Page");

            let handler = Closure::new(|values| {
                if let Some(event) = values[1].get::<Object>() {
                    if let Ok(mouse_event) = event.downcast::<DOMMouseEvent>() {
                        println!("Click at ({}, {})", mouse_event.get_x(), mouse_event.get_y());
                    }
                }
                None
            });
            document.add_event_listener_with_closure("click", &handler, false);

            println!("{}%", scroll_percentage(page));
            scroll_by(page, 45);

            println!("{}%", scroll_percentage(page));
            scroll_bottom(page);

            println!("{}%", scroll_percentage(page));
            scroll_top(page);

            println!("{}%", scroll_percentage(page));
        });
    });
}*/

web_extension_init_with_data!();

pub fn web_extension_initialize(extension: &WebExtension, user_data: Option<&Variant>) {
    let user_string: Option<String> = user_data.and_then(Variant::str).map(ToOwned::to_owned);
    dbg!(user_string);

    extension.connect_page_created(|_, page| {
        page.connect_document_loaded(|page| {
            println!("Page {} created for {:?}", page.id(), page.uri());
            let document = page.dom_document().unwrap();
            println!("URL: {:?}", document.url());
            println!("Title: {:?}", document.title());
            document.set_title("My Web Page");

            let handler = Closure::new(|values| {
                if let Ok(event) = values[1].get::<DOMMouseEvent>() {
                    if let Some(mouse_event) = event {
                        println!(
                            "Click at ({}, {})",
                            mouse_event.x(),
                            mouse_event.y()
                        );
                    }
                }
                None
            });
            document.add_event_listener_with_closure("click", &handler, false);

            println!("{}%", scroll_percentage(page));
            scroll_by(page, 45);

            println!("{}%", scroll_percentage(page));
            scroll_bottom(page);

            println!("{}%", scroll_percentage(page));
            scroll_top(page);

            println!("{}%", scroll_percentage(page));
        });
    });
}

fn scroll_by(page: &WebPage, pixels: i64) {
    let document = page.dom_document().unwrap();
    let body = document.body().unwrap();
    body.set_scroll_top(body.scroll_top() + pixels as libc::c_long);
}

fn scroll_bottom(page: &WebPage) {
    let document = page.dom_document().unwrap();
    let body = document.body().unwrap();
    body.set_scroll_top(body.scroll_height());
}

fn scroll_percentage(page: &WebPage) -> i64 {
    let document = page.dom_document().unwrap();
    let body = document.body().unwrap();
    let document = document.document_element().unwrap();
    let height = document.client_height();
    (body.scroll_top() as f64 / (body.scroll_height() as f64 - height) * 100.0) as i64
}

fn scroll_top(page: &WebPage) {
    let document = page.dom_document().unwrap();
    let body = document.body().unwrap();
    body.scroll_top();
}
