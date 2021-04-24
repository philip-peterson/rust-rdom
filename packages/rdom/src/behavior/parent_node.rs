#![macro_use]

use crate::internal_prelude::*;
use crate::node::concrete::ElementNodeArc;
use crate::node::contents::NodeType;
use crate::node::NodeCommon;
use crate::selector::Selector;

pub trait ParentNodeBehavior {
    fn child_element_count(&self) -> Result<usize, DomError>;
}

pub struct ParentNodeBehaviorStorage {
    pub(crate) node: Weak<NodeCommon>,
}

impl ParentNodeBehaviorStorage {
    pub fn new(node: Weak<NodeCommon>) -> ParentNodeBehaviorStorage {
        ParentNodeBehaviorStorage { node }
    }

    pub fn child_element_count(&self) -> Result<usize, DomError> {
        let node = self.node.upgrade().ok_or(DomError::SandboxDropped)?;
        Ok(node
            .node_graph
            .static_child_nodes()
            .iter()
            .filter(|node| node.contents.to_node_type() == NodeType::Element)
            .count())
    }

    pub fn query_selector(&self, selector: &Selector) -> Result<Option<ElementNodeArc>, DomError> {
        let node = self.node.upgrade().ok_or(DomError::SandboxDropped)?;
        let node_arc = node.node_graph.self_arc()?;

        if let Some(element) = selector.matches_selected_node(&node_arc) {
            return Ok(Some(element));
        }

        for child in node.node_graph.static_child_nodes().iter() {
            if let Ok(Some(element)) = child.common.parent_node_behavior.query_selector(selector) {
                return Ok(Some(element));
            }
        }

        return Ok(None);
    }
}

#[macro_export]
/// Implements ParentBehavior
macro_rules! impl_parent_node {
    ($structname: ty, $($fieldname: ident).+) => {
        paste::paste! {
            impl ParentNodeBehavior for $structname {
                fn child_element_count(&self) -> Result<usize, DomError> {
                    self.$($fieldname).+.child_element_count()
                }
            }
        }
    };
}