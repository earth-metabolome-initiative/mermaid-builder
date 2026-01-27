//! Submodule defining an edge struct for entity-relationship diagrams in
//! Mermaid syntax.

use alloc::rc::Rc;
use core::fmt::Display;

use crate::{
    diagrams::entity_relationship::entity_relationship_node::ERNode,
    shared::{ArrowShape, GenericEdge, LineStyle, NODE_LETTER, generic_edge::GenericEdgeBuilder},
    traits::{edge::Edge, edge_builder::EdgeBuilder, node::Node},
};
/// Type alias for an entity-relationship edge builder.
pub type EREdgeBuilder = GenericEdgeBuilder<ERNode>;
/// Type alias for an entity-relationship edge.
pub type EREdge = GenericEdge<ERNode>;

impl EREdgeBuilder {
    /// Creates a new entity-relationship edge builder.
    ///
    /// # Arguments
    ///
    /// * `source` - The source node of the edge.
    /// * `destination` - The destination node of the edge.
    ///
    /// # Example
    ///
    /// ```
    /// use std::rc::Rc;
    ///
    /// use mermaid_builder::prelude::*;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let node1 = Rc::new(ERNodeBuilder::default().label("A")?.id(0).build()?);
    ///     let node2 = Rc::new(ERNodeBuilder::default().label("B")?.id(1).build()?);
    ///
    ///     let builder = EREdgeBuilder::zero_or_one(node1, node2);
    ///     Ok(())
    /// }
    /// ```
    pub fn zero_or_one(source: Rc<ERNode>, destination: Rc<ERNode>) -> Self {
        Self::default()
            .source(source)
            .unwrap()
            .destination(destination)
            .unwrap()
            .left_arrow_shape(ArrowShape::ZeroOrOne)
            .unwrap()
            .right_arrow_shape(ArrowShape::ZeroOrOne)
            .unwrap()
            .line_style(LineStyle::Solid)
    }

    /// Creates a new entity-relationship edge builder with a one-to-one
    /// relationship.
    ///
    /// # Arguments
    ///
    /// * `source` - The source node of the edge.
    /// * `destination` - The destination node of the edge.
    ///
    /// # Example
    ///
    /// ```
    /// use std::rc::Rc;
    ///
    /// use mermaid_builder::prelude::*;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let node1 = Rc::new(ERNodeBuilder::default().label("A")?.id(0).build()?);
    ///     let node2 = Rc::new(ERNodeBuilder::default().label("B")?.id(1).build()?);
    ///
    ///     let builder = EREdgeBuilder::one_to_one(node1, node2);
    ///     Ok(())
    /// }
    /// ```
    pub fn one_to_one(source: Rc<ERNode>, destination: Rc<ERNode>) -> Self {
        Self::default()
            .source(source)
            .unwrap()
            .destination(destination)
            .unwrap()
            .left_arrow_shape(ArrowShape::ExactlyOne)
            .unwrap()
            .right_arrow_shape(ArrowShape::ExactlyOne)
            .unwrap()
            .line_style(LineStyle::Solid)
    }

    /// Creates a new entity-relationship edge builder with a zero-or-one
    /// relationship.
    ///
    /// # Arguments
    ///
    /// * `source` - The source node of the edge.
    /// * `destination` - The destination node of the edge.
    ///
    /// # Example
    ///
    /// ```
    /// use std::rc::Rc;
    ///
    /// use mermaid_builder::prelude::*;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let node1 = Rc::new(ERNodeBuilder::default().label("A")?.id(0).build()?);
    ///     let node2 = Rc::new(ERNodeBuilder::default().label("B")?.id(1).build()?);
    ///
    ///     let builder = EREdgeBuilder::zero_or_more(node1, node2);
    ///     Ok(())
    /// }
    /// ```
    pub fn zero_or_more(source: Rc<ERNode>, destination: Rc<ERNode>) -> Self {
        Self::default()
            .source(source)
            .unwrap()
            .destination(destination)
            .unwrap()
            .left_arrow_shape(ArrowShape::ZeroOrMore)
            .unwrap()
            .right_arrow_shape(ArrowShape::ZeroOrMore)
            .unwrap()
            .line_style(LineStyle::Solid)
    }

    /// Creates a new entity-relationship edge builder with a one-or-more
    /// relationship.
    ///
    /// # Arguments
    ///
    /// * `source` - The source node of the edge.
    /// * `destination` - The destination node of the edge.
    ///
    /// # Example
    ///
    /// ```
    /// use std::rc::Rc;
    ///
    /// use mermaid_builder::prelude::*;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let node1 = Rc::new(ERNodeBuilder::default().label("A")?.id(0).build()?);
    ///     let node2 = Rc::new(ERNodeBuilder::default().label("B")?.id(1).build()?);
    ///
    ///     let builder = EREdgeBuilder::one_or_more(node1, node2);
    ///     Ok(())
    /// }
    /// ```
    pub fn one_or_more(source: Rc<ERNode>, destination: Rc<ERNode>) -> Self {
        Self::default()
            .source(source)
            .unwrap()
            .destination(destination)
            .unwrap()
            .left_arrow_shape(ArrowShape::OneOrMore)
            .unwrap()
            .right_arrow_shape(ArrowShape::OneOrMore)
            .unwrap()
            .line_style(LineStyle::Solid)
    }
}

impl Display for EREdge {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use crate::traits::TabbedDisplay;
        self.fmt_tabbed(f, 0)
    }
}

impl crate::traits::TabbedDisplay for EREdge {
    fn fmt_tabbed(&self, f: &mut core::fmt::Formatter<'_>, tab_count: usize) -> core::fmt::Result {
        let indent = " ".repeat(tab_count * 2);
        writeln!(
            f,
            "{indent}{NODE_LETTER}{} {left_arrow}{segment}{right_arrow} {NODE_LETTER}{} : \"{label}\"",
            self.source().id(),
            self.destination().id(),
            label = self.label().unwrap_or(""),
            left_arrow = self.left_arrow_shape().as_ref().map_or_else(|| "", |shape| shape.left()),
            segment = match self.line_style() {
                LineStyle::Solid => "--",
                LineStyle::Thick => "==",
                LineStyle::Dashed => "..",
            },
            right_arrow =
                self.right_arrow_shape().as_ref().map_or_else(|| "", |shape| shape.right()),
        )
    }
}

#[cfg(test)]
mod tests {
    use alloc::{boxed::Box, format};

    use super::*;
    use crate::{
        diagrams::entity_relationship::entity_relationship_node::ERNodeBuilder,
        traits::node_builder::NodeBuilder,
    };

    #[test]
    fn test_er_edge_builder_methods() -> Result<(), Box<dyn core::error::Error>> {
        let node1 = Rc::new(ERNodeBuilder::default().label("A")?.id(0).build()?);
        let node2 = Rc::new(ERNodeBuilder::default().label("B")?.id(1).build()?);

        let edge_zero_one = EREdgeBuilder::zero_or_one(node1.clone(), node2.clone()).build()?;
        assert_eq!(edge_zero_one.left_arrow_shape(), Some(ArrowShape::ZeroOrOne));
        assert_eq!(edge_zero_one.right_arrow_shape(), Some(ArrowShape::ZeroOrOne));

        let edge_one_one = EREdgeBuilder::one_to_one(node1.clone(), node2.clone()).build()?;
        assert_eq!(edge_one_one.left_arrow_shape(), Some(ArrowShape::ExactlyOne));
        assert_eq!(edge_one_one.right_arrow_shape(), Some(ArrowShape::ExactlyOne));

        let edge_zero_more = EREdgeBuilder::zero_or_more(node1.clone(), node2.clone()).build()?;
        assert_eq!(edge_zero_more.left_arrow_shape(), Some(ArrowShape::ZeroOrMore));
        assert_eq!(edge_zero_more.right_arrow_shape(), Some(ArrowShape::ZeroOrMore));

        let edge_one_more = EREdgeBuilder::one_or_more(node1.clone(), node2.clone()).build()?;
        assert_eq!(edge_one_more.left_arrow_shape(), Some(ArrowShape::OneOrMore));
        assert_eq!(edge_one_more.right_arrow_shape(), Some(ArrowShape::OneOrMore));

        Ok(())
    }

    #[test]
    fn test_er_edge_display() -> Result<(), Box<dyn core::error::Error>> {
        let node1 = Rc::new(ERNodeBuilder::default().label("A")?.id(0).build()?);
        let node2 = Rc::new(ERNodeBuilder::default().label("B")?.id(1).build()?);

        let edge = EREdgeBuilder::zero_or_one(node1, node2).label("relates to")?.build()?;

        let output = format!("{edge}");
        // Expected format: v0 |o--o| v1 : "relates to"
        assert!(output.contains("v0"));
        assert!(output.contains("v1"));
        assert!(output.contains("relates to"));
        assert!(output.contains("|o--o|"));

        Ok(())
    }
}
