//! Submodule defining the possible shapes for nodes in Mermaid diagrams.
use std::{fmt::Display, str::FromStr};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `FlowchartNodeShape` represents all supported node shapes for Mermaid
/// diagrams.
pub enum FlowchartNodeShape {
    /// Standard process shape
    #[default]
    Rectangle,
    /// Represents an event
    RoundEdges,
    /// Terminal point
    StadiumShape,
    /// Subprocess
    Subprocess,
    /// Database storage
    Cylinder,
    /// Starting point
    Circle,
    /// Odd shape
    Odd,
    /// Decision-making step
    Diamond,
    /// Preparation or condition step
    Hexagon,
    /// Represents input or output (Lean right parallelogram)
    LRParallelogram,
    /// Represents output or input (Lean left parallelogram)
    LLParallelogram,
    /// Priority action (Base bottom trapezoid)
    Trapezoid,
    /// Manual task (Base top trapezoid)
    ReverseTrapezoid,
    /// Represents a stop point
    DoubleCircle,
    /// Represents a card
    NotchedRectangle,
    /// Lined/Shaded process shape
    Linedrectangle,
    /// Small starting point
    SmallCircle,
    /// Stop point
    FramedCircle,
    /// Fork or join in process flow
    LongRectangle,
    /// Represents a collate operation
    Hourglass,
    /// Adds a comment (Left curly brace)
    LeftCurlyBrace,
    /// Adds a comment (Right curly brace)
    RightCurlyBrace,
    /// Adds a comment (Braces on both sides)
    CurlyBraces,
    /// Communication link
    LightningBolt,
    /// Represents a document
    Document,
    /// Represents a delay
    HalfRoundedRectangle,
    /// Direct access storage
    HorizontalCylinder,
    /// Disk storage
    LinedCylinder,
    /// Represents a display
    CurvedTrapezoid,
    /// Divided process shape
    DividedRectangle,
    /// Extraction process
    SmallTriangle,
    /// Internal storage
    WindowPane,
    /// Junction point
    FilledCircle,
    /// Lined document
    LinedDocument,
    /// Loop limit step
    NotchedPentagon,
    /// Manual file operation
    FlippedTriangle,
    /// Manual input step
    SlopedRectangle,
    /// Multiple documents
    StackedDocument,
    /// Multiple processes
    StackedRectangle,
    /// Paper tape
    Flag,
    /// Stored data
    BowTieRectangle,
    /// Summary
    CrossedCircle,
    /// Tagged document
    TaggedDocument,
    /// Tagged process
    TaggedRectangle,
    /// Subprocess (framed rectangle)
    FramedRectangle,
    /// Text block
    TextBlock,
}

impl Display for FlowchartNodeShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rectangle => write!(f, "rect"),
            Self::RoundEdges => write!(f, "rounded"),
            Self::StadiumShape => write!(f, "stadium"),
            Self::Subprocess => write!(f, "subproc"),
            Self::Cylinder => write!(f, "cyl"),
            Self::Circle => write!(f, "circle"),
            Self::Odd => write!(f, "odd"),
            Self::Diamond => write!(f, "diamond"),
            Self::Hexagon => write!(f, "hex"),
            Self::LRParallelogram => write!(f, "lean-r"),
            Self::LLParallelogram => write!(f, "lean-l"),
            Self::Trapezoid => write!(f, "trap-b"),
            Self::ReverseTrapezoid => write!(f, "trap-t"),
            Self::DoubleCircle => write!(f, "dbl-circ"),
            Self::NotchedRectangle => write!(f, "notch-rect"),
            Self::Linedrectangle => write!(f, "lin-rect"),
            Self::SmallCircle => write!(f, "sm-circ"),
            Self::FramedCircle => write!(f, "framed-circle"),
            Self::LongRectangle => write!(f, "fork"),
            Self::Hourglass => write!(f, "hourglass"),
            Self::LeftCurlyBrace => write!(f, "comment"),
            Self::RightCurlyBrace => write!(f, "brace-r"),
            Self::CurlyBraces => write!(f, "braces"),
            Self::LightningBolt => write!(f, "bolt"),
            Self::Document => write!(f, "doc"),
            Self::HalfRoundedRectangle => write!(f, "delay"),
            Self::HorizontalCylinder => write!(f, "das"),
            Self::LinedCylinder => write!(f, "lin-cyl"),
            Self::CurvedTrapezoid => write!(f, "curv-trap"),
            Self::DividedRectangle => write!(f, "div-rect"),
            Self::SmallTriangle => write!(f, "tri"),
            Self::WindowPane => write!(f, "win-pane"),
            Self::FilledCircle => write!(f, "f-circ"),
            Self::LinedDocument => write!(f, "lin-doc"),
            Self::NotchedPentagon => write!(f, "notch-pent"),
            Self::FlippedTriangle => write!(f, "flip-tri"),
            Self::SlopedRectangle => write!(f, "sl-rect"),
            Self::StackedDocument => write!(f, "docs"),
            Self::StackedRectangle => write!(f, "processes"),
            Self::Flag => write!(f, "flag"),
            Self::BowTieRectangle => write!(f, "bow-rect"),
            Self::CrossedCircle => write!(f, "cross-circ"),
            Self::TaggedDocument => write!(f, "tag-doc"),
            Self::TaggedRectangle => write!(f, "tag-rect"),
            Self::FramedRectangle => write!(f, "fr-rect"),
            Self::TextBlock => write!(f, "text"),
        }
    }
}

impl FromStr for FlowchartNodeShape {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            // Rectangle
            "rect" | "rectangle" | "proc" | "process" => Ok(Self::Rectangle),
            // Rounded Rectangle
            "rounded" | "event" => Ok(Self::RoundEdges),
            // Stadium
            "stadium" | "pill" | "terminal" => Ok(Self::StadiumShape),
            // Subprocess
            "subproc" | "subprocess" | "subroutine" | "framed-rectangle" => Ok(Self::Subprocess),
            // Cylinder
            "cyl" | "cylinder" | "database" | "db" => Ok(Self::Cylinder),
            // Circle
            "circle" | "circ" => Ok(Self::Circle),
            // Odd
            "odd" => Ok(Self::Odd),
            // Diamond
            "diamond" | "diam" | "decision" | "question" => Ok(Self::Diamond),
            // Hexagon
            "hex" | "hexagon" | "prepare" => Ok(Self::Hexagon),
            // Lean right parallelogram
            "lean-r" | "lean-right" | "in-out" => Ok(Self::LRParallelogram),
            // Lean left parallelogram
            "lean-l" | "lean-left" | "out-in" => Ok(Self::LLParallelogram),
            // Base bottom trapezoid
            "trap-b" | "trapezoid" | "priority" | "trapezoid-bottom" => Ok(Self::Trapezoid),
            // Base top trapezoid
            "trap-t" | "inv-trapezoid" | "manual" | "trapezoid-top" => Ok(Self::ReverseTrapezoid),
            // Double Circle
            "dbl-circ" | "double-circle" | "stop" => Ok(Self::DoubleCircle),
            // Notched Rectangle
            "notch-rect" | "card" | "notched-rectangle" => Ok(Self::NotchedRectangle),
            // Lined Rectangle
            "lin-rect" | "lin-proc" | "lined-process" | "lined-rectangle" | "shaded-process" => {
                Ok(Self::Linedrectangle)
            }
            // Small Circle
            "sm-circ" | "small-circle" | "start" => Ok(Self::SmallCircle),
            // Framed Circle
            "framed-circle" | "fr-circ" => Ok(Self::FramedCircle),
            // Long Rectangle
            "fork" | "join" => Ok(Self::LongRectangle),
            // Hourglass
            "hourglass" | "collate" => Ok(Self::Hourglass),
            // Left Curly Brace
            "comment" | "brace-l" => Ok(Self::LeftCurlyBrace),
            // Right Curly Brace
            "brace-r" => Ok(Self::RightCurlyBrace),
            // Curly Braces
            "braces" => Ok(Self::CurlyBraces),
            // Lightning Bolt
            "bolt" | "com-link" | "lightning-bolt" => Ok(Self::LightningBolt),
            // Document
            "doc" | "document" => Ok(Self::Document),
            // Half-Rounded Rectangle
            "delay" | "half-rounded-rectangle" => Ok(Self::HalfRoundedRectangle),
            // Horizontal Cylinder
            "das" | "h-cyl" | "horizontal-cylinder" => Ok(Self::HorizontalCylinder),
            // Lined Cylinder
            "lin-cyl" | "disk" | "lined-cylinder" => Ok(Self::LinedCylinder),
            // Curved Trapezoid
            "curv-trap" | "curved-trapezoid" | "display" => Ok(Self::CurvedTrapezoid),
            // Divided Rectangle
            "div-rect" | "div-proc" | "divided-process" | "divided-rectangle" => {
                Ok(Self::DividedRectangle)
            }
            // Small Triangle
            "tri" | "extract" | "triangle" => Ok(Self::SmallTriangle),
            // Window Pane
            "win-pane" | "internal-storage" | "window-pane" => Ok(Self::WindowPane),
            // Filled Circle
            "f-circ" | "filled-circle" | "junction" => Ok(Self::FilledCircle),
            // Lined Document
            "lin-doc" | "lined-document" => Ok(Self::LinedDocument),
            // Notched Pentagon
            "notch-pent" | "loop-limit" | "notched-pentagon" => Ok(Self::NotchedPentagon),
            // Flipped Triangle
            "flip-tri" | "flipped-triangle" | "manual-file" => Ok(Self::FlippedTriangle),
            // Sloped Rectangle
            "sl-rect" | "manual-input" | "sloped-rectangle" => Ok(Self::SlopedRectangle),
            // Stacked Document
            "docs" | "documents" | "st-doc" | "stacked-document" => Ok(Self::StackedDocument),
            // Stacked Rectangle
            "processes" | "procs" | "st-rect" | "stacked-rectangle" => Ok(Self::StackedRectangle),
            // Flag
            "flag" | "paper-tape" => Ok(Self::Flag),
            // Bow Tie Rectangle
            "bow-rect" | "bow-tie-rectangle" | "stored-data" => Ok(Self::BowTieRectangle),
            // Crossed Circle
            "cross-circ" | "crossed-circle" | "summary" => Ok(Self::CrossedCircle),
            // Tagged Document
            "tag-doc" | "tagged-document" => Ok(Self::TaggedDocument),
            // Tagged Rectangle
            "tag-rect" | "tag-proc" | "tagged-process" | "tagged-rectangle" => {
                Ok(Self::TaggedRectangle)
            }
            // Framed Rectangle (added for completeness)
            "fr-rect" => Ok(Self::FramedRectangle),
            // Text Block
            "text" | "text-block" => Ok(Self::TextBlock),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flowchart_node_shape_display() {
        assert_eq!(format!("{}", FlowchartNodeShape::Rectangle), "rect");
        assert_eq!(format!("{}", FlowchartNodeShape::RoundEdges), "rounded");
        assert_eq!(format!("{}", FlowchartNodeShape::StadiumShape), "stadium");
        assert_eq!(format!("{}", FlowchartNodeShape::Subprocess), "subproc");
        assert_eq!(format!("{}", FlowchartNodeShape::Cylinder), "cyl");
        assert_eq!(format!("{}", FlowchartNodeShape::Circle), "circle");
        assert_eq!(format!("{}", FlowchartNodeShape::Odd), "odd");
        assert_eq!(format!("{}", FlowchartNodeShape::Diamond), "diamond");
        assert_eq!(format!("{}", FlowchartNodeShape::Hexagon), "hex");
        assert_eq!(format!("{}", FlowchartNodeShape::LRParallelogram), "lean-r");
        assert_eq!(format!("{}", FlowchartNodeShape::LLParallelogram), "lean-l");
        assert_eq!(format!("{}", FlowchartNodeShape::Trapezoid), "trap-b");
        assert_eq!(format!("{}", FlowchartNodeShape::ReverseTrapezoid), "trap-t");
        assert_eq!(format!("{}", FlowchartNodeShape::DoubleCircle), "dbl-circ");
        assert_eq!(format!("{}", FlowchartNodeShape::NotchedRectangle), "notch-rect");
        assert_eq!(format!("{}", FlowchartNodeShape::Linedrectangle), "lin-rect");
        assert_eq!(format!("{}", FlowchartNodeShape::SmallCircle), "sm-circ");
        assert_eq!(format!("{}", FlowchartNodeShape::FramedCircle), "framed-circle");
        assert_eq!(format!("{}", FlowchartNodeShape::LongRectangle), "fork");
        assert_eq!(format!("{}", FlowchartNodeShape::Hourglass), "hourglass");
        assert_eq!(format!("{}", FlowchartNodeShape::LeftCurlyBrace), "comment");
        assert_eq!(format!("{}", FlowchartNodeShape::RightCurlyBrace), "brace-r");
        assert_eq!(format!("{}", FlowchartNodeShape::CurlyBraces), "braces");
        assert_eq!(format!("{}", FlowchartNodeShape::LightningBolt), "bolt");
        assert_eq!(format!("{}", FlowchartNodeShape::Document), "doc");
        assert_eq!(format!("{}", FlowchartNodeShape::HalfRoundedRectangle), "delay");
        assert_eq!(format!("{}", FlowchartNodeShape::HorizontalCylinder), "das");
        assert_eq!(format!("{}", FlowchartNodeShape::LinedCylinder), "lin-cyl");
        assert_eq!(format!("{}", FlowchartNodeShape::CurvedTrapezoid), "curv-trap");
        assert_eq!(format!("{}", FlowchartNodeShape::DividedRectangle), "div-rect");
        assert_eq!(format!("{}", FlowchartNodeShape::SmallTriangle), "tri");
        assert_eq!(format!("{}", FlowchartNodeShape::WindowPane), "win-pane");
        assert_eq!(format!("{}", FlowchartNodeShape::FilledCircle), "f-circ");
        assert_eq!(format!("{}", FlowchartNodeShape::LinedDocument), "lin-doc");
        assert_eq!(format!("{}", FlowchartNodeShape::NotchedPentagon), "notch-pent");
        assert_eq!(format!("{}", FlowchartNodeShape::FlippedTriangle), "flip-tri");
        assert_eq!(format!("{}", FlowchartNodeShape::SlopedRectangle), "sl-rect");
        assert_eq!(format!("{}", FlowchartNodeShape::StackedDocument), "docs");
        assert_eq!(format!("{}", FlowchartNodeShape::StackedRectangle), "processes");
        assert_eq!(format!("{}", FlowchartNodeShape::Flag), "flag");
        assert_eq!(format!("{}", FlowchartNodeShape::BowTieRectangle), "bow-rect");
        assert_eq!(format!("{}", FlowchartNodeShape::CrossedCircle), "cross-circ");
        assert_eq!(format!("{}", FlowchartNodeShape::TaggedDocument), "tag-doc");
        assert_eq!(format!("{}", FlowchartNodeShape::TaggedRectangle), "tag-rect");
        assert_eq!(format!("{}", FlowchartNodeShape::FramedRectangle), "fr-rect");
        assert_eq!(format!("{}", FlowchartNodeShape::TextBlock), "text");
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_flowchart_node_shape_from_str() {
        assert_eq!(FlowchartNodeShape::from_str("rect").unwrap(), FlowchartNodeShape::Rectangle);
        assert_eq!(
            FlowchartNodeShape::from_str("rectangle").unwrap(),
            FlowchartNodeShape::Rectangle
        );
        assert_eq!(FlowchartNodeShape::from_str("proc").unwrap(), FlowchartNodeShape::Rectangle);
        assert_eq!(FlowchartNodeShape::from_str("process").unwrap(), FlowchartNodeShape::Rectangle);
        assert_eq!(
            FlowchartNodeShape::from_str("rounded").unwrap(),
            FlowchartNodeShape::RoundEdges
        );
        assert_eq!(FlowchartNodeShape::from_str("event").unwrap(), FlowchartNodeShape::RoundEdges);
        assert_eq!(
            FlowchartNodeShape::from_str("stadium").unwrap(),
            FlowchartNodeShape::StadiumShape
        );
        assert_eq!(FlowchartNodeShape::from_str("pill").unwrap(), FlowchartNodeShape::StadiumShape);
        assert_eq!(
            FlowchartNodeShape::from_str("terminal").unwrap(),
            FlowchartNodeShape::StadiumShape
        );
        assert_eq!(
            FlowchartNodeShape::from_str("subproc").unwrap(),
            FlowchartNodeShape::Subprocess
        );
        assert_eq!(
            FlowchartNodeShape::from_str("subprocess").unwrap(),
            FlowchartNodeShape::Subprocess
        );
        assert_eq!(
            FlowchartNodeShape::from_str("subroutine").unwrap(),
            FlowchartNodeShape::Subprocess
        );
        assert_eq!(
            FlowchartNodeShape::from_str("framed-rectangle").unwrap(),
            FlowchartNodeShape::Subprocess
        );
        assert_eq!(FlowchartNodeShape::from_str("cyl").unwrap(), FlowchartNodeShape::Cylinder);
        assert_eq!(FlowchartNodeShape::from_str("cylinder").unwrap(), FlowchartNodeShape::Cylinder);
        assert_eq!(FlowchartNodeShape::from_str("database").unwrap(), FlowchartNodeShape::Cylinder);
        assert_eq!(FlowchartNodeShape::from_str("db").unwrap(), FlowchartNodeShape::Cylinder);
        assert_eq!(FlowchartNodeShape::from_str("circle").unwrap(), FlowchartNodeShape::Circle);
        assert_eq!(FlowchartNodeShape::from_str("circ").unwrap(), FlowchartNodeShape::Circle);
        assert_eq!(FlowchartNodeShape::from_str("odd").unwrap(), FlowchartNodeShape::Odd);
        assert_eq!(FlowchartNodeShape::from_str("diamond").unwrap(), FlowchartNodeShape::Diamond);
        assert_eq!(FlowchartNodeShape::from_str("diam").unwrap(), FlowchartNodeShape::Diamond);
        assert_eq!(FlowchartNodeShape::from_str("decision").unwrap(), FlowchartNodeShape::Diamond);
        assert_eq!(FlowchartNodeShape::from_str("question").unwrap(), FlowchartNodeShape::Diamond);
        assert_eq!(FlowchartNodeShape::from_str("hex").unwrap(), FlowchartNodeShape::Hexagon);
        assert_eq!(FlowchartNodeShape::from_str("hexagon").unwrap(), FlowchartNodeShape::Hexagon);
        assert_eq!(FlowchartNodeShape::from_str("prepare").unwrap(), FlowchartNodeShape::Hexagon);
        assert_eq!(
            FlowchartNodeShape::from_str("lean-r").unwrap(),
            FlowchartNodeShape::LRParallelogram
        );
        assert_eq!(
            FlowchartNodeShape::from_str("lean-right").unwrap(),
            FlowchartNodeShape::LRParallelogram
        );
        assert_eq!(
            FlowchartNodeShape::from_str("in-out").unwrap(),
            FlowchartNodeShape::LRParallelogram
        );
        assert_eq!(
            FlowchartNodeShape::from_str("lean-l").unwrap(),
            FlowchartNodeShape::LLParallelogram
        );
        assert_eq!(
            FlowchartNodeShape::from_str("lean-left").unwrap(),
            FlowchartNodeShape::LLParallelogram
        );
        assert_eq!(
            FlowchartNodeShape::from_str("out-in").unwrap(),
            FlowchartNodeShape::LLParallelogram
        );
        assert_eq!(FlowchartNodeShape::from_str("trap-b").unwrap(), FlowchartNodeShape::Trapezoid);
        assert_eq!(
            FlowchartNodeShape::from_str("trapezoid").unwrap(),
            FlowchartNodeShape::Trapezoid
        );
        assert_eq!(
            FlowchartNodeShape::from_str("priority").unwrap(),
            FlowchartNodeShape::Trapezoid
        );
        assert_eq!(
            FlowchartNodeShape::from_str("trapezoid-bottom").unwrap(),
            FlowchartNodeShape::Trapezoid
        );
        assert_eq!(
            FlowchartNodeShape::from_str("trap-t").unwrap(),
            FlowchartNodeShape::ReverseTrapezoid
        );
        assert_eq!(
            FlowchartNodeShape::from_str("inv-trapezoid").unwrap(),
            FlowchartNodeShape::ReverseTrapezoid
        );
        assert_eq!(
            FlowchartNodeShape::from_str("manual").unwrap(),
            FlowchartNodeShape::ReverseTrapezoid
        );
        assert_eq!(
            FlowchartNodeShape::from_str("trapezoid-top").unwrap(),
            FlowchartNodeShape::ReverseTrapezoid
        );
        assert_eq!(
            FlowchartNodeShape::from_str("dbl-circ").unwrap(),
            FlowchartNodeShape::DoubleCircle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("double-circle").unwrap(),
            FlowchartNodeShape::DoubleCircle
        );
        assert_eq!(FlowchartNodeShape::from_str("stop").unwrap(), FlowchartNodeShape::DoubleCircle);
        assert_eq!(
            FlowchartNodeShape::from_str("notch-rect").unwrap(),
            FlowchartNodeShape::NotchedRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("card").unwrap(),
            FlowchartNodeShape::NotchedRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("notched-rectangle").unwrap(),
            FlowchartNodeShape::NotchedRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("lin-rect").unwrap(),
            FlowchartNodeShape::Linedrectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("lin-proc").unwrap(),
            FlowchartNodeShape::Linedrectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("lined-process").unwrap(),
            FlowchartNodeShape::Linedrectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("lined-rectangle").unwrap(),
            FlowchartNodeShape::Linedrectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("shaded-process").unwrap(),
            FlowchartNodeShape::Linedrectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("sm-circ").unwrap(),
            FlowchartNodeShape::SmallCircle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("small-circle").unwrap(),
            FlowchartNodeShape::SmallCircle
        );
        assert_eq!(FlowchartNodeShape::from_str("start").unwrap(), FlowchartNodeShape::SmallCircle);
        assert_eq!(
            FlowchartNodeShape::from_str("framed-circle").unwrap(),
            FlowchartNodeShape::FramedCircle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("fr-circ").unwrap(),
            FlowchartNodeShape::FramedCircle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("fork").unwrap(),
            FlowchartNodeShape::LongRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("join").unwrap(),
            FlowchartNodeShape::LongRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("hourglass").unwrap(),
            FlowchartNodeShape::Hourglass
        );
        assert_eq!(FlowchartNodeShape::from_str("collate").unwrap(), FlowchartNodeShape::Hourglass);
        assert_eq!(
            FlowchartNodeShape::from_str("comment").unwrap(),
            FlowchartNodeShape::LeftCurlyBrace
        );
        assert_eq!(
            FlowchartNodeShape::from_str("brace-l").unwrap(),
            FlowchartNodeShape::LeftCurlyBrace
        );
        assert_eq!(
            FlowchartNodeShape::from_str("brace-r").unwrap(),
            FlowchartNodeShape::RightCurlyBrace
        );
        assert_eq!(
            FlowchartNodeShape::from_str("braces").unwrap(),
            FlowchartNodeShape::CurlyBraces
        );
        assert_eq!(
            FlowchartNodeShape::from_str("bolt").unwrap(),
            FlowchartNodeShape::LightningBolt
        );
        assert_eq!(
            FlowchartNodeShape::from_str("com-link").unwrap(),
            FlowchartNodeShape::LightningBolt
        );
        assert_eq!(
            FlowchartNodeShape::from_str("lightning-bolt").unwrap(),
            FlowchartNodeShape::LightningBolt
        );
        assert_eq!(FlowchartNodeShape::from_str("doc").unwrap(), FlowchartNodeShape::Document);
        assert_eq!(FlowchartNodeShape::from_str("document").unwrap(), FlowchartNodeShape::Document);
        assert_eq!(
            FlowchartNodeShape::from_str("delay").unwrap(),
            FlowchartNodeShape::HalfRoundedRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("half-rounded-rectangle").unwrap(),
            FlowchartNodeShape::HalfRoundedRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("das").unwrap(),
            FlowchartNodeShape::HorizontalCylinder
        );
        assert_eq!(
            FlowchartNodeShape::from_str("h-cyl").unwrap(),
            FlowchartNodeShape::HorizontalCylinder
        );
        assert_eq!(
            FlowchartNodeShape::from_str("horizontal-cylinder").unwrap(),
            FlowchartNodeShape::HorizontalCylinder
        );
        assert_eq!(
            FlowchartNodeShape::from_str("lin-cyl").unwrap(),
            FlowchartNodeShape::LinedCylinder
        );
        assert_eq!(
            FlowchartNodeShape::from_str("disk").unwrap(),
            FlowchartNodeShape::LinedCylinder
        );
        assert_eq!(
            FlowchartNodeShape::from_str("lined-cylinder").unwrap(),
            FlowchartNodeShape::LinedCylinder
        );
        assert_eq!(
            FlowchartNodeShape::from_str("curv-trap").unwrap(),
            FlowchartNodeShape::CurvedTrapezoid
        );
        assert_eq!(
            FlowchartNodeShape::from_str("curved-trapezoid").unwrap(),
            FlowchartNodeShape::CurvedTrapezoid
        );
        assert_eq!(
            FlowchartNodeShape::from_str("display").unwrap(),
            FlowchartNodeShape::CurvedTrapezoid
        );
        assert_eq!(
            FlowchartNodeShape::from_str("div-rect").unwrap(),
            FlowchartNodeShape::DividedRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("div-proc").unwrap(),
            FlowchartNodeShape::DividedRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("divided-process").unwrap(),
            FlowchartNodeShape::DividedRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("divided-rectangle").unwrap(),
            FlowchartNodeShape::DividedRectangle
        );
        assert_eq!(FlowchartNodeShape::from_str("tri").unwrap(), FlowchartNodeShape::SmallTriangle);
        assert_eq!(
            FlowchartNodeShape::from_str("extract").unwrap(),
            FlowchartNodeShape::SmallTriangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("triangle").unwrap(),
            FlowchartNodeShape::SmallTriangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("win-pane").unwrap(),
            FlowchartNodeShape::WindowPane
        );
        assert_eq!(
            FlowchartNodeShape::from_str("internal-storage").unwrap(),
            FlowchartNodeShape::WindowPane
        );
        assert_eq!(
            FlowchartNodeShape::from_str("window-pane").unwrap(),
            FlowchartNodeShape::WindowPane
        );
        assert_eq!(
            FlowchartNodeShape::from_str("f-circ").unwrap(),
            FlowchartNodeShape::FilledCircle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("filled-circle").unwrap(),
            FlowchartNodeShape::FilledCircle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("junction").unwrap(),
            FlowchartNodeShape::FilledCircle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("lin-doc").unwrap(),
            FlowchartNodeShape::LinedDocument
        );
        assert_eq!(
            FlowchartNodeShape::from_str("lined-document").unwrap(),
            FlowchartNodeShape::LinedDocument
        );
        assert_eq!(
            FlowchartNodeShape::from_str("notch-pent").unwrap(),
            FlowchartNodeShape::NotchedPentagon
        );
        assert_eq!(
            FlowchartNodeShape::from_str("loop-limit").unwrap(),
            FlowchartNodeShape::NotchedPentagon
        );
        assert_eq!(
            FlowchartNodeShape::from_str("notched-pentagon").unwrap(),
            FlowchartNodeShape::NotchedPentagon
        );
        assert_eq!(
            FlowchartNodeShape::from_str("flip-tri").unwrap(),
            FlowchartNodeShape::FlippedTriangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("flipped-triangle").unwrap(),
            FlowchartNodeShape::FlippedTriangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("manual-file").unwrap(),
            FlowchartNodeShape::FlippedTriangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("sl-rect").unwrap(),
            FlowchartNodeShape::SlopedRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("manual-input").unwrap(),
            FlowchartNodeShape::SlopedRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("sloped-rectangle").unwrap(),
            FlowchartNodeShape::SlopedRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("docs").unwrap(),
            FlowchartNodeShape::StackedDocument
        );
        assert_eq!(
            FlowchartNodeShape::from_str("documents").unwrap(),
            FlowchartNodeShape::StackedDocument
        );
        assert_eq!(
            FlowchartNodeShape::from_str("st-doc").unwrap(),
            FlowchartNodeShape::StackedDocument
        );
        assert_eq!(
            FlowchartNodeShape::from_str("stacked-document").unwrap(),
            FlowchartNodeShape::StackedDocument
        );
        assert_eq!(
            FlowchartNodeShape::from_str("processes").unwrap(),
            FlowchartNodeShape::StackedRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("procs").unwrap(),
            FlowchartNodeShape::StackedRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("st-rect").unwrap(),
            FlowchartNodeShape::StackedRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("stacked-rectangle").unwrap(),
            FlowchartNodeShape::StackedRectangle
        );
        assert_eq!(FlowchartNodeShape::from_str("flag").unwrap(), FlowchartNodeShape::Flag);
        assert_eq!(FlowchartNodeShape::from_str("paper-tape").unwrap(), FlowchartNodeShape::Flag);
        assert_eq!(
            FlowchartNodeShape::from_str("bow-rect").unwrap(),
            FlowchartNodeShape::BowTieRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("bow-tie-rectangle").unwrap(),
            FlowchartNodeShape::BowTieRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("stored-data").unwrap(),
            FlowchartNodeShape::BowTieRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("cross-circ").unwrap(),
            FlowchartNodeShape::CrossedCircle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("crossed-circle").unwrap(),
            FlowchartNodeShape::CrossedCircle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("summary").unwrap(),
            FlowchartNodeShape::CrossedCircle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("tag-doc").unwrap(),
            FlowchartNodeShape::TaggedDocument
        );
        assert_eq!(
            FlowchartNodeShape::from_str("tagged-document").unwrap(),
            FlowchartNodeShape::TaggedDocument
        );
        assert_eq!(
            FlowchartNodeShape::from_str("tag-rect").unwrap(),
            FlowchartNodeShape::TaggedRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("tag-proc").unwrap(),
            FlowchartNodeShape::TaggedRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("tagged-process").unwrap(),
            FlowchartNodeShape::TaggedRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("tagged-rectangle").unwrap(),
            FlowchartNodeShape::TaggedRectangle
        );
        assert_eq!(
            FlowchartNodeShape::from_str("fr-rect").unwrap(),
            FlowchartNodeShape::FramedRectangle
        );
        assert_eq!(FlowchartNodeShape::from_str("text").unwrap(), FlowchartNodeShape::TextBlock);
        assert_eq!(
            FlowchartNodeShape::from_str("text-block").unwrap(),
            FlowchartNodeShape::TextBlock
        );
        assert!(FlowchartNodeShape::from_str("invalid").is_err());
    }
}
