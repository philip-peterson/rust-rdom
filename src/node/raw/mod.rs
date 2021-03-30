//! Raw representation of a DOM node. See [node](../index.html) module for distinction from
//! wrapped representation.

use downcast_rs::DowncastSync;
use paste::paste;

use std::rc::Rc;
use std::sync::{Arc, Weak};

use crate::behavior::NodeBehavior;
use crate::error::DomError;
use crate::node::raw::private::PrivateAnyRawNode;
use crate::sandbox::Sandbox;
use crate::window::Window;

pub mod element;
pub(crate) mod private;

/// An input event
pub struct InputEvent {}

/// A base trait for all raw node types
pub trait AnyRawNode: DowncastSync + PrivateAnyRawNode {
    /// Gives a weak reference to the sandbox the node was created in.
    fn get_context(&self) -> Weak<Sandbox>;

    /// Clones the node
    fn clone_node(&self) -> Arc<dyn AnyRawNode>;
}
impl_downcast!(sync AnyRawNode);

macro_rules! impl_raw_nodes {
    ($((
        $ty: ty,
        storage: $storage: ty,
        blurb: $blurb: literal,
        link: $link: literal,
        impl { $( $rest:tt )* }
        $(, $postlude: literal)?
    ))*) => {
        $(
            paste! {
                #[doc =
                    "The ["
                    $blurb
                    "](https://developer.mozilla.org/en-US/docs/Web/API/"
                    $link
                    ") node type"
                    $(" " $postlude)?
                ]
                pub struct $ty {
                    /// Reference to the sandbox to which this node belongs
                    pub context: Weak<Sandbox>,

                    /// Node behavior (fields/methods associated with the DOM class called Node)
                    pub(crate) node_behavior: Arc<NodeBehavior>,

                    pub(crate) storage: $storage,
                }
            }

            paste! {
                impl $ty {
                    pub(crate) fn new(context: Weak<Sandbox>, storage: $storage) -> Arc<$ty> {
                        let construction: Arc<$ty> = Arc::new_cyclic(|construction_weak| -> $ty {
                            $ty {
                                context,
                                node_behavior: Arc::new(NodeBehavior::new(construction_weak.clone())),
                                storage,
                            }
                        });

                        construction
                    }

                    $($rest)*
                }

                impl AnyRawNode for $ty {
                    fn get_context(&self) -> Weak<Sandbox> {
                        self.context.clone()
                    }

                    fn clone_node(&self) -> Arc<dyn AnyRawNode> {
                        let mut construction = $ty::new(self.get_context(), Default::default());

                        let mut cons = Arc::get_mut(&mut construction).expect("Could not construct node");
                        (*cons).storage = self.storage.clone();

                        construction
                    }
                }

                impl PrivateAnyRawNode for $ty {
                    fn get_node_behavior(&self) -> Arc<NodeBehavior> {
                        self.node_behavior.clone()
                    }
                }
            }
        )*
    }
}

#[derive(Default, Clone)]
pub(crate) struct DocumentStorage {
    // Pointer back up to the window
    pub(crate) default_view: Weak<Window>,
}

impl_raw_nodes! {
    (
        AttrNode,
        storage: (),
        blurb: "attr (attribute)",
        link: "Attr",
        impl {}
    )
    (
        TextNode,
        storage: (),
        blurb: "text",
        link: "Text",
        impl {}
    )
    (
        Document,
        storage: DocumentStorage,
        blurb: "document",
        link: "Document",
        impl {}
    )
}
