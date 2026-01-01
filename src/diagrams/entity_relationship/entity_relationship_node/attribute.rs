//! Submodule defining an attribute of an Entity-Relationship (ER) node
//! for the entity-relationship diagram in Mermaid syntax.

use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Struct representing an attribute of an entity-relationship node.
pub struct EntityRelationshipAttribute {
    /// The name of the class attribute.
    name: String,
    /// The type of the class attribute.
    attribute_type: String,
}

impl EntityRelationshipAttribute {
    /// Creates a new entity-relationship attribute.
    #[must_use]
    pub fn new(attribute_type: String, name: String) -> Self {
        Self { name, attribute_type }
    }

    /// Returns the name of the attribute.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mermaid_builder::diagrams::entity_relationship::EntityRelationshipAttribute;
    ///
    /// let attribute = EntityRelationshipAttribute::new("string".to_string(), "username".to_string());
    /// assert_eq!(attribute.name(), "username");
    /// ```
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the type of the attribute.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mermaid_builder::diagrams::entity_relationship::EntityRelationshipAttribute;
    ///
    /// let attribute = EntityRelationshipAttribute::new("string".to_string(), "username".to_string());
    /// assert_eq!(attribute.attribute_type(), "string");
    /// ```
    #[must_use]
    pub fn attribute_type(&self) -> &str {
        &self.attribute_type
    }
}

impl Display for EntityRelationshipAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.attribute_type)
    }
}
