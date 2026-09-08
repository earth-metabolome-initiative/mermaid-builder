//! Builder for state diagrams.

use super::{
    Configuration, Diagram, GenericNodeBuilder, Node, NodeBuilder, NodeError, Rc, StateDiagram,
    StateKind, StateNode, String, StyleClass, StyleProperty, ToString,
};
use crate::shared::generic_configuration::{Look, Renderer, Theme};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for a state node.
pub struct StateNodeBuilder {
    generic: GenericNodeBuilder,
    inner_diagram: Option<StateDiagram>,
    kind: StateKind,
}

impl NodeBuilder for StateNodeBuilder {
    type Node = StateNode;
    type Error = NodeError;
    fn build(mut self) -> Result<Self::Node, Self::Error> {
        if self.kind != StateKind::Normal && self.inner_diagram.is_some() {
            return Err(NodeError::InvalidStateKind);
        }
        if self.kind != StateKind::Normal && self.generic.get_label().is_none() {
            self.generic = self.generic.label("special state")?;
        }
        let node = self.generic.build()?;
        if matches!(self.kind, StateKind::Start | StateKind::End)
            && (node.has_styles() || node.classes().next().is_some())
        {
            return Err(NodeError::InvalidStateKind);
        }
        Ok(StateNode { node, inner_diagram: self.inner_diagram, kind: self.kind })
    }
    fn label<S: ToString>(mut self, label: S) -> Result<Self, Self::Error> {
        self.generic = self.generic.label(label)?;
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
    /// Returns an error if the nested diagram contains root-only configuration.
    pub fn inner_diagram(mut self, diagram: StateDiagram) -> Result<Self, NodeError> {
        let config = diagram.configuration();
        if config.title().is_some()
            || config.renderer() != Renderer::default()
            || config.theme() != Theme::default()
            || config.look() != Look::default()
        {
            return Err(NodeError::NestedStateConfiguration);
        }
        self.inner_diagram = Some(diagram);
        Ok(self)
    }
}
impl StateNodeBuilder {
    /// Sets the state kind. Special states do not require a label.
    #[must_use]
    pub fn kind(mut self, kind: StateKind) -> Self {
        self.kind = kind;
        self
    }
}
