//! State diagram state node.

mod builder;
use alloc::{
    format,
    rc::Rc,
    string::{String, ToString},
};
use core::fmt::{self, Display};

pub use builder::StateNodeBuilder;

use super::{StateDiagram, StateKind, escape::Escaped};
use crate::{
    errors::NodeError,
    shared::{GenericNode, StyleClass, StyleProperty, generic_node::GenericNodeBuilder},
    traits::{Configuration, Diagram, Node, NodeBuilder, TabbedDisplay},
};
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a node in a state diagram.
pub struct StateNode {
    node: GenericNode,
    inner_diagram: Option<StateDiagram>,
    kind: StateKind,
}

impl Node for StateNode {
    type Builder = StateNodeBuilder;
    fn label(&self) -> &str {
        self.node.label()
    }
    fn id(&self) -> u64 {
        self.node.id()
    }
    fn styles(&self) -> impl Iterator<Item = &StyleProperty> {
        self.node.styles()
    }
    fn classes(&self) -> impl Iterator<Item = &StyleClass> {
        self.node.classes()
    }
    fn is_compatible_arrow_shape(shape: crate::shared::ArrowShape) -> bool {
        matches!(shape, crate::shared::ArrowShape::Normal)
    }
}

impl Display for StateNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_tabbed(f, 0)
    }
}

impl TabbedDisplay for StateNode {
    fn fmt_tabbed(&self, f: &mut fmt::Formatter<'_>, tab_count: usize) -> fmt::Result {
        self.fmt_scoped(f, tab_count, "S")?;
        self.fmt_styles(f, tab_count, "S")
    }
}

impl StateNode {
    /// Returns the state's kind.
    #[must_use]
    pub fn kind(&self) -> StateKind {
        self.kind
    }

    pub(super) fn fmt_id(&self, f: &mut fmt::Formatter<'_>, scope: &str) -> fmt::Result {
        if matches!(self.kind, StateKind::Start | StateKind::End) {
            write!(f, "[*]")
        } else {
            write!(f, "{scope}{}", self.id())
        }
    }

    pub(super) fn fmt_scoped(
        &self,
        f: &mut fmt::Formatter<'_>,
        depth: usize,
        scope: &str,
    ) -> fmt::Result {
        if matches!(self.kind, StateKind::Start | StateKind::End) {
            return Ok(());
        }
        let indent = "    ".repeat(depth);
        let id = format!("{scope}{}", self.id());
        match self.kind {
            StateKind::Choice => writeln!(f, "{indent}state {id} <<choice>>")?,
            StateKind::Fork => writeln!(f, "{indent}state {id} <<fork>>")?,
            StateKind::Join => writeln!(f, "{indent}state {id} <<join>>")?,
            _ => writeln!(f, "{indent}state \"{}\" as {id}", Escaped(self.label()))?,
        }
        if let Some(inner) = &self.inner_diagram {
            writeln!(f, "{indent}state {id} {{")?;
            inner.fmt_body(f, depth + 1, &format!("{id}_"))?;
            writeln!(f, "{indent}}}")?;
        }
        Ok(())
    }

    pub(super) fn fmt_styles(
        &self,
        f: &mut fmt::Formatter<'_>,
        depth: usize,
        scope: &str,
    ) -> fmt::Result {
        let indent = "    ".repeat(depth);
        let id = format!("{scope}{}", self.id());
        if let Some(inner) = &self.inner_diagram {
            inner.fmt_styles(f, depth, &format!("{id}_"))?;
        }
        for class in self.classes() {
            writeln!(f, "{indent}class {id} {scope}class_{}", class.name())?;
        }
        if self.has_styles() {
            write!(f, "{indent}style {id} ")?;
            for (index, style) in self.styles().enumerate() {
                if index > 0 {
                    write!(f, ",")?;
                }
                write!(f, "{style}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
