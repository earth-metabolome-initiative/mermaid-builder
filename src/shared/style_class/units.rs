//! Submodule providing an enumeration of units which may be used in
//! style class definitions in Mermaid diagrams, including pixel and
//! point units.

use core::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents the unit of measurement used in style class definitions.
pub enum Unit {
    /// Pixel unit, denoted by `px`.
    Pixel(u8),
    /// Point unit, denoted by `pt`.
    Point(u8),
}

impl Display for Unit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Unit::Pixel(value) => write!(f, "{value}px"),
            Unit::Point(value) => write!(f, "{value}pt"),
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;

    #[test]
    fn test_unit_display() {
        assert_eq!(format!("{}", Unit::Pixel(10)), "10px");
        assert_eq!(format!("{}", Unit::Point(12)), "12pt");
    }
}
