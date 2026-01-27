//! Submodule providing an enumeration for visibility modifiers employable
//! in class entries of Mermaid class diagrams.
//!
//! These include: Public (`+`), Private (`-`), Protected (`#`), and
//! Package/Internal (`~`).

use core::fmt::{self, Display};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// An enumeration representing the visibility of class members in Mermaid class
/// diagrams.
pub enum Visibility {
    /// Public visibility, denoted by `+`.
    Public,
    /// Private visibility, denoted by `-`.
    Private,
    /// Protected visibility, denoted by `#`.
    Protected,
    /// Package/Internal visibility, denoted by `~`.
    Package,
}

impl Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Visibility::Public => write!(f, "+"),
            Visibility::Private => write!(f, "-"),
            Visibility::Protected => write!(f, "#"),
            Visibility::Package => write!(f, "~"),
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_visibility_display() {
        assert_eq!(Visibility::Public.to_string(), "+");
        assert_eq!(Visibility::Private.to_string(), "-");
        assert_eq!(Visibility::Protected.to_string(), "#");
        assert_eq!(Visibility::Package.to_string(), "~");
    }
}
