//! Builder for state diagrams.

use super::{
    DiagramBuilder, GenericDiagram, GenericDiagramBuilder, NodeBuilder, Rc, StateDiagram,
    StateDiagramConfiguration, StateDiagramConfigurationBuilder, StateEdge, StateEdgeBuilder,
    StateNode, StateNodeBuilder, StyleClass,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    fn node(&mut self, mut n: Self::NodeBuilder) -> Result<Rc<Self::Node>, Self::Error> {
        if let Some(id) = n.get_id() {
            if self.get_node_by_id(id).is_some() {
                return Err(crate::NodeError::DuplicateNode(alloc::format!("{id}")).into());
            }
        } else {
            let mut id = 0;
            while self.get_node_by_id(id).is_some() {
                id += 1;
            }
            n = n.id(id);
        }
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
