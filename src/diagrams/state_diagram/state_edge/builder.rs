//! Builder for state diagrams.

use super::{
    ArrowShape, Edge, EdgeError, GenericEdgeBuilder, LineStyle, Rc, StateEdge, StateKind,
    StateNode, ToString,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for a state edge.
pub struct StateEdgeBuilder {
    generic: GenericEdgeBuilder<StateNode>,
}

impl crate::traits::EdgeBuilder for StateEdgeBuilder {
    type Edge = StateEdge;
    type Node = StateNode;
    type Error = EdgeError;
    fn build(self) -> Result<Self::Edge, Self::Error> {
        let edge = self.generic.build()?;
        if edge.line_style() != LineStyle::Solid {
            return Err(EdgeError::UnsupportedStateLineStyle);
        }
        if edge.source().kind() == StateKind::End || edge.destination().kind() == StateKind::Start {
            return Err(EdgeError::InvalidStateTransition);
        }
        Ok(StateEdge { edge })
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
        self.generic = self.generic.label(l)?;
        Ok(self)
    }
    fn line_style(mut self, s: crate::shared::LineStyle) -> Self {
        self.generic = self.generic.line_style(s);
        self
    }
    fn left_arrow_shape(self, s: crate::shared::ArrowShape) -> Result<Self, Self::Error> {
        Err(EdgeError::IncompatibleLeftArrowShape(s))
    }
    fn right_arrow_shape(mut self, s: crate::shared::ArrowShape) -> Result<Self, Self::Error> {
        if s != ArrowShape::Normal {
            return Err(EdgeError::IncompatibleRightArrowShape(s));
        }
        self.generic = self.generic.right_arrow_shape(s)?;
        Ok(self)
    }
}
