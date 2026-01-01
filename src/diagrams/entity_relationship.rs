//! Submodule providing structs to characterize an ER (Entity-Relationship)
//! Diagram in Mermaid syntax.

pub mod entity_relationship_edge;
pub mod entity_relationship_node;
use std::fmt::Display;

use entity_relationship_edge::EREdge;
pub use entity_relationship_edge::EREdgeBuilder;
use entity_relationship_node::ERNode;
pub use entity_relationship_node::{ERNodeBuilder, attribute::EntityRelationshipAttribute};

use crate::{
    shared::{
        generic_configuration::{GenericConfiguration, GenericConfigurationBuilder},
        generic_diagram::{GenericDiagram, GenericDiagramBuilder},
    },
    traits::{configuration::Configuration, diagram::Diagram},
};

/// Represents the configuration for an entity-relationship diagram.
pub type ERDiagramConfiguration = GenericConfiguration;
/// Represents the configuration builder for an entity-relationship diagram.
pub type ERDiagramConfigurationBuilder = GenericConfigurationBuilder;
/// Represents an entity-relationship diagram in Mermaid syntax.
pub type ERDiagram = GenericDiagram<ERNode, EREdge, GenericConfiguration>;
/// Represents a builder for an entity-relationship diagram in Mermaid syntax.
pub type ERDiagramBuilder = GenericDiagramBuilder<ERNode, EREdge, GenericConfiguration>;

impl Display for ERDiagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::traits::TabbedDisplay;
        self.fmt_tabbed(f, 0)
    }
}

impl crate::traits::TabbedDisplay for ERDiagram {
    fn fmt_tabbed(&self, f: &mut std::fmt::Formatter<'_>, tab_count: usize) -> std::fmt::Result {
        let indent = " ".repeat(tab_count * 2);
        write!(f, "{}", self.configuration())?;
        writeln!(f, "{indent}erDiagram")?;
        writeln!(f, "{indent}  direction {}", self.configuration().direction())?;
        for style_class in self.style_classes() {
            style_class.fmt_tabbed(f, tab_count + 1)?;
        }
        for node in self.nodes() {
            node.fmt_tabbed(f, tab_count + 1)?;
        }
        for edge in self.edges() {
            edge.fmt_tabbed(f, tab_count + 1)?;
        }
        Ok(())
    }
}
