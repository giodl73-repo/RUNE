use rune_adapters::{AdapterCompatibilityCodes, review_packet_adapter};
use rune_core::{
    DataContractDocument, DocumentationPacketDocument, data_contract_profile,
    documentation_packet_profile,
};
use rune_shape_calculator::{
    COLLECTION_ID, COLLECTION_VERSION, Circle, Rectangle, circle_metrics, descriptor_collection,
    rectangle_metrics,
};

#[test]
fn calculator_computes_shape_metrics() {
    let circle = circle_metrics(Circle { radius: 2.0 });
    assert!((circle.area - 12.566370614359172).abs() < f64::EPSILON);
    assert!((circle.perimeter - 12.566370614359172).abs() < f64::EPSILON);

    let rectangle = rectangle_metrics(Rectangle {
        width: 3.0,
        height: 4.0,
    });
    assert_eq!(rectangle.area, 12.0);
    assert_eq!(rectangle.perimeter, 14.0);
}

#[test]
fn shape_registry_preserves_metadata_driven_contracts() {
    let collection = descriptor_collection().expect("shape descriptor collection");

    assert_eq!(collection.collection_id, COLLECTION_ID);
    assert_eq!(collection.collection_version, COLLECTION_VERSION);
    assert_eq!(collection.descriptors[0].id, "example.shape.circle");
    assert_eq!(collection.descriptors[0].invariants.len(), 1);
    assert_eq!(collection.descriptors[0].extensions.len(), 2);
    assert_eq!(
        collection.descriptors[0].extensions[0].value,
        "pi * radius^2"
    );
}

#[test]
fn shape_collection_matches_retained_evidence() {
    let collection = descriptor_collection().expect("shape descriptor collection");
    let actual = serde_json::to_string_pretty(&collection).expect("serialize collection");
    let expected = include_str!("fixtures/shape_contract_collection.json");

    assert_eq!(normalize_newlines(&actual), normalize_newlines(expected));
}

#[test]
fn shape_documentation_packet_matches_retained_evidence() {
    let collection = descriptor_collection().expect("shape descriptor collection");
    let profile = documentation_packet_profile();
    let packet = DocumentationPacketDocument::from_collection(&profile, collection);
    let actual = serde_json::to_string_pretty(&packet).expect("serialize documentation packet");
    let expected = include_str!("fixtures/shape_documentation_packet.json");

    assert_eq!(normalize_newlines(&actual), normalize_newlines(expected));
}

#[test]
fn shape_data_contract_profile_matches_retained_evidence() {
    let collection = descriptor_collection().expect("shape descriptor collection");
    let profile = data_contract_profile();
    let packet = DataContractDocument::from_collection(&profile, collection);
    let actual = serde_json::to_string_pretty(&packet).expect("serialize data contract profile");
    let expected = include_str!("fixtures/shape_data_contract_profile.json");

    assert_eq!(normalize_newlines(&actual), normalize_newlines(expected));
}

#[test]
fn review_packet_adapter_rejects_unrepresented_shape_metadata() {
    let collection = descriptor_collection().expect("shape descriptor collection");
    let adapter = review_packet_adapter();
    let error = adapter
        .validate_collection_with_codes(
            &collection,
            AdapterCompatibilityCodes {
                unsupported_kind: "RUNE-ADAPT-003",
                unsupported_invariant: "RUNE-ADAPT-004",
                unsupported_trace_link: "RUNE-ADAPT-005",
                unsupported_extension: "RUNE-ADAPT-006",
            },
        )
        .expect_err("review packet adapter should reject invariants first");

    assert!(error.contains("RUNE-ADAPT-004"));
}

fn normalize_newlines(value: &str) -> String {
    value.replace("\r\n", "\n").trim_end().to_owned()
}
