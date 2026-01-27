//! Submodule defining a node which may be used in a flowchart diagram
//! in Mermaid syntax.

mod builder;
mod shape;
use alloc::{rc::Rc, vec::Vec};
use core::fmt::{self, Display};

pub use builder::FlowchartNodeBuilder;
pub use shape::FlowchartNodeShape;

use crate::{
    shared::{
        ClickEvent, GenericNode, NODE_LETTER, StyleClass, generic_configuration::Direction,
        style_class::StyleProperty,
    },
    traits::Node,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a node in a flowchart diagram, which can have various
/// properties and may include click events.
///
/// # Examples
///
/// ```
/// use mermaid_builder::{
///     diagrams::flowchart::FlowchartNodeBuilder,
///     traits::{Node, NodeBuilder},
/// };
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let node = FlowchartNodeBuilder::default().label("My Node")?.id(1).build()?;
///
///     assert_eq!(node.label(), "My Node");
///     Ok(())
/// }
/// ```
pub struct FlowchartNode {
    /// Underlying node structure.
    node: GenericNode,
    /// The click event associated with the node, if any.
    click_event: Option<ClickEvent>,
    /// The shape of the node, which can be customized.
    shape: FlowchartNodeShape,
    /// The sub-nodes, when the node is a subgraph.
    subnodes: Vec<Rc<FlowchartNode>>,
    /// The direction of the subgraph, if applicable.
    direction: Option<Direction>,
}

impl FlowchartNode {
    /// Returns an iterator over the subnodes of the flowchart node.
    pub fn subnodes(&self) -> impl Iterator<Item = &FlowchartNode> {
        self.subnodes.iter().map(AsRef::as_ref)
    }
}

impl Node for FlowchartNode {
    type Builder = FlowchartNodeBuilder;

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
            crate::shared::ArrowShape::Normal
                | crate::shared::ArrowShape::Circle
                | crate::shared::ArrowShape::X
        )
    }
}

impl Display for FlowchartNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::traits::TabbedDisplay;
        self.fmt_tabbed(f, 0)
    }
}

impl crate::traits::TabbedDisplay for FlowchartNode {
    fn fmt_tabbed(&self, f: &mut fmt::Formatter<'_>, tab_count: usize) -> fmt::Result {
        let indent = " ".repeat(tab_count * 2);
        if self.subnodes.is_empty() {
            writeln!(
                f,
                "{indent}{NODE_LETTER}{}@{{shape: {}, label: \"{}\"}}",
                self.id(),
                self.shape,
                self.label()
            )?;

            if let Some(click_event) = &self.click_event {
                writeln!(f, "{indent}click {NODE_LETTER}{} {click_event}", self.id(),)?;
            }

            for class in self.classes() {
                writeln!(f, "{indent}class {NODE_LETTER}{} {}", self.id(), class.name())?;
            }
        } else {
            writeln!(f, "{indent}subgraph {NODE_LETTER}{} [\"`{}`\"]", self.id(), self.label())?;
            if let Some(direction) = &self.direction {
                writeln!(f, "{indent}    direction {direction}")?;
            }

            for node in &self.subnodes {
                node.fmt_tabbed(f, tab_count + 1)?;
            }
            writeln!(f, "{indent}end")?;
        }
        if self.has_styles() {
            write!(f, "{indent}style {NODE_LETTER}{} ", self.id())?;
            for (style_number, style) in self.styles().enumerate() {
                if style_number > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{style} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use alloc::{boxed::Box, format};

    use super::*;
    use crate::{
        shared::{
            ClickEvent, StyleClassBuilder, StyleProperty, click_event::Navigation,
            style_class::Color,
        },
        traits::NodeBuilder,
    };

    #[test]
    fn test_flowchart_node_display_simple() -> Result<(), Box<dyn core::error::Error>> {
        let node = FlowchartNodeBuilder::default()
            .label("My Node")?
            .id(1)
            .shape(FlowchartNodeShape::Circle)
            .build()?;

        let output = format!("{node}");
        assert!(output.contains("v1@{shape: circle, label: \"My Node\"}"));
        Ok(())
    }

    #[test]
    fn test_flowchart_node_display_full() -> Result<(), Box<dyn core::error::Error>> {
        let style_class = Rc::new(
            StyleClassBuilder::default()
                .name("myClass")?
                .property(StyleProperty::Fill(Color::from((255, 0, 0))))?
                .build()?,
        );

        let node = FlowchartNodeBuilder::default()
            .label("My Node")?
            .id(1)
            .shape(FlowchartNodeShape::Rectangle)
            .style_class(style_class)?
            .style_property(StyleProperty::Stroke(Color::from((0, 0, 255))))?
            .click_event(ClickEvent::Navigation(
                Navigation::new("https://example.com").anchor(true).tooltip("Open link"),
            ))
            .build()?;

        let output = format!("{node}");
        assert!(output.contains("v1@{shape: rect, label: \"My Node\"}"));
        assert!(output.contains("click v1 href \"https://example.com\" \"Open link\""));
        assert!(output.contains("class v1 myClass"));
        assert!(output.contains("style v1 stroke: #0000ff"));
        Ok(())
    }

    #[test]
    fn test_flowchart_node_subgraph() -> Result<(), Box<dyn core::error::Error>> {
        let subnode = Rc::new(FlowchartNodeBuilder::default().label("Sub Node")?.id(2).build()?);

        let node = FlowchartNodeBuilder::default()
            .label("My Subgraph")?
            .id(1)
            .subnode(subnode)?
            .direction(Direction::LeftToRight)
            .build()?;

        let output = format!("{node}");
        assert!(output.contains("subgraph v1 [\"`My Subgraph`\"]"));
        assert!(output.contains("direction LR"));
        assert!(output.contains("v2@{shape: rect, label: \"Sub Node\"}"));
        assert!(output.contains("end"));
        Ok(())
    }
}
