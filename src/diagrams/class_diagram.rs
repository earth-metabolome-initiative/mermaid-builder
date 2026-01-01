//! Submodule defining a class diagram in Mermaid syntax.

pub mod class_edge;
pub mod class_node;
mod configuration;
pub mod visibility;
use std::fmt::Display;

use class_edge::ClassEdge;
pub use class_edge::ClassEdgeBuilder;
use class_node::ClassNode;
pub use class_node::ClassNodeBuilder;
pub use configuration::ClassDiagramConfiguration;

use crate::{
    shared::generic_diagram::{GenericDiagram, GenericDiagramBuilder},
    traits::{configuration::Configuration, diagram::Diagram},
};

/// Represents a class diagram in Mermaid syntax.
pub type ClassDiagram = GenericDiagram<ClassNode, ClassEdge, ClassDiagramConfiguration>;
/// Represents a builder for a class diagram in Mermaid syntax.
pub type ClassDiagramBuilder =
    GenericDiagramBuilder<ClassNode, ClassEdge, ClassDiagramConfiguration>;

impl Display for ClassDiagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::traits::TabbedDisplay;
        self.fmt_tabbed(f, 0)
    }
}

impl crate::traits::TabbedDisplay for ClassDiagram {
    fn fmt_tabbed(&self, f: &mut std::fmt::Formatter<'_>, tab_count: usize) -> std::fmt::Result {
        let indent = " ".repeat(tab_count * 2);
        write!(f, "{}", self.configuration())?; // Configuration might need tabbed display too? Usually it's frontmatter or directives.
        writeln!(f, "{indent}classDiagram")?;
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
