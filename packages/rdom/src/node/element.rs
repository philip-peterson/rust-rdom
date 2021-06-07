//! Data and functionality to each element type live here.

use super::concrete::{ConcreteNodeArc, ElementNodeArc, ElementNodeWeak};
use crate::sandbox::Builder;
use crate::{internal_prelude::*, named_node_map::NamedNodeMap};
use std::sync::RwLock;

use std::convert::TryInto;

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

/// Data common to all elements
#[derive(Clone)]
pub struct ElementStore {
    /// Data specific to this particular element
    node_store: ElementKind,

    /// Reference back up to the DOM node
    pub(crate) node: AnyNodeWeak,

    /// Attributes
    attrs: Arc<RwLock<NamedNodeMap>>,
}

impl ElementStore {
    pub(crate) fn new(
        node_store: ElementKind,
        context: Weak<Sandbox>,
        node: AnyNodeWeak,
    ) -> ElementStore {
        ElementStore {
            node_store,
            attrs: Arc::new(RwLock::new(NamedNodeMap::new(
                context,
                node.clone()
                    .try_into()
                    .expect("Node was, unexpectedly, not an element"),
            ))),
            node,
        }
    }

    /// [Element.tagName](https://developer.mozilla.org/en-US/docs/Web/API/Element/tagName)
    pub fn tag_name(&self) -> String {
        self.node_store.tag_name()
    }

    /// [Element.hasAttribute](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttribute)
    pub fn has_attribute(&self, attr_name: String) -> bool {
        self.attrs
            .read()
            .expect("Could not lock attributes for reading")
            .get_named_item(attr_name)
            .is_some()
    }

    /// [Element.getAttribute](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttribute)
    pub fn get_attribute(&self, attr_name: String) -> Option<String> {
        self.attrs
            .read()
            .expect("Could not lock attributes for reading")
            .get_named_item(attr_name)
            .map(|item| item.contents.value.read().unwrap().clone())
    }

    /// [Element.removeAttribute](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttribute)
    pub fn remove_attribute(&self, attr_name: String) -> Result<(), DomError> {
        self.attrs
            .write()
            .expect("Could not lock attributes for writing")
            .remove_named_item(attr_name)
            .map(|_| ())
    }
}

impl ConcreteNodeArc<ElementStore> {
    /// [Element.hasAttribute](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttribute)
    pub fn has_attribute(&self, attr_name: String) -> bool {
        self.contents.has_attribute(attr_name)
    }

    /// [Element.getAttribute](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttribute)
    pub fn get_attribute(&self, attr_name: String) -> Option<String> {
        self.contents.get_attribute(attr_name)
    }

    /// [Element.removeAttribute](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttribute)
    pub fn remove_attribute(&mut self, attr_name: String) -> Result<(), DomError> {
        self.contents.remove_attribute(attr_name)
    }
}

/// Enum of all concrete elements
#[derive(Clone)]
pub(crate) enum ElementKind {
    /// Enum variant for an HTMLElement
    HtmlElement(HtmlElementStore),

    /// Enum variant for an SVGElement
    SvgElement(SvgElementStore),
}

impl ElementKind {
    /// [Element.tagName](https://developer.mozilla.org/en-US/docs/Web/API/Element/tagName)
    pub fn tag_name(&self) -> String {
        match self {
            ElementKind::HtmlElement(el) => el.tag_name(),
            ElementKind::SvgElement(_) => {
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
    pub fn build_html(&self) -> ConcreteNodeArc<ElementStore> {
        ConcreteNodeArc::<ElementStore>::new_cyclic(self.sandbox.clone(), |node_weak| {
            ElementStore::new(
                ElementKind::HtmlElement(HtmlElementStore::HtmlHtml(HtmlHtmlStore)),
                self.sandbox.clone(),
                node_weak.clone().into(),
            )
        })
    }

    /// Builds a new HtmlBodyElement node
    pub fn build_body(&self) -> ConcreteNodeArc<ElementStore> {
        ConcreteNodeArc::<ElementStore>::new_cyclic(self.sandbox.clone(), |node_weak| {
            ElementStore::new(
                ElementKind::HtmlElement(HtmlElementStore::HtmlBody(HtmlBodyStore)),
                self.sandbox.clone(),
                node_weak.clone().into(),
            )
        })
    }

    /// Builds a new HtmlButtonElement node
    pub fn build_button(&self) -> ConcreteNodeArc<ElementStore> {
        ConcreteNodeArc::<ElementStore>::new_cyclic(self.sandbox.clone(), |node_weak| {
            ElementStore::new(
                ElementKind::HtmlElement(HtmlElementStore::HtmlButton(HtmlButtonStore)),
                self.sandbox.clone(),
                node_weak.clone().into(),
            )
        })
    }

    /// Builds a new HtmlUnknownElement node
    pub fn build_unknown(&self, tag_name: String) -> ConcreteNodeArc<ElementStore> {
        ConcreteNodeArc::<ElementStore>::new_cyclic(self.sandbox.clone(), |node_weak| {
            ElementStore::new(
                ElementKind::HtmlElement(HtmlElementStore::HtmlUnknown(HtmlUnknownStore {
                    tag_name,
                })),
                self.sandbox.clone(),
                node_weak.clone().into(),
            )
        })
    }
}
