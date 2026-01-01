//! Submodule defining a generic diagram struct which can be used as a base
//! for various types of diagrams in Mermaid syntax.

use std::{fmt::Display, rc::Rc};

use crate::{
    shared::{StyleClass, StyleClassError},
    traits::{
        Configuration, ConfigurationBuilder, Diagram, DiagramBuilder, Edge, EdgeBuilder, Node,
        NodeBuilder,
    },
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A generic diagram struct that can be extended for specific diagram types.
pub struct GenericDiagram<Node, Edge, Config> {
    /// Style classes associated with this diagram.
    style_classes: Vec<Rc<StyleClass>>,
    /// Nodes in the diagram.
    nodes: Vec<Rc<Node>>,
    /// Edges in the diagram.
    edges: Vec<Rc<Edge>>,
    /// Configuration options for the diagram.
    configuration: Config,
}

impl<N: Node + Display, E: Edge<Node = N> + Display, C: Configuration> Diagram
    for GenericDiagram<N, E, C>
where
    crate::errors::Error: From<<N::Builder as NodeBuilder>::Error>
        + From<<E::Builder as EdgeBuilder>::Error>
        + From<<C::Builder as ConfigurationBuilder>::Error>,
{
    type Builder = GenericDiagramBuilder<N, E, C>;
    type Node = N;
    type Edge = E;
    type Configuration = C;

    fn configuration(&self) -> &Self::Configuration {
        &self.configuration
    }

    fn nodes(&self) -> impl Iterator<Item = &Self::Node> {
        self.nodes.iter().map(AsRef::as_ref)
    }

    fn edges(&self) -> impl Iterator<Item = &Self::Edge> {
        self.edges.iter().map(AsRef::as_ref)
    }

    fn style_classes(&self) -> impl Iterator<Item = &StyleClass> {
        self.style_classes.iter().map(AsRef::as_ref)
    }

    fn get_node_by_id(&self, id: u64) -> Option<Rc<Self::Node>> {
        self.nodes.iter().find(|node| node.id() == id).cloned()
    }

    fn get_style_class_by_name(&self, name: &str) -> Option<Rc<StyleClass>> {
        self.style_classes.iter().find(|sc| sc.name() == name).cloned()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A builder for creating a generic diagram.
pub struct GenericDiagramBuilder<Node, Edge, Config> {
    /// Underlying generic diagram.
    generic_diagram: GenericDiagram<Node, Edge, Config>,
}

impl<Node, Edge, Config: Default> Default for GenericDiagramBuilder<Node, Edge, Config> {
    fn default() -> Self {
        Self {
            generic_diagram: GenericDiagram {
                style_classes: Vec::new(),
                nodes: Vec::new(),
                edges: Vec::new(),
                configuration: Config::default(),
            },
        }
    }
}

impl<N: Node + Display, E: Edge<Node = N> + Display, C: Configuration>
    From<GenericDiagramBuilder<N, E, C>> for GenericDiagram<N, E, C>
{
    fn from(builder: GenericDiagramBuilder<N, E, C>) -> Self {
        builder.generic_diagram
    }
}

impl<N: Node + Display, E: Edge<Node = N> + Display, C: Configuration> DiagramBuilder
    for GenericDiagramBuilder<N, E, C>
where
    crate::errors::Error: From<<N::Builder as NodeBuilder>::Error>
        + From<<E::Builder as EdgeBuilder>::Error>
        + From<<C::Builder as ConfigurationBuilder>::Error>,
    GenericDiagram<N, E, C>: Diagram<Node = N, Edge = E, Configuration = C, Builder = Self>,
{
    type Diagram = GenericDiagram<N, E, C>;
    type Node = N;
    type NodeBuilder = N::Builder;
    type Edge = E;
    type EdgeBuilder = E::Builder;
    type Configuration = C;
    type ConfigurationBuilder = C::Builder;
    type Error = crate::errors::Error;

    fn configuration(
        mut self,
        configuration: Self::ConfigurationBuilder,
    ) -> Result<Self, Self::Error> {
        self.generic_diagram.configuration =
            configuration.build().map_err(crate::errors::Error::from)?;
        Ok(self)
    }

    fn edge(&mut self, edge: Self::EdgeBuilder) -> Result<Rc<Self::Edge>, Self::Error> {
        let edge = edge.build().map_err(crate::errors::Error::from)?;

        if !self.generic_diagram.nodes.contains(edge.source()) {
            return Err(crate::errors::EdgeError::SourceNodeNotFound(
                edge.source().label().to_owned(),
            )
            .into());
        }

        if !self.generic_diagram.nodes.contains(edge.destination()) {
            return Err(crate::errors::EdgeError::DestinationNodeNotFound(
                edge.destination().label().to_owned(),
            )
            .into());
        }

        let rc = Rc::new(edge);
        self.generic_diagram.edges.push(rc.clone());
        Ok(rc)
    }

    fn node(&mut self, mut node: Self::NodeBuilder) -> Result<Rc<Self::Node>, Self::Error> {
        let number_of_nodes = self.generic_diagram.nodes.len();
        {
            use crate::traits::node_builder::NodeBuilder;
            if node.get_id().is_none() {
                node = node.id(number_of_nodes as u64);
            }
        }
        let node = node.build().map_err(crate::errors::Error::from)?;

        for class in node.classes() {
            if !self.generic_diagram.style_classes.iter().any(|sc| sc.name() == class.name()) {
                return Err(StyleClassError::UnknownClass(class.clone()).into());
            }
        }

        let rc = Rc::new(node);
        self.generic_diagram.nodes.push(rc.clone());
        Ok(rc)
    }

    fn nodes(&self) -> impl Iterator<Item = &Rc<Self::Node>> + '_ {
        self.generic_diagram.nodes.iter()
    }

    fn get_node_by_id(&self, id: u64) -> Option<Rc<Self::Node>> {
        self.generic_diagram.get_node_by_id(id)
    }

    fn get_style_class_by_name(&self, name: &str) -> Option<Rc<StyleClass>> {
        self.generic_diagram.get_style_class_by_name(name)
    }

    fn number_of_nodes(&self) -> usize {
        self.generic_diagram.nodes.len()
    }

    fn number_of_edges(&self) -> usize {
        self.generic_diagram.edges.len()
    }

    fn style_class(
        &mut self,
        style_class: super::StyleClassBuilder,
    ) -> Result<Rc<StyleClass>, Self::Error> {
        let style_class = style_class.build().map_err(crate::errors::Error::from)?;

        if self.generic_diagram.style_classes.iter().any(|sc| sc.name() == style_class.name()) {
            return Err(StyleClassError::DuplicateClass(style_class.name().to_owned()).into());
        }

        let rc = Rc::new(style_class);
        self.generic_diagram.style_classes.push(rc.clone());
        Ok(rc)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;
    use crate::{
        diagrams::flowchart::{
            FlowchartConfiguration, FlowchartConfigurationBuilder, FlowchartEdge,
            FlowchartEdgeBuilder, FlowchartNode, FlowchartNodeBuilder,
        },
        shared::{StyleClassBuilder, StyleProperty, style_class::Unit},
        traits::{ConfigurationBuilder, EdgeBuilder, NodeBuilder},
    };

    #[test]
    fn test_generic_diagram_builder() -> Result<(), Box<dyn std::error::Error>> {
        let mut builder =
            GenericDiagramBuilder::<FlowchartNode, FlowchartEdge, FlowchartConfiguration>::default(
            );

        // Test Node
        let node_builder = FlowchartNodeBuilder::default().label("Node 1")?.id(1);
        let node1 = builder.node(node_builder)?;
        assert_eq!(node1.id(), 1);

        let node_builder2 = FlowchartNodeBuilder::default().label("Node 2")?.id(2);
        let node2 = builder.node(node_builder2)?;
        assert_eq!(node2.id(), 2);

        // Test Edge
        let edge_builder = FlowchartEdgeBuilder::default()
            .source(node1.clone())?
            .destination(node2.clone())?
            .id(1);
        let edge = builder.edge(edge_builder)?;
        assert_eq!(edge.source().id(), 1);
        assert_eq!(edge.destination().id(), 2);

        // Test Config
        let config_builder = FlowchartConfigurationBuilder::default().title("My Diagram")?;
        builder = builder.configuration(config_builder)?;

        let diagram: GenericDiagram<FlowchartNode, FlowchartEdge, FlowchartConfiguration> =
            builder.into();

        assert_eq!(diagram.nodes().count(), 2);
        assert_eq!(diagram.edges().count(), 1);
        assert_eq!(diagram.configuration().title(), Some("My Diagram"));

        Ok(())
    }

    #[test]
    fn test_generic_diagram_methods() -> Result<(), Box<dyn std::error::Error>> {
        let mut builder =
            GenericDiagramBuilder::<FlowchartNode, FlowchartEdge, FlowchartConfiguration>::default(
            );

        let node_builder = FlowchartNodeBuilder::default().label("Node 1")?.id(1);
        builder.node(node_builder)?;

        // Test Builder methods
        assert_eq!(builder.number_of_nodes(), 1);
        assert_eq!(builder.number_of_edges(), 0);
        assert!(builder.get_node_by_id(1).is_some());
        assert!(builder.get_node_by_id(2).is_none());
        assert_eq!(builder.nodes().count(), 1);

        let diagram: GenericDiagram<FlowchartNode, FlowchartEdge, FlowchartConfiguration> =
            builder.into();

        assert_eq!(diagram.nodes().count(), 1);
        assert_eq!(diagram.edges().count(), 0);
        assert!(diagram.get_node_by_id(1).is_some());
        assert!(diagram.get_node_by_id(2).is_none());

        Ok(())
    }

    #[test]
    fn test_style_class_management() -> Result<(), Box<dyn std::error::Error>> {
        let mut builder =
            GenericDiagramBuilder::<FlowchartNode, FlowchartEdge, FlowchartConfiguration>::default(
            );

        let style_class_builder = StyleClassBuilder::default()
            .name("test_class")?
            .property(StyleProperty::StrokeWidth(Unit::Pixel(1)))?;
        let style_class = builder.style_class(style_class_builder)?;

        assert_eq!(style_class.name(), "test_class");

        // Test Builder methods
        assert!(builder.get_style_class_by_name("test_class").is_some());
        assert!(builder.get_style_class_by_name("unknown").is_none());

        // Test duplicate class
        let style_class_builder_dup = StyleClassBuilder::default()
            .name("test_class")?
            .property(StyleProperty::StrokeWidth(Unit::Pixel(1)))?;
        let result = builder.style_class(style_class_builder_dup);
        assert!(result.is_err());

        let diagram: GenericDiagram<FlowchartNode, FlowchartEdge, FlowchartConfiguration> =
            builder.into();
        assert!(diagram.get_style_class_by_name("test_class").is_some());
        assert!(diagram.get_style_class_by_name("unknown").is_none());
        assert_eq!(diagram.style_classes().count(), 1);

        Ok(())
    }

    #[test]
    fn test_node_with_unknown_class() -> Result<(), Box<dyn std::error::Error>> {
        let mut builder =
            GenericDiagramBuilder::<FlowchartNode, FlowchartEdge, FlowchartConfiguration>::default(
            );

        let class = StyleClassBuilder::default()
            .name("unknown")?
            .property(StyleProperty::StrokeWidth(Unit::Pixel(2)))?
            .build()?;

        let node_builder =
            FlowchartNodeBuilder::default().label("Node 1")?.id(1).style_class(Rc::new(class))?;

        let result = builder.node(node_builder);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_edge_with_unknown_nodes() -> Result<(), Box<dyn std::error::Error>> {
        let mut builder =
            GenericDiagramBuilder::<FlowchartNode, FlowchartEdge, FlowchartConfiguration>::default(
            );

        let node1 = Rc::new(FlowchartNodeBuilder::default().label("Node 1")?.id(1).build()?);
        let node2 = Rc::new(FlowchartNodeBuilder::default().label("Node 2")?.id(2).build()?);

        let edge_builder = FlowchartEdgeBuilder::default().source(node1)?.destination(node2)?.id(1);

        let result = builder.edge(edge_builder);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_edge_destination_node_not_found() -> Result<(), Box<dyn std::error::Error>> {
        let mut builder =
            GenericDiagramBuilder::<FlowchartNode, FlowchartEdge, FlowchartConfiguration>::default(
            );

        let node1_builder = FlowchartNodeBuilder::default().label("Node 1")?.id(1);
        let node1 = builder.node(node1_builder)?;

        let node2 = Rc::new(FlowchartNodeBuilder::default().label("Node 2")?.id(2).build()?);

        let edge_builder = FlowchartEdgeBuilder::default().source(node1)?.destination(node2)?.id(1);

        let result = builder.edge(edge_builder);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_node_auto_id() -> Result<(), Box<dyn std::error::Error>> {
        let mut builder =
            GenericDiagramBuilder::<FlowchartNode, FlowchartEdge, FlowchartConfiguration>::default(
            );

        let node_builder = FlowchartNodeBuilder::default().label("Node 1")?;
        let node = builder.node(node_builder)?;
        assert_eq!(node.id(), 0);

        let node_builder2 = FlowchartNodeBuilder::default().label("Node 2")?;
        let node2 = builder.node(node_builder2)?;
        assert_eq!(node2.id(), 1);

        Ok(())
    }
}
