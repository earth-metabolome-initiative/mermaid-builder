//! Submodule defining the struct to represent an entity-relationship node
//! for the entity-relationship diagram in Mermaid syntax.

use std::fmt::Display;

pub mod attribute;
mod builder;
use attribute::EntityRelationshipAttribute;
pub use builder::ERNodeBuilder;

use crate::{
    shared::{GenericNode, NODE_LETTER, StyleClass, StyleProperty},
    traits::Node,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Struct representing an entity-relationship node in an ER diagram.
///
/// # Examples
///
/// ```
/// use mermaid_builder::{
///     diagrams::entity_relationship::ERNodeBuilder,
///     traits::{Node, NodeBuilder},
/// };
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let node = ERNodeBuilder::default()
///         .label("USER")?
///         .id(1)
///         .attribute("string", "username")
///         .build()?;
///
///     assert_eq!(node.label(), "USER");
///     Ok(())
/// }
/// ```
pub struct ERNode {
    /// Underlying node structure.
    node: GenericNode,
    /// Attributes of the entity-relationship node.
    attributes: Vec<EntityRelationshipAttribute>,
}

impl Node for ERNode {
    type Builder = ERNodeBuilder;

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
        matches!(
            shape,
            crate::shared::ArrowShape::OneOrMore
                | crate::shared::ArrowShape::ExactlyOne
                | crate::shared::ArrowShape::ZeroOrOne
                | crate::shared::ArrowShape::ZeroOrMore
        )
    }
}

impl Display for ERNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::traits::TabbedDisplay;
        self.fmt_tabbed(f, 0)
    }
}

impl crate::traits::TabbedDisplay for ERNode {
    fn fmt_tabbed(&self, f: &mut std::fmt::Formatter<'_>, tab_count: usize) -> std::fmt::Result {
        let indent = " ".repeat(tab_count * 2);
        write!(f, "{indent}{NODE_LETTER}{}[\"{}\"]", self.id(), self.label())?;

        if self.attributes.is_empty() {
            writeln!(f)?;
        } else {
            writeln!(f, " {{")?;

            for attr in &self.attributes {
                writeln!(f, "{indent}    {attr}")?;
            }
            writeln!(f, "{indent}}}")?;
        }

        for class in self.classes() {
            writeln!(f, "{indent}class {NODE_LETTER}{} {}", self.id(), class.name())?;
        }

        Ok(())
    }
}
