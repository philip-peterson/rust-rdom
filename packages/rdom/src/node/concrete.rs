//! Concrete (as opposed to abstract) types of nodes. Each node class is represented in this module.

use crate::internal_prelude::*;
use crate::node::element::ElementStore;
use crate::node_list::NodeList;
use crate::selector::Selector;
use crate::{impl_parent_node, proxy_node_behavior, proxy_parent_node_behavior};

use super::contents::{
    AttributeStore, CDataSectionStore, CommentStore, DocumentFragmentStore, DocumentStore,
    DocumentTypeStore, ProcessingInstructionStore, TextStore,
};
use super::{
    AnyNodeStore, Buildable, NodeBehavior, NodeCommon, NodeContentsArc, NodeContentsWeak,
    NodeGraphStorage,
};

use arc_new_cyclic_n::arc::new_cyclic_2;
use std::convert::TryFrom;

crate::use_behaviors!(parent_node);

/// A strongly-typed handle to a node with a strong reference.
/// Generic type `S` may be the underlying storage
/// type of any node class.
#[sourcegen::sourcegen(generator = "behave", script = "SandboxMember common.context")]
// Generated. All manual edits to the block annotated with #[sourcegen...] will be discarded.
#[derive(Clone)]
pub struct ConcreteNodeArc<S: AnyNodeStore> {
    pub(crate) contents: Arc<S>,
    pub(crate) common: Arc<NodeCommon>,
}

#[sourcegen::generated]
impl<S: AnyNodeStore> ConcreteNodeArc<S> {
    /// gets `Weak<Sandbox>` to the `Sandbox` that it is in
    pub fn get_context(&self) -> Weak<Sandbox> {
        self.common.context.clone()
    }
}

#[sourcegen::generated]
impl<S: AnyNodeStore> SandboxMemberBehavior for ConcreteNodeArc<S> {
    fn get_context(&self) -> Weak<Sandbox> {
        self.get_context()
    }
}

impl<S: AnyNodeStore> PartialEq for ConcreteNodeArc<S> {
    fn eq(&self, other: &Self) -> bool {
        let a = Arc::ptr_eq(&self.contents, &other.contents);
        let b = Arc::ptr_eq(&self.common, &other.common);
        if a && !b || !a && b {
            log::warn!("Two ConcreteNodeArc pointers were observed being 'half-equal'; this means there is a bug, probably in RDOM!");
        }
        a && b
    }
}

/// A strongly-typed handle to a node with a weak reference.
/// Generic type `S` may be the underlying storage
/// type of any node class.
#[derive(Clone)]
pub struct ConcreteNodeWeak<S: AnyNodeStore> {
    pub(crate) contents: Weak<S>,
    pub(crate) common: Weak<NodeCommon>,
}

impl<S: AnyNodeStore> PartialEq for ConcreteNodeWeak<S> {
    fn eq(&self, other: &Self) -> bool {
        let a = self.contents.ptr_eq(&other.contents);
        let b = self.common.ptr_eq(&other.common);
        if a && !b || !a && b {
            log::warn!("Two ConcreteNodeWeak pointers were observed being 'half-equal'; this means there is a bug, probably in RDOM!");
        }
        a && b
    }
}

macro_rules! impl_concrete {
    ($($ti:expr => $name:ident),*) => {
        paste::paste! {
            $(
                impl AnyNodeStore for [<$name Store>] {}

                #[doc = "Convenience alias for a strong reference to a(n) " $name " node"]
                pub type [<$name NodeArc>] = ConcreteNodeArc<[<$name Store>]>;

                #[doc = "Convenience alias for a weak reference to a(n) " $name " node"]
                pub type [<$name NodeWeak>] = ConcreteNodeWeak<[<$name Store>]>;

                impl ConcreteNodeArc<[<$name Store>]> {
                    pub(crate) fn new(context: Weak<Sandbox>, contents: Arc<[<$name Store>]>) ->
                    ConcreteNodeArc<[<$name Store>]> {
                        Self::new_cyclic(context, |_| (*contents).clone())
                    }

                    pub(crate) fn new_cyclic(context: Weak<Sandbox>,
                        data_fn: impl FnOnce(&ConcreteNodeWeak<[<$name Store>]>) -> [<$name Store>]) ->
                    ConcreteNodeArc<[<$name Store>]> {
                        let (common_strong, contents_strong) = new_cyclic_2(|common_weak, contents_weak| {
                            let node_weak = ConcreteNodeWeak {
                                contents: contents_weak.clone(),
                                common: common_weak.clone()
                            };

                            let contents: [<$name Store>] = data_fn(&node_weak);

                            let weak_contents_ref: NodeContentsWeak = contents_weak.clone().into();

                            let common = NodeCommon {
                                node_graph: NodeGraphStorage::new(AnyNodeWeak {
                                    contents: weak_contents_ref,
                                    common: common_weak.clone(),
                                }),
                                parent_node_behavior: ParentNodeBehaviorStorage::new(common_weak.clone()),
                                context,
                            };

                            (common, contents)
                        });

                        ConcreteNodeArc { contents: contents_strong, common: common_strong }
                    }

                    proxy_node_behavior!();
                }

                impl Buildable for ConcreteNodeArc<[<$name Store>]> {
                    type Storage = [<$name Store>];
                }

                impl TryFrom<AnyNodeArc> for ConcreteNodeArc<[<$name Store>]> {
                    type Error = AnyNodeArc;

                    fn try_from(value: AnyNodeArc) -> Result<Self, Self::Error> {
                        match value.contents {
                            NodeContentsArc::$name(element) => {
                                return Ok(ConcreteNodeArc {
                                    contents: element,
                                    common: value.common,
                                })
                            },
                            _ => Err(value),
                        }
                    }
                }

                impl TryFrom<AnyNodeWeak> for ConcreteNodeWeak<[<$name Store>]> {
                    type Error = AnyNodeWeak;

                    fn try_from(value: AnyNodeWeak) -> Result<Self, Self::Error> {
                        match value.contents {
                            NodeContentsWeak::$name(element) => {
                                return Ok(ConcreteNodeWeak {
                                    contents: element,
                                    common: value.common,
                                })
                            },
                            _ => Err(value),
                        }
                    }
                }

                impl From<ConcreteNodeArc<[<$name Store>]>> for AnyNodeArc {
                    fn from(concrete: ConcreteNodeArc<[<$name Store>]>) -> Self {
                        AnyNodeArc {
                            common: concrete.common,
                            contents: NodeContentsArc::$name(concrete.contents),
                        }
                    }
                }

                impl From<ConcreteNodeWeak<[<$name Store>]>> for AnyNodeWeak {
                    fn from(concrete: ConcreteNodeWeak<[<$name Store>]>) -> Self {
                        AnyNodeWeak {
                            common: concrete.common,
                            contents: NodeContentsWeak::$name(concrete.contents),
                        }
                    }
                }

                impl NodeBehavior for ConcreteNodeArc<[<$name Store>]> {
                    fn first_child(&self) -> Option<AnyNodeArc> {
                        self.common.node_graph.first_child()
                    }

                    fn last_child(&self) -> Option<AnyNodeArc> {
                        self.common.node_graph.last_child()
                    }

                    fn append_child(&self, other: AnyNodeArc) {
                        self.common.node_graph.append_child(other)
                    }

                    fn child_nodes(&self) -> Arc<NodeList> {
                        self.common.node_graph.child_nodes()
                    }

                    fn clone_node(&self) -> AnyNodeArc {
                        AnyNodeArc::from(self.clone()).clone_node()
                    }

                    fn node_type(&self) -> isize {
                        $ti
                    }

                    fn query_selector(&self, selector: &Selector) -> Result<Option<ElementNodeArc>, DomError> {
                        self.common.parent_node_behavior.query_selector(selector)
                    }
                }
            )*
        }
    }
}

impl_concrete! {
    1 => Element,
    2 => Attribute,
    3 => Text,
    4 => CDataSection,
    5 => ProcessingInstruction,
    6 => Comment,
    7 => Document,
    8 => DocumentType,
    9 => DocumentFragment
}

impl_parent_node!(ConcreteNodeArc<ElementStore>, common.parent_node_behavior);
impl_parent_node!(ConcreteNodeArc<DocumentStore>, common.parent_node_behavior);

impl ConcreteNodeArc<ElementStore> {
    proxy_parent_node_behavior!();
}

impl ConcreteNodeArc<DocumentStore> {
    proxy_parent_node_behavior!();

    /// Creates a new text node with the given text contents
    pub fn create_text_node(&self, text: String) -> Result<TextNodeArc, DomError> {
        match self.get_context().upgrade() {
            Some(context) => Ok(context
                .builder::<TextNodeArc>()
                .build(TextStore { data: text })),
            None => Err(DomError::SandboxDropped),
        }
    }

    /// Creates an HTML element with the given tag name
    pub fn create_element(&self, tag_name: String) -> Result<ElementNodeArc, DomError> {
        let context = self
            .get_context()
            .upgrade()
            .ok_or(DomError::SandboxDropped)?;

        let builder = context.builder::<ElementNodeArc>();

        Ok(match tag_name.to_lowercase().as_ref() {
            "html" => builder.build_html(),
            "body" => builder.build_body(),
            "button" => builder.build_button(),
            _ => builder.build_unknown(tag_name),
        })
    }
}
