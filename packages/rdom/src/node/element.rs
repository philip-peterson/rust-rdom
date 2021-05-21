//! Data and functionality to each element type live here.

use super::concrete::ConcreteNodeArc;
use crate::internal_prelude::*;
use crate::node::concrete::ElementNodeArc;
use crate::sandbox::Builder;

macro_rules! declare_html_elements {
    ($($tag:literal => $name:ident),*) => {
        paste::paste! {
        /// Enum of all HTMLElements
        #[derive(Clone)]
        pub enum HtmlElementStore {
            $(
                #[doc = "[" $tag "](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/" $tag ")"]
                $name([<$name Store>]),
            )*
            /// Represents an invalid HTML element
            HtmlUnknown(HtmlUnknownStore)
        }

        impl HtmlElementStore {
            /// [Element.tagName](https://developer.mozilla.org/en-US/docs/Web/API/Element/tagName)
            pub fn tag_name(&self) -> String {
                match self {
                    $(
                        HtmlElementStore::$name(_) => $tag.to_string(),
                    )*
                    HtmlElementStore::HtmlUnknown(store) => store.tag_name.clone(),
                }
            }
        }
    }
    };
}

/// Enum of all SVGElements
#[derive(Clone)]
pub enum SvgElementStore {}

/// Enum of all concrete elements
#[derive(Clone)]
pub enum ElementNodeStore {
    /// Enum variant for an HTMLElement
    HtmlElement(HtmlElementStore),

    /// Enum variant for an SVGElement
    SvgElement(SvgElementStore),
}

impl ElementNodeStore {
    /// [Element.tagName](https://developer.mozilla.org/en-US/docs/Web/API/Element/tagName)
    pub fn tag_name(&self) -> String {
        match self {
            ElementNodeStore::HtmlElement(el) => el.tag_name(),
            ElementNodeStore::SvgElement(_) => {
                unimplemented!()
            }
        }
    }
}

declare_html_elements! {
    "HTML" => HtmlHtml,
    "BODY" => HtmlBody,
    "BUTTON" => HtmlButton
}

/// html element storage
#[derive(Clone)]
pub struct HtmlHtmlStore;
/// html unknown element storage
#[derive(Clone)]
pub struct HtmlUnknownStore {
    tag_name: String,
}
/// body element storage
#[derive(Clone)]
pub struct HtmlBodyStore;
/// button element storage
#[derive(Clone)]
pub struct HtmlButtonStore;

impl Builder<ElementNodeArc> {
    // TODO it would be nice if these didn't all return generic Elements but instead we had some kind of
    // concrete types representing each element type.

    /// Builds a new HtmlHtmlElement node with a weak reference to its corresponding window
    pub fn build_html(&self) -> ConcreteNodeArc<ElementNodeStore> {
        ConcreteNodeArc::<ElementNodeStore>::new(
            self.sandbox.clone(),
            Arc::new(ElementNodeStore::HtmlElement(HtmlElementStore::HtmlHtml(
                HtmlHtmlStore,
            ))),
        )
    }

    /// Builds a new HtmlBodyElement node
    pub fn build_body(&self) -> ConcreteNodeArc<ElementNodeStore> {
        ConcreteNodeArc::<ElementNodeStore>::new(
            self.sandbox.clone(),
            Arc::new(ElementNodeStore::HtmlElement(HtmlElementStore::HtmlBody(
                HtmlBodyStore,
            ))),
        )
    }

    /// Builds a new HtmlButtonElement node
    pub fn build_button(&self) -> ConcreteNodeArc<ElementNodeStore> {
        ConcreteNodeArc::<ElementNodeStore>::new(
            self.sandbox.clone(),
            Arc::new(ElementNodeStore::HtmlElement(HtmlElementStore::HtmlButton(
                HtmlButtonStore,
            ))),
        )
    }

    /// Builds a new HtmlUnknownElement node
    pub fn build_unknown(&self, tag_name: String) -> ConcreteNodeArc<ElementNodeStore> {
        ConcreteNodeArc::<ElementNodeStore>::new(
            self.sandbox.clone(),
            Arc::new(ElementNodeStore::HtmlElement(HtmlElementStore::HtmlUnknown(
                HtmlUnknownStore { tag_name },
            ))),
        )
    }
}
