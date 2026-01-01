//! Submodule defining a builder struct for the class node in class diagrams.

use std::rc::Rc;

use crate::{
    diagrams::class_diagram::class_node::{ClassAttribute, ClassMethod, ClassNode},
    errors::NodeError,
    shared::{ClickEvent, StyleClass, StyleClassError, generic_node::GenericNodeBuilder},
    traits::NodeBuilder,
};

/// Builder for `ClassNode`.
///
/// # Example
///
/// ```
/// use mermaid_builder::prelude::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let node =
///         ClassNodeBuilder::default().label("MyClass")?.annotation("interface").id(0).build()?;
///     Ok(())
/// }
/// ```
#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassNodeBuilder {
    /// Underlying generic node builder.
    builder: GenericNodeBuilder,
    /// The click event associated with the node, if any.
    click_event: Option<ClickEvent>,
    /// The annotation of the class node, which usually
    /// contains functional information such as `trait`, `interface`, etc.
    annotation: Option<String>,
    /// Attributes of the class node.
    attributes: Vec<ClassAttribute>,
    /// Methods of the class node.
    methods: Vec<ClassMethod>,
}

impl ClassNodeBuilder {
    /// Sets the click event for the class node.
    #[must_use]
    pub fn click_event(mut self, click_event: ClickEvent) -> Self {
        self.click_event = Some(click_event);
        self
    }

    /// Sets the annotation for the class node.
    #[must_use]
    pub fn annotation<S: ToString + ?Sized>(mut self, annotation: &S) -> Self {
        self.annotation = Some(annotation.to_string());
        self
    }

    /// Adds an attribute to the class node.
    #[must_use]
    pub fn attribute(mut self, attribute: ClassAttribute) -> Self {
        self.attributes.push(attribute);
        self
    }

    /// Adds a method to the class node.
    #[must_use]
    pub fn method(mut self, method: ClassMethod) -> Self {
        self.methods.push(method);
        self
    }
}

impl TryFrom<ClassNodeBuilder> for ClassNode {
    type Error = NodeError;

    fn try_from(builder: ClassNodeBuilder) -> Result<Self, Self::Error> {
        Ok(ClassNode {
            node: builder.builder.try_into()?,
            click_event: builder.click_event,
            annotation: builder.annotation,
            attributes: builder.attributes,
            methods: builder.methods,
        })
    }
}

impl NodeBuilder for ClassNodeBuilder {
    type Node = ClassNode;
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
        diagrams::class_diagram::class_node::{ClassAttribute, ClassMethod},
        shared::{
            ClickEvent, StyleClassBuilder, StyleProperty, click_event::Navigation,
            style_class::Unit,
        },
        traits::node::Node,
    };

    #[test]
    fn test_class_node_builder() -> Result<(), Box<dyn std::error::Error>> {
        let style_class = Rc::new(
            StyleClassBuilder::default()
                .name("test")?
                .property(StyleProperty::StrokeWidth(Unit::Pixel(2)))?
                .build()?,
        );

        let node = ClassNodeBuilder::default()
            .id(1)
            .label("MyClass")?
            .annotation("interface")
            .attribute(ClassAttribute::new("int", "id"))
            .method(ClassMethod::new("void", "method", vec![]))
            .click_event(ClickEvent::Navigation(Navigation::new("https://example.com")))
            .style_class(style_class.clone())?
            .style_property(StyleProperty::StrokeWidth(Unit::Pixel(2)))?
            .build()?;

        assert_eq!(node.id(), 1);
        assert_eq!(node.label(), "MyClass");
        assert_eq!(node.annotation, Some("interface".to_string()));
        assert_eq!(node.attributes.len(), 1);
        assert_eq!(node.methods.len(), 1);
        assert!(matches!(node.click_event, Some(ClickEvent::Navigation(_))));
        assert_eq!(node.classes().count(), 1);
        assert_eq!(node.styles().count(), 1);

        Ok(())
    }
}
