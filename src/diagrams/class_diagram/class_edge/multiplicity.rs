//! Submodule defining the multiplicities which may be used to characterize
//! a class edge in a class diagram in Mermaid syntax.
//!
//! The different cardinality options are :
//!
//! - `1` Only 1
//! - `0..1` Zero or One
//! - `1..*` One or more
//! - `*` Many
//! - `n` n (where n>1)
//! - `0..n` zero to n (where n>1)
//! - `1..n` one to n (where n>1)

use core::fmt::{self, Display};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// An enumeration representing the multiplicity of a class edge in a Mermaid
/// class diagram.
pub enum Multiplicity {
    /// Only 1
    One,
    /// Zero or One
    ZeroOrOne,
    /// One or more
    OneOrMore,
    /// Many, which is analogous to `Zero or More`
    Many,
    /// n (where n>1)
    N,
    /// Zero to n (where n>1)
    ZeroToN,
    /// One to n (where n>1)
    OneToN,
}

impl Display for Multiplicity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Multiplicity::One => write!(f, "1"),
            Multiplicity::ZeroOrOne => write!(f, "0..1"),
            Multiplicity::OneOrMore => write!(f, "1..*"),
            Multiplicity::Many => write!(f, "*"),
            Multiplicity::N => write!(f, "n"),
            Multiplicity::ZeroToN => write!(f, "0..n"),
            Multiplicity::OneToN => write!(f, "1..n"),
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_multiplicity_display() {
        assert_eq!(Multiplicity::One.to_string(), "1");
        assert_eq!(Multiplicity::ZeroOrOne.to_string(), "0..1");
        assert_eq!(Multiplicity::OneOrMore.to_string(), "1..*");
        assert_eq!(Multiplicity::Many.to_string(), "*");
        assert_eq!(Multiplicity::N.to_string(), "n");
        assert_eq!(Multiplicity::ZeroToN.to_string(), "0..n");
        assert_eq!(Multiplicity::OneToN.to_string(), "1..n");
    }
}
