//! Submodule defining whether a flowchart is meant to extend in a horizontal or
//! vertical direction.

use core::fmt::Display;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents the direction of a flowchart in Mermaid diagrams.
pub enum Direction {
    /// The flowchart extends horizontally.
    #[default]
    LeftToRight,
    /// The flowchart extends vertically.
    TopToBottom,
    /// The flowchart extends in a right-to-left direction.
    RightToLeft,
    /// The flowchart extends in a bottom-to-top direction.
    BottomToTop,
}

impl Direction {
    #[must_use]
    /// Changes the orientation from vertical to horizontal or vice versa.
    pub fn flip(self) -> Self {
        match self {
            Self::LeftToRight => Self::TopToBottom,
            Self::TopToBottom => Self::LeftToRight,
            Self::RightToLeft => Self::BottomToTop,
            Self::BottomToTop => Self::RightToLeft,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::LeftToRight => "LR",
                Self::TopToBottom => "TB",
                Self::RightToLeft => "RL",
                Self::BottomToTop => "BT",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;

    #[test]
    fn test_direction_display() {
        assert_eq!(format!("{}", Direction::LeftToRight), "LR");
        assert_eq!(format!("{}", Direction::TopToBottom), "TB");
        assert_eq!(format!("{}", Direction::RightToLeft), "RL");
        assert_eq!(format!("{}", Direction::BottomToTop), "BT");
    }

    #[test]
    fn test_direction_flip() {
        assert_eq!(Direction::LeftToRight.flip(), Direction::TopToBottom);
        assert_eq!(Direction::TopToBottom.flip(), Direction::LeftToRight);
        assert_eq!(Direction::RightToLeft.flip(), Direction::BottomToTop);
        assert_eq!(Direction::BottomToTop.flip(), Direction::RightToLeft);
    }
}
