//! Submodule defining a class method struct for the class diagram in
//! Mermaid syntax, including its visibility and parameters signatures.

use std::fmt::Display;

use crate::diagrams::class_diagram::visibility::Visibility;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a method argument in a Mermaid class diagram.
pub struct Argument {
    /// The name of the argument (e.g., `x`).
    name: String,
    /// The type of the argument (e.g., `int`, `String`).
    arg_type: String,
}

impl Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.arg_type)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a class method for Mermaid class diagrams, including its name,
/// arguments, and visibility.
pub struct ClassMethod {
    /// The name of the class method (e.g., `doSomething`).
    name: String,
    /// The list of arguments for the method, each with a name and type.
    arguments: Vec<Argument>,
    /// The return type of the method (e.g., `void`, `int`).
    return_type: Option<String>,
    /// The visibility of the method (e.g., public, private, protected).
    visibility: Visibility,
}

impl Display for ClassMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.visibility)?;
        write!(f, "{}", self.name)?;

        if self.arguments.is_empty() {
            write!(f, "()")?;
        } else {
            write!(f, "(")?;
            for (i, arg) in self.arguments.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{arg}")?;
            }
            write!(f, ")")?;
        }

        if let Some(return_type) = &self.return_type {
            write!(f, ": {return_type}")?;
        } else {
            write!(f, ": void")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_argument_display() {
        let arg = Argument { name: "arg1".to_string(), arg_type: "int".to_string() };
        assert_eq!(arg.to_string(), "arg1: int");
    }

    #[test]
    fn test_class_method_display() {
        let method = ClassMethod {
            name: "method1".to_string(),
            arguments: vec![
                Argument { name: "arg1".to_string(), arg_type: "int".to_string() },
                Argument { name: "arg2".to_string(), arg_type: "String".to_string() },
            ],
            return_type: Some("bool".to_string()),
            visibility: Visibility::Public,
        };
        assert_eq!(method.to_string(), "+method1(arg1: int, arg2: String): bool");

        let method_void = ClassMethod {
            name: "method2".to_string(),
            arguments: vec![],
            return_type: None,
            visibility: Visibility::Private,
        };
        assert_eq!(method_void.to_string(), "-method2(): void");
    }
}
