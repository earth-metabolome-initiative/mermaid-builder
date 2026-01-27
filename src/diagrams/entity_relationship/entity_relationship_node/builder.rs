//! Submodule defining a builder struct for the entity-relationship node in
//! entity-relationship diagrams.

use alloc::{
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};

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
/// extern crate alloc;
/// use alloc::boxed::Box;
///
/// use mermaid_builder::prelude::*;
///
/// fn main() -> Result<(), Box<dyn core::error::Error>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        shared::{StyleClassBuilder, StyleProperty, style_class::Color},
        traits::node::Node,
    };

    #[test]
    fn test_er_node_builder() -> Result<(), alloc::boxed::Box<dyn core::error::Error>> {
        let style = Rc::new(
            StyleClassBuilder::default()
                .name("myStyle")?
                .property(StyleProperty::Fill(Color::from((255, 0, 0))))?
                .build()?,
        );

        let node = ERNodeBuilder::default()
            .id(1)
            .label("CUSTOMER")?
            .attribute("string", "name")
            .attribute("int", "age")
            .style_class(style)?
            .style_property(StyleProperty::Fill(Color::from((255, 0, 0))))?
            .build()?;

        assert_eq!(node.node.id(), 1);
        assert_eq!(node.node.label(), "CUSTOMER");
        assert_eq!(node.attributes.len(), 2);
        assert_eq!(node.attributes[0].attribute_type(), "string");
        assert_eq!(node.attributes[0].name(), "name");
        assert_eq!(node.attributes[1].attribute_type(), "int");
        assert_eq!(node.attributes[1].name(), "age");

        // Check styles
        let styles: Vec<_> = node.node.styles().collect();
        assert!(styles.contains(&&StyleProperty::Fill(Color::from((255, 0, 0)))));

        Ok(())
    }
}
