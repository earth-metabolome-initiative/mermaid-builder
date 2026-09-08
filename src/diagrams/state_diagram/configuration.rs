//! State diagram configuration.

mod builder;
use alloc::string::ToString;
use core::fmt::{self, Display};

pub use builder::StateDiagramConfigurationBuilder;

pub use crate::shared::generic_configuration::{Look, Theme};
use crate::{
    errors::ConfigError,
    shared::generic_configuration::{
        Direction, GenericConfiguration, GenericConfigurationBuilder, Renderer,
    },
    traits::{Configuration, ConfigurationBuilder},
};
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
        if self.title().is_none()
            && self.renderer() == Renderer::default()
            && self.theme() == Theme::default()
            && self.look() == Look::default()
        {
            return Ok(());
        }
        Display::fmt(&self.generic, f)
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;
    use crate::{
        NodeError,
        diagrams::state_diagram::{StateDiagramBuilder, StateNodeBuilder},
        traits::DiagramBuilder,
    };

    #[test]
    fn theme_and_look_are_rendered_and_rejected_when_nested() -> Result<(), crate::Error> {
        for config in [
            StateDiagramConfigurationBuilder::default().theme(Theme::Forest),
            StateDiagramConfigurationBuilder::default().look(Look::HandDrawn),
        ] {
            let diagram = StateDiagramBuilder::default().configuration(config)?;
            assert!(matches!(
                StateNodeBuilder::default().inner_diagram(diagram.into()),
                Err(NodeError::NestedStateConfiguration)
            ));
        }
        let config = StateDiagramConfigurationBuilder::default()
            .theme(Theme::Forest)
            .look(Look::HandDrawn)
            .build()?;
        assert_eq!(
            config.to_string(),
            "---\nconfig:\n  layout: dagre\n  theme: forest\n  look: handDrawn\n---\n"
        );
        Ok(())
    }
}
