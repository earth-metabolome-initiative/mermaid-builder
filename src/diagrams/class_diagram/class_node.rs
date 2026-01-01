//! Submodule defining a node struct for the class diagram in
//! Mermaid syntax.

mod builder;
mod class_attribute;
mod class_method;
use std::fmt::Display;

pub use builder::ClassNodeBuilder;
pub use class_attribute::ClassAttribute;
pub use class_method::ClassMethod;

use crate::{
    shared::{ClickEvent, GenericNode, NODE_LETTER, StyleClass, StyleProperty},
    traits::Node,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Struct representing a class node in a class diagram.
///
/// # Examples
///
/// ```
/// use mermaid_builder::{
///     diagrams::class_diagram::ClassNodeBuilder,
///     traits::{Node, NodeBuilder},
/// };
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let node =
///         ClassNodeBuilder::default().label("MyClass")?.id(1).annotation("interface").build()?;
///
///     assert_eq!(node.label(), "MyClass");
///     Ok(())
/// }
/// ```
pub struct ClassNode {
    /// Underlying generic node.
    node: GenericNode,
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

impl Node for ClassNode {
    type Builder = ClassNodeBuilder;

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
            crate::shared::ArrowShape::Triangle
                | crate::shared::ArrowShape::Star
                | crate::shared::ArrowShape::Circle
                | crate::shared::ArrowShape::Normal
        )
    }
}

impl Display for ClassNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::traits::TabbedDisplay;
        self.fmt_tabbed(f, 0)
    }
}

impl crate::traits::TabbedDisplay for ClassNode {
    fn fmt_tabbed(&self, f: &mut std::fmt::Formatter<'_>, tab_count: usize) -> std::fmt::Result {
        let indent = " ".repeat(tab_count * 2);
        writeln!(f, "{indent}class {NODE_LETTER}{}[\"{}\"] {{", self.id(), self.label())?;
        if let Some(annotation) = &self.annotation {
            writeln!(f, "{indent}    <<{annotation}>>")?;
        }

        for attr in &self.attributes {
            writeln!(f, "{indent}    {attr}")?;
        }
        for method in &self.methods {
            writeln!(f, "{indent}    {method}")?;
        }
        writeln!(f, "{indent}}}")?;

        if let Some(click_event) = &self.click_event {
            writeln!(f, "{indent}click {NODE_LETTER}{} {}", self.id(), click_event)?;
        }

        for class in self.classes() {
            writeln!(f, "{indent}cssClass {NODE_LETTER}{} {}", self.id(), class.name())?;
        }

        Ok(())
    }
}
