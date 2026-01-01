//! Submodule providing the enumeration `FontStyle` which defines
//! different font styles that can be applied to text in Mermaid diagrams.

use std::fmt::Display;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents different font styles that can be applied to text in Mermaid
/// diagrams.
pub enum FontStyle {
    /// Normal font style.
    #[default]
    Normal,
    /// Italic font style.
    Italic,
    /// Oblique font style.
    Oblique,
}

impl Display for FontStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontStyle::Normal => write!(f, "normal"),
            FontStyle::Italic => write!(f, "italic"),
            FontStyle::Oblique => write!(f, "oblique"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_style_display() {
        assert_eq!(format!("{}", FontStyle::Normal), "normal");
        assert_eq!(format!("{}", FontStyle::Italic), "italic");
        assert_eq!(format!("{}", FontStyle::Oblique), "oblique");
    }
}
