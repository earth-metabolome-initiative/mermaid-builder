//! The themes enumeration to use for rendering a Mermaid diagram.

use core::fmt::Display;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The themes enumeration to use for rendering a Mermaid diagram.
pub enum Theme {
    /// The classic Mermaid chart theme.
    MermaidChart,
    /// The `Neo` theme, a modern style for diagrams.
    Neo,
    /// The `NeoDark` theme, a dark variant of Neo.
    NeoDark,
    /// The default theme for Mermaid diagrams.
    #[default]
    Default,
    /// The `Forest` theme, with green accents and natural tones.
    Forest,
    /// The `Base` theme, a minimal and clean style.
    Base,
    /// The `Dark` theme, for diagrams with a dark background.
    Dark,
    /// The `Neutral` theme, with muted and balanced colors.
    Neutral,
    /// The `Redux` theme, a vibrant and bold style.
    Redux,
    /// The `ReduxDark` theme, a dark variant of Redux.
    ReduxDark,
}

impl Display for Theme {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Theme::MermaidChart => "mc",
                Theme::Neo => "neo",
                Theme::NeoDark => "neo-dark",
                Theme::Default => "default",
                Theme::Forest => "forest",
                Theme::Base => "base",
                Theme::Dark => "dark",
                Theme::Neutral => "neutral",
                Theme::Redux => "redux",
                Theme::ReduxDark => "redux-dark",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;

    #[test]
    fn test_theme_display() {
        assert_eq!(format!("{}", Theme::MermaidChart), "mc");
        assert_eq!(format!("{}", Theme::Neo), "neo");
        assert_eq!(format!("{}", Theme::NeoDark), "neo-dark");
        assert_eq!(format!("{}", Theme::Default), "default");
        assert_eq!(format!("{}", Theme::Forest), "forest");
        assert_eq!(format!("{}", Theme::Base), "base");
        assert_eq!(format!("{}", Theme::Dark), "dark");
        assert_eq!(format!("{}", Theme::Neutral), "neutral");
        assert_eq!(format!("{}", Theme::Redux), "redux");
        assert_eq!(format!("{}", Theme::ReduxDark), "redux-dark");
    }
}
