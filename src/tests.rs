#![cfg(test)]

use std::sync::Arc;

use crate::behavior::node::NodeBehavior;
use crate::config::ScreenMetrics;
use crate::node::element::{HtmlBodyElement, HtmlButtonElement, HtmlHtmlElement};
use crate::sandbox::Sandbox;

use crate::node::AnyNode;

#[test]
fn it_works() {
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let doc = sbox.clone().window().document();
    let document_element = HtmlHtmlElement::new(Arc::downgrade(&sbox), ());
    let _text = doc.create_text_node("Hello, world!".to_string());
    doc.append_child(document_element);
    assert_eq!(doc.child_nodes().length(), 1);
}

#[test]
fn query_selector() {
    let sbox = Sandbox::new(Default::default());
    let doc = sbox.clone().window().document();

    let html = HtmlHtmlElement::new(Arc::downgrade(&sbox), ());
    let body = HtmlBodyElement::new(Arc::downgrade(&sbox), ());

    doc.append_child(html.clone());
    html.append_child(body.clone());

    let qbody = doc
        .query_selector("body")
        .unwrap()
        .unwrap()
        .downcast_arc()
        .unwrap();

    // query selector on nodes
    assert!(Arc::ptr_eq(&qbody, &body));
}
