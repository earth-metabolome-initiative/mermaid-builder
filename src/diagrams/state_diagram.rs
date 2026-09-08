//! Types for Mermaid `stateDiagram-v2` diagrams.
mod builder;
mod configuration;
mod escape;
mod kind;
mod state_edge;
mod state_node;
use alloc::rc::Rc;
use core::fmt::{self, Display};

pub use builder::StateDiagramBuilder;
pub use configuration::{Look, StateDiagramConfiguration, StateDiagramConfigurationBuilder, Theme};
pub use kind::StateKind;
pub use state_edge::{StateEdge, StateEdgeBuilder};
pub use state_node::{StateNode, StateNodeBuilder};

use crate::{
    shared::{
        StyleClass,
        generic_diagram::{GenericDiagram, GenericDiagramBuilder},
    },
    traits::{Configuration, Diagram, DiagramBuilder, NodeBuilder, TabbedDisplay},
};
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
        Self::from(GenericDiagram::from(builder))
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
        write!(f, "{}", self.configuration())?;
        writeln!(f, "{:width$}stateDiagram-v2", "", width = tab_count * 4)?;
        self.fmt_body(f, tab_count + 1, "S")
    }
}

impl StateDiagram {
    fn fmt_body(&self, f: &mut fmt::Formatter<'_>, depth: usize, scope: &str) -> fmt::Result {
        let indent = "    ".repeat(depth);
        writeln!(f, "{indent}direction {}", self.configuration().direction())?;
        for class in self.style_classes() {
            write!(f, "{indent}classDef {scope}class_{} ", class.name())?;
            for (index, property) in class.properties().iter().enumerate() {
                if index > 0 {
                    write!(f, ",")?;
                }
                write!(f, "{property}")?;
            }
            writeln!(f)?;
        }
        for node in self.nodes() {
            node.fmt_scoped(f, depth, scope)?;
        }
        for edge in self.edges() {
            edge.fmt_scoped(f, depth, scope)?;
        }
        Ok(())
    }
}
