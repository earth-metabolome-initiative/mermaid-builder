//! Streaming escape adapter for state descriptions and transition labels.
use core::fmt::{self, Display};
pub(super) struct Escaped<'a>(pub &'a str);
impl Display for Escaped<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.0.chars() {
            match c {
                ':' => write!(f, "#colon;")?,
                ';' => write!(f, "#59;")?,
                '#' => write!(f, "#35;")?,
                '"' => write!(f, "#quot;")?,
                '&' => write!(f, "#38;")?,
                '<' => write!(f, "#60;")?,
                '>' => write!(f, "#62;")?,
                '%' => write!(f, "#37;")?,
                c if c.is_whitespace() => write!(f, " ")?,
                c => write!(f, "{c}")?,
            }
        }
        Ok(())
    }
}
