#![cfg(test)]

use std::convert::{TryFrom, TryInto};
use std::sync::Arc;

use crate::behavior::sandbox_member::SandboxMemberBehavior;
use crate::node::concrete::*;
use crate::node::contents::{AttributeStore, CommentStore, NodeType, TextStore};
use crate::node::element::{ElementStore, HtmlBodyStore, HtmlButtonStore};
use crate::node::element::{HtmlBodyTemplate, HtmlButtonTemplate, HtmlHtmlTemplate};
use crate::node::{AnyNodeArc, NodeBehavior};
use crate::sandbox::Sandbox;
use crate::{config::ScreenMetrics, node::graph_storage::Selector};

#[test]
fn it_works() {
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let doc = sbox.clone().window().document();
    let document_element = doc.build(HtmlHtmlTemplate).unwrap().into();
    let _text = doc.create_text_node("Hello, world!".to_string());
    doc.append_child(document_element);
    assert_eq!(doc.child_nodes().length(), 1);
}

macro_rules! test_node_creation {
    ($ty: ty, $node_type: expr, $storage: expr) => {{
        let metrics: ScreenMetrics = Default::default();
        let sbox = Sandbox::new(metrics);
        let doc = sbox.clone().window().document();
        let weak_sbox = Arc::downgrade(&sbox);

        let node = <$ty>::new(weak_sbox, $storage);
        doc.append_child(node.into());
        assert_eq!(doc.child_nodes().length(), 1);
        assert_eq!(
            doc.first_child().unwrap().get_node_type(),
            $node_type.get_node_number()
        );

        doc
    }};
}

#[test]
fn test_element_node_m() {
    let _elem = test_node_creation!(
        ElementNodeArc,
        NodeType::Element,
        Arc::new(ElementStore::HtmlButton(HtmlButtonStore))
    );
}

#[test]
fn test_attr_node() {
    let _doc = test_node_creation!(AttributeNodeArc, NodeType::Attribute, Default::default());
}

#[test]
fn test_text_node() {
    let text = test_node_creation!(
        TextNodeArc,
        NodeType::Text,
        Arc::new(TextStore {
            data: "test".to_owned()
        })
    );

    let node = text.first_child().unwrap();
    let node: Result<TextNodeArc, AnyNodeArc> = node.try_into();
    match node {
        Ok(node) => {
            assert_eq!(node.contents.data().unwrap(), "test".to_owned());
        }
        _ => {
            panic!("Could not cast node");
        }
    }
}

#[test]
fn test_c_data_section_node_node() {
    let _cds = test_node_creation!(
        CDataSectionNodeArc,
        NodeType::CDataSection,
        Default::default()
    );
}

#[test]
fn test_processing_instruction_node() {
    let _pi = test_node_creation!(
        ProcessingInstructionNodeArc,
        NodeType::ProcessingInstruction,
        Default::default()
    );
}

#[test]
fn test_comment_node() {
    let _com = test_node_creation!(
        CommentNodeArc,
        NodeType::Comment,
        Arc::new(CommentStore {
            data: "test".to_owned()
        })
    );
}

#[test]
fn test_document_type_node() {
    let _dt = test_node_creation!(
        DocumentTypeNodeArc,
        NodeType::DocumentType,
        Default::default()
    );
}

#[test]
fn test_document_fragment_node() {
    let _frag = test_node_creation!(
        DocumentFragmentNodeArc,
        NodeType::DocumentFragment,
        Default::default()
    );
}

#[test]
fn can_build_node() {
    use crate::behavior::sandbox_member::SandboxMemberBehavior;
    use std::sync::Weak;

    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);

    let node = sbox.build(AttributeTemplate(Default::default())).unwrap();
    let _: ConcreteNodeArc<AttributeStore> = node; // assert that we got an AttributeNode

    assert!(Weak::ptr_eq(&node.get_context(), &Arc::downgrade(&sbox)));
}

#[test]
fn tag_name() {
    let button = ElementStore::HtmlButton(HtmlButtonStore);
    let body = ElementStore::HtmlBody(HtmlBodyStore);
    assert_eq!(button.tag_name(), "BUTTON");
    assert_eq!(body.tag_name(), "BODY");
}

#[test]
fn selector() {
    let sbox = Sandbox::new(Default::default());

    let button = sbox.build(HtmlButtonTemplate).unwrap();
    let body = sbox.build(HtmlBodyTemplate).unwrap();

    let button_any = button.clone().into();

    let selector = Selector::try_from("button".to_string()).unwrap();

    assert!(selector.filter_selected_node(button_any).is_ok());
    assert!(selector.is_selected_element(button));
    assert!(!selector.is_selected_element(body));
}

#[test]
fn query_selector() {
    let sbox = Sandbox::new(Default::default());

    let button: ElementNodeArc = sbox.build(HtmlButtonTemplate).unwrap().into();
    let body: ElementNodeArc = sbox.build(HtmlBodyTemplate).unwrap().into();

    let buttonselector = Selector::try_from("BUTTON").unwrap();
    let bodyselector = Selector::try_from("BODY").unwrap();

    let doc = sbox.window().document();

    doc.append_child(button.clone().into());
    doc.append_child(body.clone().into());

    let qbody = doc.query_selector(&bodyselector).unwrap();
    let qbutton = doc.query_selector(&buttonselector).unwrap();

    assert!(Arc::ptr_eq(&qbody.common, &body.common));
    assert!(Arc::ptr_eq(&qbutton.common, &button.common));
}

#[test]
fn query_selector_child() {
    let sbox = Sandbox::new(Default::default());

    let button: ElementNodeArc = sbox.build(HtmlButtonTemplate).unwrap().into();
    let body: ElementNodeArc = sbox.build(HtmlBodyTemplate).unwrap().into();

    let buttonselector = Selector::try_from("BUTTON").unwrap();
    let bodyselector = Selector::try_from("BODY").unwrap();

    let doc = sbox.window().document();

    doc.append_child(body.clone().into());
    body.append_child(button.clone().into());

    let qbody = doc.query_selector(&bodyselector).unwrap();
    let qbutton = doc.query_selector(&buttonselector).unwrap();

    assert!(Arc::ptr_eq(&qbody.common, &body.common));
    assert!(Arc::ptr_eq(&qbutton.common, &button.common));
}
