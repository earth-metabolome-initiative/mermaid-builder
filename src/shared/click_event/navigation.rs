//! Submodule handling navigation click events in Mermaid diagrams, which
//! can include anchor-like links or JavaScript function calls navigating to
//! external resources or internal sections, with or without opening in a new
//! tab.

use alloc::string::String;
use core::fmt::Display;

/// Represents a navigation event triggered by a click on a node in a Mermaid
/// diagram. This can include external links, with options for opening in a new
/// tab and whether to use an anchor-like link or a JavaScript function for
/// navigation.
///
/// # Example
///
/// Some example of mermaid syntax for a navigation event:
///
/// ```mermaid
/// click A "https://www.github.com" _blank
/// click B "https://www.github.com" "Open this in a new tab" _blank
/// click C href "https://www.github.com" _blank
/// click D href "https://www.github.com" "Open this in a new tab" _blank
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Navigation {
    /// The URL to navigate to when the node is clicked.
    url: String,
    /// Whether to open the link in a new tab.
    new_tab: bool,
    /// Whether to employ an anchor-like link or a JavaScript function for
    /// navigation.
    anchor: bool,
    /// Descriptive tooltip for the navigation link.
    tooltip: Option<String>,
}

impl Navigation {
    /// Creates a new navigation event.
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into(), new_tab: false, anchor: false, tooltip: None }
    }

    /// Sets whether to open the link in a new tab.
    pub fn new_tab(mut self, new_tab: bool) -> Self {
        self.new_tab = new_tab;
        self
    }

    /// Sets whether to employ an anchor-like link or a JavaScript function for
    /// navigation.
    pub fn anchor(mut self, anchor: bool) -> Self {
        self.anchor = anchor;
        self
    }

    /// Sets the tooltip for the navigation link.
    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }
}

impl Display for Navigation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // We omit the `click {node_name}` part as it is not relevant for the
        // display of the navigation event, and is handled by the parent
        // `ClickEvent` enum.
        if self.anchor {
            write!(f, "href")?;
        }
        write!(f, " \"{}\"", self.url)?;

        if let Some(tooltip) = &self.tooltip {
            write!(f, " \"{tooltip}\"")?;
        }

        if self.new_tab {
            write!(f, " _blank")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use alloc::{format, string::ToString};

    use super::*;

    #[test]
    fn test_navigation_display() {
        let nav = Navigation {
            url: "https://example.com".to_string(),
            new_tab: false,
            anchor: false,
            tooltip: None,
        };
        assert_eq!(format!("{nav}"), " \"https://example.com\"");

        let nav = Navigation {
            url: "https://example.com".to_string(),
            new_tab: true,
            anchor: false,
            tooltip: None,
        };
        assert_eq!(format!("{nav}"), " \"https://example.com\" _blank");

        let nav = Navigation {
            url: "https://example.com".to_string(),
            new_tab: false,
            anchor: true,
            tooltip: None,
        };
        assert_eq!(format!("{nav}"), "href \"https://example.com\"");

        let nav = Navigation {
            url: "https://example.com".to_string(),
            new_tab: true,
            anchor: true,
            tooltip: Some("Tooltip".to_string()),
        };
        assert_eq!(format!("{nav}"), "href \"https://example.com\" \"Tooltip\" _blank");
    }
}
