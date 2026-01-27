//! Submodule providing an enumeration of possible curve styles for flowchart
//! edges in Mermaid diagrams.

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents the curve styles available for flowchart edges in Mermaid syntax.
pub enum CurveStyle {
    /// Basis curve style.
    #[default]
    Basis,
    /// `BumpX` curve style.
    BumpX,
    /// `BumpY` curve style.
    BumpY,
    /// `Cardinal` curve style.
    Cardinal,
    /// `CatmullRom` curve style.
    CatmullRom,
    /// `Linear` curve style.
    Linear,
    /// `MonotoneX` curve style.
    MonotoneX,
    /// `MonotoneY` curve style.
    MonotoneY,
    /// `Natural` curve style.
    Natural,
    /// `Step` curve style.
    Step,
    /// `StepAfter` curve style.
    StepAfter,
    /// `StepBefore` curve style.
    StepBefore,
}

impl core::fmt::Display for CurveStyle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CurveStyle::Basis => write!(f, "basis"),
            CurveStyle::BumpX => write!(f, "bumpX"),
            CurveStyle::BumpY => write!(f, "bumpY"),
            CurveStyle::Cardinal => write!(f, "cardinal"),
            CurveStyle::CatmullRom => write!(f, "catmullRom"),
            CurveStyle::Linear => write!(f, "linear"),
            CurveStyle::MonotoneX => write!(f, "monotoneX"),
            CurveStyle::MonotoneY => write!(f, "monotoneY"),
            CurveStyle::Natural => write!(f, "natural"),
            CurveStyle::Step => write!(f, "step"),
            CurveStyle::StepAfter => write!(f, "stepAfter"),
            CurveStyle::StepBefore => write!(f, "stepBefore"),
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_curve_style_display() {
        assert_eq!(CurveStyle::Basis.to_string(), "basis");
        assert_eq!(CurveStyle::BumpX.to_string(), "bumpX");
        assert_eq!(CurveStyle::BumpY.to_string(), "bumpY");
        assert_eq!(CurveStyle::Cardinal.to_string(), "cardinal");
        assert_eq!(CurveStyle::CatmullRom.to_string(), "catmullRom");
        assert_eq!(CurveStyle::Linear.to_string(), "linear");
        assert_eq!(CurveStyle::MonotoneX.to_string(), "monotoneX");
        assert_eq!(CurveStyle::MonotoneY.to_string(), "monotoneY");
        assert_eq!(CurveStyle::Natural.to_string(), "natural");
        assert_eq!(CurveStyle::Step.to_string(), "step");
        assert_eq!(CurveStyle::StepAfter.to_string(), "stepAfter");
        assert_eq!(CurveStyle::StepBefore.to_string(), "stepBefore");
    }
}
