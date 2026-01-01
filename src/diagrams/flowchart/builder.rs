//! Submodule providing a builder for flowchart diagrams in Mermaid syntax.

use crate::{
    prelude::{
        Flowchart, FlowchartConfiguration, FlowchartConfigurationBuilder, FlowchartEdge,
        FlowchartEdgeBuilder, FlowchartNode, FlowchartNodeBuilder,
    },
    shared::{StyleClass, StyleClassBuilder, generic_diagram::GenericDiagramBuilder},
    traits::DiagramBuilder,
};

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a builder for a flowchart diagram in Mermaid syntax.
pub struct FlowchartBuilder {
    /// The configuration of the flowchart.
    generic: GenericDiagramBuilder<FlowchartNode, FlowchartEdge, FlowchartConfiguration>,
}

impl From<FlowchartBuilder> for Flowchart {
    fn from(builder: FlowchartBuilder) -> Self {
        let generic = builder.generic.into();
        Flowchart { generic }
    }
}

impl DiagramBuilder for FlowchartBuilder {
    type Configuration = FlowchartConfiguration;
    type ConfigurationBuilder = FlowchartConfigurationBuilder;
    type Diagram = Flowchart;
    type Edge = FlowchartEdge;
    type EdgeBuilder = FlowchartEdgeBuilder;
    type Node = FlowchartNode;
    type NodeBuilder = FlowchartNodeBuilder;
    type Error = crate::errors::Error;

    fn configuration(
        mut self,
        configuration: Self::ConfigurationBuilder,
    ) -> Result<Self, Self::Error> {
        self.generic = self.generic.configuration(configuration)?;
        Ok(self)
    }

    fn edge(
        &mut self,
        mut edge: Self::EdgeBuilder,
    ) -> Result<std::rc::Rc<Self::Edge>, Self::Error> {
        edge = edge.id(self.number_of_edges());
        self.generic.edge(edge)
    }

    fn get_node_by_id(&self, id: u64) -> Option<std::rc::Rc<Self::Node>> {
        self.generic.get_node_by_id(id)
    }

    fn node(&mut self, node: Self::NodeBuilder) -> Result<std::rc::Rc<Self::Node>, Self::Error> {
        self.generic.node(node)
    }

    fn nodes(&self) -> impl Iterator<Item = &std::rc::Rc<Self::Node>> + '_ {
        self.generic.nodes()
    }

    fn number_of_edges(&self) -> usize {
        self.generic.number_of_edges()
    }

    fn number_of_nodes(&self) -> usize {
        self.generic.number_of_nodes()
    }

    fn style_class(
        &mut self,
        style_class: StyleClassBuilder,
    ) -> Result<std::rc::Rc<StyleClass>, Self::Error> {
        self.generic.style_class(style_class)
    }

    fn get_style_class_by_name(&self, name: &str) -> Option<std::rc::Rc<StyleClass>> {
        self.generic.get_style_class_by_name(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        prelude::{FlowchartConfigurationBuilder, FlowchartEdgeBuilder, FlowchartNodeBuilder},
        shared::{StyleClassBuilder, StyleProperty, style_class::Color},
        traits::{
            ConfigurationBuilder, DiagramBuilder, EdgeBuilder, NodeBuilder, edge::Edge, node::Node,
        },
    };

    #[test]
    fn test_flowchart_builder() -> Result<(), Box<dyn std::error::Error>> {
        let mut builder = FlowchartBuilder::default();

        // Test configuration
        let config = FlowchartConfigurationBuilder::default().title("My Flowchart")?;
        builder = builder.configuration(config)?;

        // Test style class
        let style_class_builder = StyleClassBuilder::default()
            .name("myStyle")?
            .property(StyleProperty::Fill(Color::from((255, 0, 0))))?;
        let style_class = builder.style_class(style_class_builder)?;
        assert_eq!(style_class.name(), "myStyle");
        assert!(builder.get_style_class_by_name("myStyle").is_some());

        // Test node
        let node_builder = FlowchartNodeBuilder::default().label("Node A")?;
        let node_a = builder.node(node_builder)?;
        assert_eq!(builder.number_of_nodes(), 1);
        assert!(builder.get_node_by_id(node_a.id()).is_some());

        let node_builder_b = FlowchartNodeBuilder::default().label("Node B")?;
        let node_b = builder.node(node_builder_b)?;
        assert_eq!(builder.number_of_nodes(), 2);

        // Test nodes iterator
        let nodes: Vec<_> = builder.nodes().collect();
        assert_eq!(nodes.len(), 2);

        // Test edge
        let edge_builder =
            FlowchartEdgeBuilder::default().source(node_a.clone())?.destination(node_b.clone())?;
        let edge = builder.edge(edge_builder)?;
        assert_eq!(builder.number_of_edges(), 1);
        assert_eq!(edge.source().id(), node_a.id());
        assert_eq!(edge.destination().id(), node_b.id());

        // Test build (into Flowchart)
        let _flowchart: Flowchart = builder.into();
        // We can't easily inspect the flowchart internals here without more accessors,
        // but the conversion should succeed.

        Ok(())
    }
}
