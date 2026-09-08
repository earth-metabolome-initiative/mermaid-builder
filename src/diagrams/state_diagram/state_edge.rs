//! State diagram state edge.

mod builder;
use alloc::{rc::Rc, string::ToString};
use core::fmt::{self, Display};

pub use builder::StateEdgeBuilder;

use super::{StateKind, StateNode, escape::Escaped};
use crate::{
    errors::EdgeError,
    shared::{
        ArrowShape, LineStyle, StyleClass,
        generic_edge::{GenericEdge, GenericEdgeBuilder},
    },
    traits::{Edge, TabbedDisplay},
};
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents an edge in a state diagram.
pub struct StateEdge {
    edge: GenericEdge<StateNode>,
}

impl crate::traits::edge::Edge for StateEdge {
    type Node = StateNode;
    type Builder = StateEdgeBuilder;
    fn source(&self) -> &Rc<Self::Node> {
        self.edge.source()
    }
    fn destination(&self) -> &Rc<Self::Node> {
        self.edge.destination()
    }
    fn label(&self) -> Option<&str> {
        self.edge.label()
    }
    fn classes(&self) -> impl Iterator<Item = &StyleClass> {
        self.edge.classes()
    }
    fn line_style(&self) -> crate::shared::LineStyle {
        self.edge.line_style()
    }
    fn left_arrow_shape(&self) -> Option<crate::shared::ArrowShape> {
        self.edge.left_arrow_shape()
    }
    fn right_arrow_shape(&self) -> Option<crate::shared::ArrowShape> {
        Some(ArrowShape::Normal)
    }
}

impl Display for StateEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_tabbed(f, 0)
    }
}

impl TabbedDisplay for StateEdge {
    fn fmt_tabbed(&self, f: &mut fmt::Formatter<'_>, tab_count: usize) -> fmt::Result {
        self.fmt_scoped(f, tab_count, "S")
    }
}

impl StateEdge {
    pub(super) fn fmt_scoped(
        &self,
        f: &mut fmt::Formatter<'_>,
        depth: usize,
        scope: &str,
    ) -> fmt::Result {
        write!(f, "{:width$}", "", width = depth * 4)?;
        self.source().fmt_id(f, scope)?;
        write!(f, " --> ")?;
        self.destination().fmt_id(f, scope)?;
        if let Some(label) = self.label() {
            // Mermaid transition labels require entity encoding.
            write!(f, " : {}", Escaped(label))?;
        }
        writeln!(f)
    }
}
