//! The looks enumeration to use for rendering a Mermaid diagram.

use core::fmt::Display;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The looks enumeration to use for rendering a Mermaid diagram.
pub enum Look {
    /// The Neo look, a modern style for diagrams.
    Neo,
    /// The hand-drawn look, a sketch-like style for diagrams.
    HandDrawn,
    #[default]
    /// The Classic look, the traditional Mermaid style.
    Classic,
}

impl Display for Look {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Look::Neo => "neo",
                Look::HandDrawn => "handDrawn",
                Look::Classic => "classic",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;

    #[test]
    fn test_look_display() {
        assert_eq!(format!("{}", Look::Neo), "neo");
        assert_eq!(format!("{}", Look::HandDrawn), "handDrawn");
        assert_eq!(format!("{}", Look::Classic), "classic");
    }
}
