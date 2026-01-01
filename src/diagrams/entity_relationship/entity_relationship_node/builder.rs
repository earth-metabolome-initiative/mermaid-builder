//! Submodule defining a builder struct for the entity-relationship node in
//! entity-relationship diagrams.

use std::rc::Rc;

use crate::{
    diagrams::entity_relationship::entity_relationship_node::{
        ERNode, attribute::EntityRelationshipAttribute,
    },
    errors::NodeError,
    shared::{StyleClass, StyleClassError, generic_node::GenericNodeBuilder},
    traits::NodeBuilder,
};

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for the entity-relationship node in Mermaid syntax.
///
/// # Example
///
/// ```
/// use mermaid_builder::prelude::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let node = ERNodeBuilder::default().label("CUSTOMER")?.id(0).build()?;
///     Ok(())
/// }
/// ```
pub struct ERNodeBuilder {
    /// Shared attributes builder for the node.
    builder: GenericNodeBuilder,
    /// The attributes of the entity-relationship node.
    class_attributes: Vec<EntityRelationshipAttribute>,
}

impl ERNodeBuilder {
    /// Adds an attribute to the entity-relationship node.
    #[must_use]
    pub fn attribute<S: ToString + ?Sized>(mut self, attribute_type: &S, name: &S) -> Self {
        self.class_attributes
            .push(EntityRelationshipAttribute::new(attribute_type.to_string(), name.to_string()));
        self
    }
}

impl TryFrom<ERNodeBuilder> for ERNode {
    type Error = NodeError;

    fn try_from(builder: ERNodeBuilder) -> Result<Self, Self::Error> {
        Ok(ERNode { node: builder.builder.try_into()?, attributes: builder.class_attributes })
    }
}

impl NodeBuilder for ERNodeBuilder {
    type Node = ERNode;
    type Error = NodeError;

    fn build(self) -> Result<Self::Node, Self::Error> {
        self.try_into()
    }

    fn id(mut self, id: u64) -> Self {
        self.builder = self.builder.id(id);
        self
    }

    fn get_id(&self) -> Option<u64> {
        self.builder.get_id()
    }

    fn label<S: ToString>(mut self, label: S) -> Result<Self, Self::Error> {
        self.builder = self.builder.label(label)?;
        Ok(self)
    }

    fn get_label(&self) -> Option<&String> {
        self.builder.get_label()
    }

    fn style_class(mut self, style_class: Rc<StyleClass>) -> Result<Self, StyleClassError> {
        self.builder = self.builder.style_class(style_class)?;
        Ok(self)
    }

    fn style_property(
        mut self,
        property: crate::shared::StyleProperty,
    ) -> Result<Self, StyleClassError> {
        self.builder = self.builder.style_property(property)?;
        Ok(self)
    }

    fn style_properties(&self) -> impl Iterator<Item = &crate::prelude::StyleProperty> {
        self.builder.style_properties()
    }
}
