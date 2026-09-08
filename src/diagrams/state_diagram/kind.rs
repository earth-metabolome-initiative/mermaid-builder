//! Special state kinds.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The kind of state to render.
pub enum StateKind {
    /// An ordinary state, optionally containing a nested diagram.
    #[default]
    Normal,
    /// The initial pseudostate, rendered as `[*]` at a transition source.
    Start,
    /// The final pseudostate, rendered as `[*]` at a transition destination.
    End,
    /// A conditional branch.
    Choice,
    /// A fork into concurrent paths.
    Fork,
    /// A join of concurrent paths.
    Join,
}
