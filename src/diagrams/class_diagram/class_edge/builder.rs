//! Submodule defining a builder for class edges in class diagrams
//! in Mermaid syntax.

use crate::{
    diagrams::class_diagram::{
        class_edge::{ClassEdge, multiplicity::Multiplicity},
        class_node::ClassNode,
    },
    errors::EdgeError,
    shared::generic_edge::GenericEdgeBuilder,
    traits::EdgeBuilder,
};

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating a `ClassEdge`.
///
/// # Example
///
/// ```
/// use std::rc::Rc;
///
/// use mermaid_builder::prelude::*;
///
/// let node1 = Rc::new(ClassNodeBuilder::default().label("A").unwrap().id(0).build().unwrap());
/// let node2 = Rc::new(ClassNodeBuilder::default().label("B").unwrap().id(1).build().unwrap());
///
/// let edge = ClassEdgeBuilder::default()
///     .source(node1)
///     .unwrap()
///     .destination(node2)
///     .unwrap()
///     .right_arrow_shape(ArrowShape::Triangle)
///     .unwrap()
///     .build()
///     .unwrap();
/// ```
pub struct ClassEdgeBuilder {
    /// Underlying generic edge builder.
    edge_builder: GenericEdgeBuilder<ClassNode>,
    /// Left Multiplicity of the edge.
    left_multiplicity: Option<Multiplicity>,
    /// Right Multiplicity of the edge.
    right_multiplicity: Option<Multiplicity>,
}

impl ClassEdgeBuilder {
    /// Sets the left multiplicity of the edge.
    #[must_use]
    pub fn left_multiplicity(mut self, multiplicity: Multiplicity) -> Self {
        self.left_multiplicity = Some(multiplicity);
        self
    }

    /// Sets the right multiplicity of the edge.
    #[must_use]
    pub fn right_multiplicity(mut self, multiplicity: Multiplicity) -> Self {
        self.right_multiplicity = Some(multiplicity);
        self
    }
}

impl TryFrom<ClassEdgeBuilder> for ClassEdge {
    type Error = EdgeError;

    fn try_from(builder: ClassEdgeBuilder) -> Result<Self, Self::Error> {
        Ok(ClassEdge {
            edge: builder.edge_builder.try_into()?,
            left_multiplicity: builder.left_multiplicity,
            right_multiplicity: builder.right_multiplicity,
        })
    }
}

impl EdgeBuilder for ClassEdgeBuilder {
    type Edge = ClassEdge;
    type Node = ClassNode;
    type Error = EdgeError;

    fn build(self) -> Result<Self::Edge, Self::Error> {
        self.try_into()
    }

    fn source(mut self, node: std::rc::Rc<Self::Node>) -> Result<Self, Self::Error> {
        self.edge_builder = self.edge_builder.source(node)?;
        Ok(self)
    }

    fn destination(mut self, node: std::rc::Rc<Self::Node>) -> Result<Self, Self::Error> {
        self.edge_builder = self.edge_builder.destination(node)?;
        Ok(self)
    }

    fn label<S: ToString>(mut self, label: S) -> Result<Self, Self::Error> {
        self.edge_builder = self.edge_builder.label(label)?;
        Ok(self)
    }

    fn line_style(mut self, style: crate::shared::LineStyle) -> Self {
        self.edge_builder = self.edge_builder.line_style(style);
        self
    }

    fn left_arrow_shape(mut self, shape: crate::shared::ArrowShape) -> Result<Self, Self::Error> {
        self.edge_builder = self.edge_builder.left_arrow_shape(shape)?;
        Ok(self)
    }

    fn right_arrow_shape(mut self, shape: crate::shared::ArrowShape) -> Result<Self, Self::Error> {
        self.edge_builder = self.edge_builder.right_arrow_shape(shape)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;
    use crate::{
        diagrams::class_diagram::class_node::ClassNodeBuilder, shared::ArrowShape,
        traits::NodeBuilder,
    };

    #[test]
    fn test_class_edge_builder() -> Result<(), Box<dyn std::error::Error>> {
        let node1 = Rc::new(ClassNodeBuilder::default().label("A")?.id(0).build()?);
        let node2 = Rc::new(ClassNodeBuilder::default().label("B")?.id(1).build()?);

        let edge = ClassEdgeBuilder::default()
            .source(node1)?
            .destination(node2)?
            .left_multiplicity(Multiplicity::One)
            .right_multiplicity(Multiplicity::Many)
            .right_arrow_shape(ArrowShape::Triangle)?
            .build()?;

        assert_eq!(edge.left_multiplicity, Some(Multiplicity::One));
        assert_eq!(edge.right_multiplicity, Some(Multiplicity::Many));
        Ok(())
    }
}
