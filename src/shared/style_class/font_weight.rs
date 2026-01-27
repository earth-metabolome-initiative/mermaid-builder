//! Submodule providing the `FontWeight` enum used in Mermaid diagrams.

use core::fmt::Display;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents different font weights that can be applied to text in Mermaid
/// diagrams.
pub enum FontWeight {
    /// Normal font weight.
    #[default]
    Normal,
    /// Bold font weight.
    Bold,
    /// Bolder font weight.
    Bolder,
    /// Lighter font weight.
    Lighter,
    /// A specific numeric font weight (e.g., 100, 200, ..., 900).
    Number(u16),
}

impl Display for FontWeight {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FontWeight::Normal => write!(f, "normal"),
            FontWeight::Bold => write!(f, "bold"),
            FontWeight::Bolder => write!(f, "bolder"),
            FontWeight::Lighter => write!(f, "lighter"),
            FontWeight::Number(value) => write!(f, "{value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;

    #[test]
    fn test_font_weight_display() {
        assert_eq!(format!("{}", FontWeight::Normal), "normal");
        assert_eq!(format!("{}", FontWeight::Bold), "bold");
        assert_eq!(format!("{}", FontWeight::Bolder), "bolder");
        assert_eq!(format!("{}", FontWeight::Lighter), "lighter");
        assert_eq!(format!("{}", FontWeight::Number(400)), "400");
    }
}
