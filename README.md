# Mermaid Builder

[![CI](https://github.com/earth-metabolome-initiative/mermaid-builder/workflows/Rust%20CI/badge.svg)](https://github.com/earth-metabolome-initiative/mermaid-builder/actions)
[![Security Audit](https://github.com/earth-metabolome-initiative/mermaid-builder/workflows/Security%20Audit/badge.svg)](https://github.com/earth-metabolome-initiative/mermaid-builder/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Codecov](https://codecov.io/gh/earth-metabolome-initiative/mermaid-builder/branch/main/graph/badge.svg)](https://codecov.io/gh/earth-metabolome-initiative/mermaid-builder)
[![Crates.io](https://img.shields.io/crates/v/mermaid-builder.svg)](https://crates.io/crates/mermaid-builder)
[![Docs.rs](https://docs.rs/mermaid-builder/badge.svg)](https://docs.rs/mermaid-builder)

**Mermaid Builder** is a Rust crate that provides a type-safe, builder-pattern-based API for generating [Mermaid](https://mermaid.js.org/) diagram syntax. It allows you to define diagrams programmatically in Rust and export them as strings that can be rendered by Mermaid tools.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mermaid-builder = "0.1.0"
```

Or run:

```bash
cargo add mermaid-builder
```

## Examples

### Flowchart

```rust
use mermaid_builder::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = FlowchartBuilder::default();

    // Create nodes
    let node1 = builder
        .node(FlowchartNodeBuilder::default().label("Start")?)?;
    let node2 = builder
        .node(FlowchartNodeBuilder::default().label("End")?)?;

    // Create edge
    builder
        .edge(
            FlowchartEdgeBuilder::default()
                .source(node1)?
                .destination(node2)?
                .right_arrow_shape(ArrowShape::Normal)?
        )?;

    // Build the flowchart
    let flowchart = Flowchart::from(builder);

    // Print the mermaid syntax
    println!("{}", flowchart);

    let expected = r#"flowchart LR
  v0@{shape: rect, label: "Start"}
  v1@{shape: rect, label: "End"}
  v0 ---> v1
"#;
    assert_eq!(flowchart.to_string(), expected);
    Ok(())
}
```

### Class Diagram

```rust
use mermaid_builder::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = ClassDiagramBuilder::default();

    // Create class nodes
    let animal = builder
        .node(ClassNodeBuilder::default().label("Animal")?)?;
    let dog = builder
        .node(ClassNodeBuilder::default().label("Dog")?)?;

    // Create inheritance edge
    builder
        .edge(
            ClassEdgeBuilder::default()
                .source(animal)?
                .destination(dog)?
                .right_arrow_shape(ArrowShape::Triangle)?
        )?;

    let class_diagram = ClassDiagram::from(builder);
    println!("{}", class_diagram);

    let expected = r#"---
config:
  class:
    hideEmptyMembersBox: "false"
---
classDiagram
  direction LR
  class v0[Animal] {
  }
  class v1[Dog] {
  }
  v0 --|> v1
"#;
    assert_eq!(class_diagram.to_string(), expected);
    Ok(())
}
```

### Entity Relationship Diagram

```rust
use mermaid_builder::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = ERDiagramBuilder::default();

    let customer = builder
        .node(ERNodeBuilder::default().label("CUSTOMER")?)?;
    let order = builder
        .node(ERNodeBuilder::default().label("ORDER")?)?;

    // Create relationship
    builder
        .edge(EREdgeBuilder::one_or_more(customer, order))?;

    let er_diagram = ERDiagram::from(builder);
    println!("{}", er_diagram);

    let expected = r#"---
config:
  layout: dagre
  theme: default
  look: classic
---
erDiagram
  direction LR
  v0["CUSTOMER"]
  v1["ORDER"]
  v0 }|--|{ v1 : ""
"#;
    assert_eq!(er_diagram.to_string(), expected);
    Ok(())
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.
