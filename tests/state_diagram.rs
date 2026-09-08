//! Regression tests for state diagram rendering and validation.

use mermaid_builder::{ConfigError, EdgeError, NodeError, prelude::*};

type Result = core::result::Result<(), Box<dyn core::error::Error>>;

#[test]
fn leaf_states_and_escaped_transitions() -> Result {
    let mut builder = StateDiagramBuilder::default();
    let a = builder.node(StateNodeBuilder::default().label("Same label")?)?;
    let b = builder.node(StateNodeBuilder::default().label("Same label")?)?;
    builder.edge(
        StateEdgeBuilder::default().source(a)?.destination(b)?.label("Foo::Bar;\n#x \"<&>%%")?,
    )?;
    assert_eq!(
        StateDiagram::from(builder).to_string(),
        concat!(
            "stateDiagram-v2\n    direction LR\n",
            "    state \"Same label\" as S0\n    state \"Same label\" as S1\n",
            "    S0 --> S1 : Foo#colon;#colon;Bar#59; #35;x #quot;#60;#38;#62;#37;#37;\n"
        )
    );
    Ok(())
}

#[test]
fn nested_scopes_directions_and_empty_composites() -> Result {
    let mut inner = StateDiagramBuilder::default().configuration(
        StateDiagramConfigurationBuilder::default().direction(Direction::TopToBottom),
    )?;
    let a = inner.node(StateNodeBuilder::default().label("Inner")?)?;
    let b = inner.node(StateNodeBuilder::default().label("Inner")?)?;
    inner.edge(StateEdgeBuilder::default().source(a)?.destination(b)?)?;
    let inner = StateDiagram::from(inner);
    let mut builder = StateDiagramBuilder::default();
    builder.node(StateNodeBuilder::default().label("Parent")?.inner_diagram(inner.clone())?)?;
    builder.node(StateNodeBuilder::default().label("Parent")?.inner_diagram(inner)?)?;
    builder.node(
        StateNodeBuilder::default()
            .label("Empty")?
            .inner_diagram(StateDiagramBuilder::default().into())?,
    )?;
    assert_eq!(
        StateDiagram::from(builder).to_string(),
        concat!(
            "stateDiagram-v2\n    direction LR\n",
            "    state \"Parent\" as S0\n    state S0 {\n        direction TB\n",
            "        state \"Inner\" as S0_0\n        state \"Inner\" as S0_1\n        S0_0 --> S0_1\n    }\n",
            "    state \"Parent\" as S1\n    state S1 {\n        direction TB\n",
            "        state \"Inner\" as S1_0\n        state \"Inner\" as S1_1\n        S1_0 --> S1_1\n    }\n",
            "    state \"Empty\" as S2\n    state S2 {\n        direction LR\n    }\n"
        )
    );
    Ok(())
}

#[test]
fn special_states() -> Result {
    let mut builder = StateDiagramBuilder::default();
    let mut nodes = Vec::new();
    for kind in
        [StateKind::Start, StateKind::Choice, StateKind::Fork, StateKind::Join, StateKind::End]
    {
        nodes.push(builder.node(StateNodeBuilder::default().kind(kind))?);
    }
    for pair in nodes.windows(2) {
        builder.edge(
            StateEdgeBuilder::default().source(pair[0].clone())?.destination(pair[1].clone())?,
        )?;
    }
    assert_eq!(
        StateDiagram::from(builder).to_string(),
        concat!(
            "stateDiagram-v2\n    direction LR\n",
            "    state S1 <<choice>>\n    state S2 <<fork>>\n    state S3 <<join>>\n",
            "    [*] --> S1\n    S1 --> S2\n    S2 --> S3\n    S3 --> [*]\n"
        )
    );
    assert!(matches!(
        StateEdgeBuilder::default()
            .source(nodes[4].clone())?
            .destination(nodes[1].clone())?
            .build(),
        Err(EdgeError::InvalidStateTransition)
    ));
    assert!(matches!(
        StateEdgeBuilder::default()
            .source(nodes[1].clone())?
            .destination(nodes[0].clone())?
            .build(),
        Err(EdgeError::InvalidStateTransition)
    ));
    assert!(matches!(
        StateNodeBuilder::default()
            .kind(StateKind::Choice)
            .id(0)
            .inner_diagram(StateDiagramBuilder::default().into())?
            .build(),
        Err(NodeError::InvalidStateKind)
    ));
    Ok(())
}

#[test]
fn styles_and_classes_are_emitted() -> Result {
    let mut builder = StateDiagramBuilder::default();
    let class = builder.style_class(
        StyleClassBuilder::default()
            .name("active")?
            .property(StyleProperty::Fill(Color::from((255, 0, 0))))?,
    )?;
    builder.node(
        StateNodeBuilder::default()
            .label("Styled")?
            .style_class(class)?
            .style_property(StyleProperty::StrokeWidth(Unit::Pixel(2)))?,
    )?;
    assert_eq!(
        StateDiagram::from(builder).to_string(),
        concat!(
            "stateDiagram-v2\n    direction LR\n    state \"Styled\" as S0\n",
            "    classDef Sclass_active fill: #ff0000\n    class S0 Sclass_active\n    style S0 stroke-width: 2px\n"
        )
    );
    Ok(())
}

#[test]
fn configuration_and_nested_validation() -> Result {
    assert!(matches!(
        StateDiagramConfigurationBuilder::default().title(""),
        Err(ConfigError::EmptyTitle)
    ));
    let config = StateDiagramConfigurationBuilder::default()
        .title("Machine")?
        .renderer(Renderer::EclipseLayoutKernel);
    let diagram = StateDiagram::from(StateDiagramBuilder::default().configuration(config)?);
    assert_eq!(
        diagram.to_string(),
        "---\nconfig:\n  layout: elk\n  theme: default\n  look: classic\ntitle: Machine\n---\nstateDiagram-v2\n    direction LR\n"
    );
    assert!(matches!(
        StateNodeBuilder::default().inner_diagram(diagram),
        Err(NodeError::NestedStateConfiguration)
    ));
    let diagram = StateDiagram::from(StateDiagramBuilder::default().configuration(
        StateDiagramConfigurationBuilder::default().renderer(Renderer::EclipseLayoutKernel),
    )?);
    assert!(matches!(
        StateNodeBuilder::default().inner_diagram(diagram),
        Err(NodeError::NestedStateConfiguration)
    ));
    Ok(())
}

#[test]
fn unsupported_edges_and_missing_fields() -> Result {
    assert!(matches!(StateNodeBuilder::default().build(), Err(NodeError::MissingId)));
    assert!(matches!(StateNodeBuilder::default().id(0).build(), Err(NodeError::MissingLabel)));
    assert!(matches!(StateNodeBuilder::default().label(""), Err(NodeError::EmptyLabel)));
    assert!(matches!(StateEdgeBuilder::default().build(), Err(EdgeError::MissingSource)));
    assert!(matches!(StateEdgeBuilder::default().label(""), Err(EdgeError::EmptyLabel)));
    let node = std::rc::Rc::new(StateNodeBuilder::default().id(0).label("A")?.build()?);
    assert!(matches!(
        StateEdgeBuilder::default().source(node.clone())?.build(),
        Err(EdgeError::MissingDestination)
    ));
    for style in [LineStyle::Dashed, LineStyle::Thick] {
        assert!(matches!(
            StateEdgeBuilder::default()
                .source(node.clone())?
                .destination(node.clone())?
                .line_style(style)
                .build(),
            Err(EdgeError::UnsupportedStateLineStyle)
        ));
    }
    for shape in [
        ArrowShape::Normal,
        ArrowShape::Sharp,
        ArrowShape::X,
        ArrowShape::Circle,
        ArrowShape::Triangle,
        ArrowShape::Star,
        ArrowShape::ZeroOrOne,
        ArrowShape::ExactlyOne,
        ArrowShape::ZeroOrMore,
        ArrowShape::OneOrMore,
    ] {
        assert!(StateEdgeBuilder::default().left_arrow_shape(shape).is_err());
        assert_eq!(
            StateEdgeBuilder::default().right_arrow_shape(shape).is_ok(),
            shape == ArrowShape::Normal
        );
    }
    let edge = StateEdgeBuilder::default()
        .source(node.clone())?
        .destination(node)?
        .right_arrow_shape(ArrowShape::Normal)?
        .build()?;
    assert_eq!(edge.line_style(), LineStyle::Solid);
    assert_eq!(edge.left_arrow_shape(), None);
    assert_eq!(edge.right_arrow_shape(), Some(ArrowShape::Normal));
    assert_eq!(edge.classes().count(), 0);
    assert_eq!(edge.to_string(), "S0 --> S0\n");
    Ok(())
}

#[test]
fn ids_and_builder_accessors() -> Result {
    let mut builder = StateDiagramBuilder::default();
    let a = builder.node(StateNodeBuilder::default().label("A")?.id(1))?;
    let b = builder.node(StateNodeBuilder::default().label("B")?)?;
    assert_eq!(b.id(), 0);
    assert!(builder.node(StateNodeBuilder::default().label("Duplicate")?.id(1)).is_err());
    assert_eq!(builder.number_of_nodes(), 2);
    assert_eq!(builder.nodes().count(), 2);
    assert_eq!(builder.get_node_by_id(1), Some(a.clone()));
    assert!(builder.get_node_by_id(3).is_none());
    builder.edge(StateEdgeBuilder::default().source(a.clone())?.destination(b)?)?;
    assert_eq!(builder.number_of_edges(), 1);
    let diagram = StateDiagram::from(builder);
    assert_eq!(diagram.get_node_by_id(1), Some(a));
    assert!(diagram.get_style_class_by_name("missing").is_none());
    Ok(())
}

#[test]
fn standalone_nodes_and_style_validation() -> Result {
    let builder = StateNodeBuilder::default()
        .id(9)
        .label("Quoted \"label\";\nnext")?
        .style_property(StyleProperty::StrokeWidth(Unit::Pixel(2)))?
        .style_property(StyleProperty::Fill(Color::from((0, 0, 0))))?;
    assert_eq!(builder.get_label().map(String::as_str), Some("Quoted \"label\";\nnext"));
    assert_eq!(builder.style_properties().count(), 2);
    assert!(builder.clone().style_property(StyleProperty::StrokeWidth(Unit::Pixel(3))).is_err());
    assert_eq!(
        builder.build()?.to_string(),
        "state \"Quoted #quot;label#quot;#59; next\" as S9\nstyle S9 stroke-width: 2px,fill: #000000\n"
    );
    assert!(matches!(
        StateNodeBuilder::default()
            .id(0)
            .kind(StateKind::Start)
            .style_property(StyleProperty::StrokeWidth(Unit::Pixel(2)))?
            .build(),
        Err(NodeError::InvalidStateKind)
    ));
    assert!(!StateNode::is_compatible_arrow_shape(ArrowShape::Circle));
    assert!(StateNode::is_compatible_arrow_shape(ArrowShape::Normal));
    Ok(())
}

#[test]
fn class_lookup_and_multiple_properties() -> Result {
    let mut builder = StateDiagramBuilder::default();
    let class = builder.style_class(
        StyleClassBuilder::default()
            .name("active")?
            .property(StyleProperty::Fill(Color::from((0, 0, 0))))?
            .property(StyleProperty::StrokeWidth(Unit::Pixel(2)))?,
    )?;
    assert_eq!(builder.get_style_class_by_name("active"), Some(class.clone()));
    assert!(builder.get_style_class_by_name("unknown").is_none());
    let node = StateNodeBuilder::default().label("Styled")?.style_class(class.clone())?;
    assert!(node.clone().style_class(class.clone()).is_err());
    builder.node(node)?;
    let diagram = StateDiagram::from(builder);
    assert_eq!(diagram.get_style_class_by_name("active"), Some(class));
    assert_eq!(
        diagram.to_string(),
        "stateDiagram-v2\n    direction LR\n    state \"Styled\" as S0\n    classDef Sclass_active fill: #000000,stroke-width: 2px\n    class S0 Sclass_active\n"
    );
    Ok(())
}

#[test]
fn nested_styles_are_hoisted_to_root() -> Result {
    let mut inner = StateDiagramBuilder::default();
    let class = inner.style_class(
        StyleClassBuilder::default()
            .name("active")?
            .property(StyleProperty::Fill(Color::from((0, 0, 0))))?,
    )?;
    inner.node(
        StateNodeBuilder::default()
            .label("Child")?
            .style_class(class)?
            .style_property(StyleProperty::StrokeWidth(Unit::Pixel(2)))?,
    )?;
    let mut outer = StateDiagramBuilder::default();
    outer.node(StateNodeBuilder::default().label("Parent")?.inner_diagram(inner.into())?)?;
    assert_eq!(
        StateDiagram::from(outer).to_string(),
        concat!(
            "stateDiagram-v2\n    direction LR\n",
            "    state \"Parent\" as S0\n    state S0 {\n        direction LR\n",
            "        state \"Child\" as S0_0\n    }\n",
            "    classDef S0_class_active fill: #000000\n",
            "    class S0_0 S0_class_active\n    style S0_0 stroke-width: 2px\n"
        )
    );
    Ok(())
}
