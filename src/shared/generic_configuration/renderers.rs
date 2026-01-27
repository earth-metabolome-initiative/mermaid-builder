//! Submodule defining the possible renderers which may be used in a flowchart
//! configuration in Mermaid.

use core::fmt::Display;

#[derive(Default, Copy, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents the renderer used for Mermaid diagrams.
pub enum Renderer {
    /// The dagre renderer, which is the default renderer for flowcharts.
    #[default]
    Dagre,
    /// The newer Eclipse Layout Kernel (ELK) renderer, which is an alternative
    /// to the dagre renderer.
    EclipseLayoutKernel,
}

impl Display for Renderer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Renderer::Dagre => write!(f, "dagre"),
            Renderer::EclipseLayoutKernel => write!(f, "elk"),
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;

    #[test]
    fn test_renderer_display() {
        assert_eq!(format!("{}", Renderer::Dagre), "dagre");
        assert_eq!(format!("{}", Renderer::EclipseLayoutKernel), "elk");
    }
}
