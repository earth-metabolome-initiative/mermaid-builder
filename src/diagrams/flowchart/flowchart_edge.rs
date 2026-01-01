//! Submodule defining an edge which may be used in a flowchart diagram
//! in Mermaid syntax.

use std::{fmt::Display, rc::Rc};

use crate::{
    diagrams::flowchart::{curve_styles::CurveStyle, flowchart_node::FlowchartNode},
    shared::{
        ArrowShape, EDGE_LETTER, GenericEdge, LineStyle, NODE_LETTER, StyleClass, StyleProperty,
    },
    traits::{Edge, node::Node},
};

pub mod builder;
pub use builder::FlowchartEdgeBuilder;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents an edge in a flowchart diagram, connecting two nodes with various
/// properties such as styles, classes, and curve styles.
///
/// # Examples
///
/// ```
/// use std::rc::Rc;
///
/// use mermaid_builder::{
///     diagrams::flowchart::{FlowchartEdgeBuilder, FlowchartNodeBuilder},
///     traits::{EdgeBuilder, NodeBuilder},
/// };
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let node1 = Rc::new(FlowchartNodeBuilder::default().label("A")?.id(1).build()?);
///     let node2 = Rc::new(FlowchartNodeBuilder::default().label("B")?.id(2).build()?);
///
///     let edge =
///         FlowchartEdgeBuilder::default().source(node1)?.destination(node2)?.id(1).build()?;
///     Ok(())
/// }
/// ```
pub struct FlowchartEdge {
    /// Unique identifier for the edge.
    id: usize,
    /// Underlying generic edge.
    edge: GenericEdge<FlowchartNode>,
    /// Classes associated with the edge, used for styling.
    style_classes: Vec<Rc<StyleClass>>,
    /// Styling properties for the edge, such as color and font.
    style_properties: Vec<StyleProperty>,
    /// The curve style of the edge.
    curve_style: CurveStyle,
    /// The number of segments composing the link style.
    length: u8,
}

impl Edge for FlowchartEdge {
    type Builder = FlowchartEdgeBuilder;
    type Node = FlowchartNode;

    fn label(&self) -> Option<&str> {
        self.edge.label()
    }

    fn source(&self) -> &Rc<Self::Node> {
        self.edge.source()
    }

    fn destination(&self) -> &Rc<Self::Node> {
        self.edge.destination()
    }

    fn classes(&self) -> impl Iterator<Item = &StyleClass> {
        self.style_classes.iter().map(AsRef::as_ref)
    }

    fn line_style(&self) -> LineStyle {
        self.edge.line_style()
    }

    fn left_arrow_shape(&self) -> Option<ArrowShape> {
        self.edge.left_arrow_shape()
    }

    fn right_arrow_shape(&self) -> Option<ArrowShape> {
        self.edge.right_arrow_shape()
    }
}

impl Display for FlowchartEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::traits::TabbedDisplay;
        self.fmt_tabbed(f, 0)
    }
}

impl crate::traits::TabbedDisplay for FlowchartEdge {
    fn fmt_tabbed(&self, f: &mut std::fmt::Formatter<'_>, tab_count: usize) -> std::fmt::Result {
        let indent = " ".repeat(tab_count * 2);
        let segment = match self.line_style() {
            LineStyle::Solid => "-".repeat(2 + self.length as usize),
            LineStyle::Thick => "=".repeat(2 + self.length as usize),
            LineStyle::Dashed => format!("-{}-", ".".repeat(self.length as usize)),
        };

        let edge_prefix = if self.curve_style != CurveStyle::default()
            || !self.style_classes.is_empty()
            || !self.style_properties.is_empty()
        {
            format!("{EDGE_LETTER}{}@", self.id)
        } else {
            String::default()
        };

        writeln!(
            f,
            "{indent}{NODE_LETTER}{} {edge_prefix}{left_arrow}{segment}{right_arrow}{} {NODE_LETTER}{}",
            self.source().id(),
            self.label().map_or_else(String::new, |label| format!("|\"`{label}`\"|")),
            self.destination().id(),
            left_arrow = self.left_arrow_shape().as_ref().map_or_else(|| "", |shape| shape.left()),
            right_arrow =
                self.right_arrow_shape().as_ref().map_or_else(|| "", |shape| shape.right()),
        )?;

        if self.curve_style != CurveStyle::default() {
            writeln!(f, "{indent}{EDGE_LETTER}{}@{{curve: {}}}", self.id, self.curve_style)?;
        }

        for class in &self.style_classes {
            writeln!(f, "{indent}class {EDGE_LETTER}{} {}", self.id, class.name())?;
        }

        if !self.style_properties.is_empty() {
            write!(f, "{indent}linkStyle {EDGE_LETTER}{} ", self.id)?;
            for (style_number, style) in self.style_properties.iter().enumerate() {
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
    use super::*;
    use crate::{
        diagrams::flowchart::flowchart_node::FlowchartNodeBuilder,
        shared::{StyleClassBuilder, style_class::Color},
        traits::{EdgeBuilder, NodeBuilder},
    };

    #[test]
    fn test_flowchart_edge_display() -> Result<(), Box<dyn std::error::Error>> {
        let node1 = Rc::new(FlowchartNodeBuilder::default().label("A")?.id(0).build()?);
        let node2 = Rc::new(FlowchartNodeBuilder::default().label("B")?.id(1).build()?);
        let style_class = Rc::new(
            StyleClassBuilder::default()
                .name("myStyle")?
                .property(StyleProperty::Stroke(Color::from((255, 0, 0))))?
                .build()?,
        );

        let edge = FlowchartEdgeBuilder::default()
            .id(1)
            .source(node1.clone())?
            .destination(node2.clone())?
            .label("Edge Label")?
            .line_style(LineStyle::Dashed)
            .left_arrow_shape(ArrowShape::Circle)?
            .right_arrow_shape(ArrowShape::X)?
            .curve_style(CurveStyle::StepAfter)
            .length(2)
            .style_class(style_class.clone())?
            .style_property(StyleProperty::Stroke(Color::from((255, 0, 0))))?
            .build()?;

        let output = format!("{edge}");
        assert!(output.contains("v0 e1@o-..-x|\"`Edge Label`\"| v1"));
        assert!(output.contains("e1@{curve: stepAfter}"));
        assert!(output.contains("class e1 myStyle"));
        assert!(output.contains("linkStyle e1 stroke: #ff0000"));

        Ok(())
    }

    #[test]
    fn test_flowchart_edge_traits() -> Result<(), Box<dyn std::error::Error>> {
        let node1 = Rc::new(FlowchartNodeBuilder::default().label("A")?.id(0).build()?);
        let node2 = Rc::new(FlowchartNodeBuilder::default().label("B")?.id(1).build()?);
        let style_class = Rc::new(
            StyleClassBuilder::default()
                .name("myStyle")?
                .property(StyleProperty::Stroke(Color::from((255, 0, 0))))?
                .build()?,
        );

        let edge = FlowchartEdgeBuilder::default()
            .id(1)
            .source(node1.clone())?
            .destination(node2.clone())?
            .style_class(style_class.clone())?
            .build()?;

        assert_eq!(edge.classes().count(), 1);
        assert_eq!(edge.classes().next().ok_or("No class found")?.name(), "myStyle");
        Ok(())
    }
}
