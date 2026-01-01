//! Submodule defining the possible arrow shapes for links in Mermaid diagrams.

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents the shape of an arrow that can be used in Mermaid diagrams.
pub enum ArrowShape {
    /// Arrow shape with a normal arrowhead.
    #[default]
    Normal,
    /// A sharp arrowhead shape.
    Sharp,
    /// X shape arrowhead.
    X,
    /// Circle shape arrowhead.
    Circle,
    /// Triangle shape arrowhead.
    Triangle,
    /// Star shape arrowhead.
    Star,
    /// Shape representing zero or one dependency.
    ZeroOrOne,
    /// Shape representing exactly one dependency.
    ExactlyOne,
    /// Shape representing zero or more dependencies.
    ZeroOrMore,
    /// Shape representing one or more dependencies.
    OneOrMore,
}

impl ArrowShape {
    #[must_use]
    /// Returns the left-oriented arrow shape.
    pub fn left(&self) -> &str {
        match self {
            ArrowShape::Normal => "<",
            ArrowShape::Sharp => "(",
            ArrowShape::X => "x",
            ArrowShape::Circle => "o",
            ArrowShape::Triangle => "<|",
            ArrowShape::Star => "*",
            ArrowShape::ZeroOrOne => "|o",
            ArrowShape::ExactlyOne => "||",
            ArrowShape::ZeroOrMore => "}o",
            ArrowShape::OneOrMore => "}|",
        }
    }

    #[must_use]
    /// Returns the right-oriented arrow shape.
    pub fn right(&self) -> &str {
        match self {
            ArrowShape::Normal => ">",
            ArrowShape::Sharp => ")",
            ArrowShape::X => "x",
            ArrowShape::Circle => "o",
            ArrowShape::Triangle => "|>",
            ArrowShape::Star => "*",
            ArrowShape::ZeroOrOne => "o|",
            ArrowShape::ExactlyOne => "||",
            ArrowShape::ZeroOrMore => "o{",
            ArrowShape::OneOrMore => "|{",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrow_shape_left() {
        assert_eq!(ArrowShape::Normal.left(), "<");
        assert_eq!(ArrowShape::Sharp.left(), "(");
        assert_eq!(ArrowShape::X.left(), "x");
        assert_eq!(ArrowShape::Circle.left(), "o");
        assert_eq!(ArrowShape::Triangle.left(), "<|");
        assert_eq!(ArrowShape::Star.left(), "*");
        assert_eq!(ArrowShape::ZeroOrOne.left(), "|o");
        assert_eq!(ArrowShape::ExactlyOne.left(), "||");
        assert_eq!(ArrowShape::ZeroOrMore.left(), "}o");
        assert_eq!(ArrowShape::OneOrMore.left(), "}|");
    }

    #[test]
    fn test_arrow_shape_right() {
        assert_eq!(ArrowShape::Normal.right(), ">");
        assert_eq!(ArrowShape::Sharp.right(), ")");
        assert_eq!(ArrowShape::X.right(), "x");
        assert_eq!(ArrowShape::Circle.right(), "o");
        assert_eq!(ArrowShape::Triangle.right(), "|>");
        assert_eq!(ArrowShape::Star.right(), "*");
        assert_eq!(ArrowShape::ZeroOrOne.right(), "o|");
        assert_eq!(ArrowShape::ExactlyOne.right(), "||");
        assert_eq!(ArrowShape::ZeroOrMore.right(), "o{");
        assert_eq!(ArrowShape::OneOrMore.right(), "|{");
    }
}
