//! Module defining the `TabbedDisplay` trait for formatted output with
//! indentation.

/// Trait for displaying objects with indentation.
pub trait TabbedDisplay {
    /// Formats the object with the given indentation level.
    ///
    /// # Errors
    ///
    /// Returns an error if formatting fails.
    fn fmt_tabbed(&self, f: &mut core::fmt::Formatter<'_>, tab_count: usize) -> core::fmt::Result;
}
