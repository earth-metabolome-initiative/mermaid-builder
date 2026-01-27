//! Submodule defining a class attribute struct for the class diagram in
//! Mermaid syntax, including its visibility and type.

use alloc::string::String;
use core::fmt::{self, Display};

use crate::diagrams::class_diagram::visibility::Visibility;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Struct representing a class attribute in a class diagram.
pub struct ClassAttribute {
    /// The name of the class attribute.
    name: String,
    /// The type of the class attribute.
    attribute_type: String,
    /// The visibility of the class attribute (e.g., public, private).
    visibility: Visibility,
}

impl ClassAttribute {
    /// Creates a new class attribute.
    pub fn new(attribute_type: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            attribute_type: attribute_type.into(),
            visibility: Visibility::Public,
        }
    }
}

impl Display for ClassAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}: {}", self.visibility, self.name, self.attribute_type)
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_class_attribute_display() {
        let attr = ClassAttribute {
            name: "attr1".to_string(),
            attribute_type: "int".to_string(),
            visibility: Visibility::Public,
        };
        assert_eq!(attr.to_string(), "+ attr1: int");
    }
}
