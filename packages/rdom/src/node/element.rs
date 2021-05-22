//! Data and functionality to each element type live here.

use super::concrete::ConcreteNodeArc;
use crate::node::concrete::ElementNodeArc;
use crate::sandbox::Builder;
use crate::{internal_prelude::*, named_node_map::NamedNodeMap};

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
pub struct ElementCommon {
    attrs: Arc<NamedNodeMap>,
}

/// Layer at the top of element storage
#[derive(Clone)]
pub struct ElementStore {
    /// Data common to all elements
    element_common: ElementCommon,

    /// Data specific to this particular element
    node_store: ElementKind,

    /// Reference back up to the DOM node
    pub(crate) node: AnyNodeWeak,
}

impl ElementStore {
    pub(crate) fn new(
        node_store: ElementKind,
        context: Weak<Sandbox>,
        node: AnyNodeWeak,
    ) -> ElementStore {
        ElementStore {
            node_store,
            element_common: ElementCommon {
                attrs: NamedNodeMap::new(context),
            },
            node,
        }
    }

    /// [Element.tagName](https://developer.mozilla.org/en-US/docs/Web/API/Element/tagName)
    pub fn tag_name(&self) -> String {
        self.node_store.tag_name()
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
