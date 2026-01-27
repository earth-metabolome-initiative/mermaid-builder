#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

pub mod diagrams;
mod errors;
mod shared;
pub mod traits;
pub use errors::{ConfigError, EdgeError, Error, NodeError, StyleClassError};

/// Submodule providing common traits and types for Mermaid diagrams.
pub mod prelude {
    pub use crate::{
        diagrams::{class_diagram::*, entity_relationship::*, flowchart::*},
        shared::{
            ArrowShape, Color, Direction, FontWeight, LineStyle, Renderer, StyleClass,
            StyleClassBuilder, StyleProperty, Unit,
        },
        traits::*,
    };
}
