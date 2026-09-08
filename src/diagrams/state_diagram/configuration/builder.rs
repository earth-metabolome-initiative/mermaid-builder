//! Builder for state diagrams.

use super::{
    ConfigError, ConfigurationBuilder, Direction, GenericConfigurationBuilder, Look, Renderer,
    StateDiagramConfiguration, Theme, ToString,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for state diagram configuration.
pub struct StateDiagramConfigurationBuilder {
    generic: GenericConfigurationBuilder,
}

impl ConfigurationBuilder for StateDiagramConfigurationBuilder {
    type Configuration = StateDiagramConfiguration;
    type Error = ConfigError;
    fn build(self) -> Result<Self::Configuration, Self::Error> {
        Ok(StateDiagramConfiguration { generic: self.generic.build()? })
    }
    fn title<S: ToString>(mut self, title: S) -> Result<Self, Self::Error> {
        self.generic = self.generic.title(title)?;
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
impl StateDiagramConfigurationBuilder {
    /// Sets the diagram theme.
    #[must_use]
    pub fn theme(mut self, theme: Theme) -> Self {
        self.generic = self.generic.theme(theme);
        self
    }
    /// Sets the diagram look.
    #[must_use]
    pub fn look(mut self, look: Look) -> Self {
        self.generic = self.generic.look(look);
        self
    }
}
