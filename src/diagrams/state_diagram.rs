//! Submodule providing structs for creating state diagrams in Mermaid syntax (stateDiagram-v2).

use crate::shared::StyleClass;
use crate::shared::generic_configuration::{
    Direction, GenericConfiguration, GenericConfigurationBuilder, Look, Renderer, Theme,
};
use crate::shared::generic_diagram::{GenericDiagram, GenericDiagramBuilder};
use crate::shared::generic_edge::{GenericEdge, GenericEdgeBuilder};
use crate::shared::generic_node::{GenericNode, GenericNodeBuilder};
use crate::shared::style_class::StyleProperty;
use crate::traits::{
    Configuration, ConfigurationBuilder, Diagram, DiagramBuilder, Node, NodeBuilder, TabbedDisplay,
};
use alloc::{rc::Rc, string::String, string::ToString};
use core::fmt::{self, Display};

// ============================================================================
// 1. Configuration
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Configuration for a state diagram.
pub struct StateDiagramConfiguration {
    generic: GenericConfiguration,
}

impl Configuration for StateDiagramConfiguration {
    type Builder = StateDiagramConfigurationBuilder;
    fn title(&self) -> Option<&str> {
        self.generic.title()
    }
    fn renderer(&self) -> Renderer {
        self.generic.renderer()
    }
    fn direction(&self) -> Direction {
        self.generic.direction()
    }
    fn theme(&self) -> Theme {
        self.generic.theme()
    }
    fn look(&self) -> Look {
        self.generic.look()
    }
}

impl Display for StateDiagramConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(title) = self.title() {
            writeln!(f, "---\ntitle: {title}\n---")?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
/// Builder for state diagram configuration.
pub struct StateDiagramConfigurationBuilder {
    generic: GenericConfigurationBuilder,
}

impl ConfigurationBuilder for StateDiagramConfigurationBuilder {
    type Configuration = StateDiagramConfiguration;
    type Error = crate::errors::Error;
    fn build(self) -> Result<Self::Configuration, Self::Error> {
        Ok(StateDiagramConfiguration { generic: self.generic.build()? })
    }
    fn title<S: ToString>(mut self, title: S) -> Result<Self, Self::Error> {
        self.generic = self.generic.title(title.to_string())?;
        Ok(self)
    }
    fn renderer(mut self, renderer: Renderer) -> Self {
        self.generic = self.generic.renderer(renderer);
        self
    }
    fn direction(mut self, direction: Direction) -> Self {
        self.generic = self.generic.direction(direction);
        self
    }
}

// ============================================================================
// 2. Node
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a node in a state diagram.
pub struct StateNode {
    node: GenericNode,
    inner_diagram: Option<StateDiagram>,
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
    fn is_compatible_arrow_shape(_: crate::shared::ArrowShape) -> bool {
        true
    }
}

impl Display for StateNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_tabbed(f, 0)
    }
}

impl TabbedDisplay for StateNode {
    fn fmt_tabbed(&self, f: &mut fmt::Formatter<'_>, tab_count: usize) -> fmt::Result {
        if let Some(inner) = &self.inner_diagram {
            // Mermaid stateDiagram-v2 rejects empty composite states (`state X { }`).
            if inner.nodes().next().is_none() && inner.edges().next().is_none() {
                return Ok(());
            }
            let indent = "    ".repeat(tab_count);
            writeln!(f, "{}state {} {{", indent, self.label())?;
            inner.fmt_tabbed(f, tab_count + 1)?;
            writeln!(f, "{indent}}}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
/// Builder for a state node.
pub struct StateNodeBuilder {
    generic: GenericNodeBuilder,
    inner_diagram: Option<StateDiagram>,
}

impl NodeBuilder for StateNodeBuilder {
    type Node = StateNode;
    type Error = crate::errors::Error;
    fn build(self) -> Result<Self::Node, Self::Error> {
        Ok(StateNode { node: self.generic.build()?, inner_diagram: self.inner_diagram })
    }
    fn label<S: ToString>(mut self, label: S) -> Result<Self, Self::Error> {
        self.generic = self.generic.label(label.to_string())?;
        Ok(self)
    }
    fn id(mut self, id: u64) -> Self {
        self.generic = self.generic.id(id);
        self
    }
    fn get_id(&self) -> Option<u64> {
        self.generic.get_id()
    }
    fn style_property(mut self, p: StyleProperty) -> Result<Self, crate::errors::StyleClassError> {
        self.generic = self.generic.style_property(p)?;
        Ok(self)
    }
    fn style_class(mut self, c: Rc<StyleClass>) -> Result<Self, crate::errors::StyleClassError> {
        self.generic = self.generic.style_class(c)?;
        Ok(self)
    }
    fn style_properties(&self) -> impl Iterator<Item = &StyleProperty> {
        self.generic.style_properties()
    }
    fn get_label(&self) -> Option<&String> {
        self.generic.get_label()
    }
}

impl StateNodeBuilder {
    /// Sets the inner diagram for this state node.
    ///
    /// # Errors
    ///
    /// Returns an error if the builder rejects the provided nested diagram.
    pub fn inner_diagram(mut self, diagram: StateDiagram) -> Result<Self, crate::errors::Error> {
        self.inner_diagram = Some(diagram);
        Ok(self)
    }
}

// ============================================================================
// 3. Edge
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents an edge in a state diagram.
pub struct StateEdge {
    edge: GenericEdge<StateNode>,
}

impl crate::traits::edge::Edge for StateEdge {
    type Node = StateNode;
    type Builder = StateEdgeBuilder;
    fn source(&self) -> &Rc<Self::Node> {
        self.edge.source()
    }
    fn destination(&self) -> &Rc<Self::Node> {
        self.edge.destination()
    }
    fn label(&self) -> Option<&str> {
        self.edge.label()
    }
    fn classes(&self) -> impl Iterator<Item = &StyleClass> {
        self.edge.classes()
    }
    fn line_style(&self) -> crate::shared::LineStyle {
        self.edge.line_style()
    }
    fn left_arrow_shape(&self) -> Option<crate::shared::ArrowShape> {
        self.edge.left_arrow_shape()
    }
    fn right_arrow_shape(&self) -> Option<crate::shared::ArrowShape> {
        self.edge.right_arrow_shape()
    }
}

impl Display for StateEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_tabbed(f, 0)
    }
}

impl TabbedDisplay for StateEdge {
    fn fmt_tabbed(&self, f: &mut fmt::Formatter<'_>, tab_count: usize) -> fmt::Result {
        use crate::traits::edge::Edge;
        let indent = "    ".repeat(tab_count);
        write!(f, "{}{} --> {}", indent, self.source().label(), self.destination().label())?;
        if let Some(label) = self.label() {
            // Mermaid stateDiagram-v2 treats `:` as the description terminator, so any
            // colon inside the label (e.g. Rust paths like `Foo::Bar`) must be encoded
            // as the `#colon;` entity that the renderer turns back into `:`.
            let clean =
                label.replace(|c: char| c.is_whitespace() && c != ' ', " ").replace(':', "#colon;");
            write!(f, " : {clean}")?;
        }
        writeln!(f)
    }
}

#[derive(Debug, Default, Clone)]
/// Builder for a state edge.
pub struct StateEdgeBuilder {
    generic: GenericEdgeBuilder<StateNode>,
}

impl crate::traits::EdgeBuilder for StateEdgeBuilder {
    type Edge = StateEdge;
    type Node = StateNode;
    type Error = crate::errors::Error;
    fn build(self) -> Result<Self::Edge, Self::Error> {
        Ok(StateEdge { edge: self.generic.build()? })
    }
    fn source(mut self, s: Rc<Self::Node>) -> Result<Self, Self::Error> {
        self.generic = self.generic.source(s)?;
        Ok(self)
    }
    fn destination(mut self, d: Rc<Self::Node>) -> Result<Self, Self::Error> {
        self.generic = self.generic.destination(d)?;
        Ok(self)
    }
    fn label<S: ToString>(mut self, l: S) -> Result<Self, Self::Error> {
        self.generic = self.generic.label(l.to_string())?;
        Ok(self)
    }
    fn line_style(mut self, s: crate::shared::LineStyle) -> Self {
        self.generic = self.generic.line_style(s);
        self
    }
    fn left_arrow_shape(mut self, s: crate::shared::ArrowShape) -> Result<Self, Self::Error> {
        self.generic = self.generic.left_arrow_shape(s)?;
        Ok(self)
    }
    fn right_arrow_shape(mut self, s: crate::shared::ArrowShape) -> Result<Self, Self::Error> {
        self.generic = self.generic.right_arrow_shape(s)?;
        Ok(self)
    }
}

// ============================================================================
// 4. Diagram
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a state diagram.
pub struct StateDiagram {
    generic: GenericDiagram<StateNode, StateEdge, StateDiagramConfiguration>,
}

impl From<GenericDiagram<StateNode, StateEdge, StateDiagramConfiguration>> for StateDiagram {
    fn from(generic: GenericDiagram<StateNode, StateEdge, StateDiagramConfiguration>) -> Self {
        Self { generic }
    }
}

impl From<StateDiagramBuilder> for StateDiagram {
    fn from(builder: StateDiagramBuilder) -> Self {
        Self { generic: builder.into() }
    }
}

impl Diagram for StateDiagram {
    type Builder = StateDiagramBuilder;
    type Configuration = StateDiagramConfiguration;
    type Edge = StateEdge;
    type Node = StateNode;
    fn configuration(&self) -> &Self::Configuration {
        self.generic.configuration()
    }
    fn edges(&self) -> impl Iterator<Item = &Self::Edge> {
        self.generic.edges()
    }
    fn get_node_by_id(&self, id: u64) -> Option<Rc<Self::Node>> {
        self.generic.get_node_by_id(id)
    }
    fn get_style_class_by_name(&self, name: &str) -> Option<Rc<crate::shared::StyleClass>> {
        self.generic.get_style_class_by_name(name)
    }
    fn nodes(&self) -> impl Iterator<Item = &Self::Node> {
        self.generic.nodes()
    }
    fn style_classes(&self) -> impl Iterator<Item = &crate::shared::StyleClass> {
        self.generic.style_classes()
    }
}

impl Display for StateDiagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_tabbed(f, 0)
    }
}

impl TabbedDisplay for StateDiagram {
    fn fmt_tabbed(&self, f: &mut fmt::Formatter<'_>, tab_count: usize) -> fmt::Result {
        // At the root, emit the title block + `stateDiagram-v2` header + direction line,
        // then indent body content one level under the header. When nested inside a
        // composite state, `tab_count` already encodes the depth, so we use it directly.
        let body_tab = if tab_count == 0 {
            write!(f, "{}", self.configuration())?;
            writeln!(f, "stateDiagram-v2")?;
            writeln!(f, "    direction {}", self.configuration().direction())?;
            writeln!(f)?;
            1
        } else {
            tab_count
        };

        for node in self.nodes() {
            node.fmt_tabbed(f, body_tab)?;
        }
        for edge in self.edges() {
            edge.fmt_tabbed(f, body_tab)?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
/// Builder for a state diagram.
pub struct StateDiagramBuilder {
    generic: GenericDiagramBuilder<StateNode, StateEdge, StateDiagramConfiguration>,
}

impl DiagramBuilder for StateDiagramBuilder {
    type Diagram = StateDiagram;
    type Node = StateNode;
    type NodeBuilder = StateNodeBuilder;
    type Edge = StateEdge;
    type EdgeBuilder = StateEdgeBuilder;
    type Configuration = StateDiagramConfiguration;
    type ConfigurationBuilder = StateDiagramConfigurationBuilder;
    type Error = crate::errors::Error;

    fn configuration(mut self, c: Self::ConfigurationBuilder) -> Result<Self, Self::Error> {
        self.generic = self.generic.configuration(c)?;
        Ok(self)
    }
    fn edge(&mut self, e: Self::EdgeBuilder) -> Result<Rc<Self::Edge>, Self::Error> {
        self.generic.edge(e)
    }
    fn node(&mut self, n: Self::NodeBuilder) -> Result<Rc<Self::Node>, Self::Error> {
        self.generic.node(n)
    }
    fn nodes(&self) -> impl Iterator<Item = &Rc<Self::Node>> {
        self.generic.nodes()
    }
    fn get_node_by_id(&self, id: u64) -> Option<Rc<Self::Node>> {
        self.generic.get_node_by_id(id)
    }
    fn get_style_class_by_name(&self, name: &str) -> Option<Rc<StyleClass>> {
        self.generic.get_style_class_by_name(name)
    }
    fn number_of_nodes(&self) -> usize {
        self.generic.number_of_nodes()
    }
    fn number_of_edges(&self) -> usize {
        self.generic.number_of_edges()
    }
    fn style_class(
        &mut self,
        b: crate::shared::StyleClassBuilder,
    ) -> Result<Rc<StyleClass>, Self::Error> {
        self.generic.style_class(b)
    }
}

impl From<StateDiagramBuilder> for GenericDiagram<StateNode, StateEdge, StateDiagramConfiguration> {
    fn from(builder: StateDiagramBuilder) -> Self {
        builder.generic.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use alloc::boxed::Box;
    use alloc::string::ToString;

    #[test]
    fn test_state_diagram_formatting() -> Result<(), Box<dyn core::error::Error>> {
        let mut builder = StateDiagramBuilder::default();
        let s1 = builder.node(StateNodeBuilder::default().label("S1")?)?;
        let s2 = builder.node(StateNodeBuilder::default().label("S2")?)?;
        builder.edge(StateEdgeBuilder::default().source(s1)?.destination(s2)?.label("E1")?)?;

        let diagram = StateDiagram::from(builder);
        let output = diagram.to_string();

        assert!(output.contains("stateDiagram-v2\n"));
        assert!(output.contains("S1 --> S2 : E1\n"));
        Ok(())
    }

    #[test]
    fn test_nested_state_diagram_formatting() -> Result<(), Box<dyn core::error::Error>> {
        let mut inner_builder = StateDiagramBuilder::default();
        let low = inner_builder.node(StateNodeBuilder::default().label("Low")?)?;
        let high = inner_builder.node(StateNodeBuilder::default().label("High")?)?;
        inner_builder
            .edge(StateEdgeBuilder::default().source(low)?.destination(high)?.label("Up")?)?;

        let mut builder = StateDiagramBuilder::default();
        builder.node(
            StateNodeBuilder::default()
                .label("Playing")?
                .inner_diagram(StateDiagram::from(inner_builder))?,
        )?;

        let diagram = StateDiagram::from(builder);
        let output = diagram.to_string();

        assert!(output.contains("stateDiagram-v2\n"));
        assert!(output.contains("state Playing {\n"));
        assert!(output.contains("    Low --> High : Up\n"));
        assert!(output.contains("}\n"));
        Ok(())
    }
}
