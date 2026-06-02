use rune_adapters::{AdapterCompatibilityCodes, ReviewPacketDocument, review_packet_adapter};
use rune_adopter::descriptor_collection;
use rune_core::{DocumentationPacketDocument, documentation_packet_profile};

#[test]
fn v1_workflow_emits_documentation_packet_evidence() {
    let collection = descriptor_collection().expect("crate-owned descriptor collection");
    let profile = documentation_packet_profile();
    let packet = DocumentationPacketDocument::from_collection(&profile, collection);
    let actual = serde_json::to_string_pretty(&packet).expect("serialize documentation packet");
    let expected = include_str!("fixtures/adopter_documentation_packet.json");

    assert_eq!(normalize_newlines(&actual), normalize_newlines(expected));
}

#[test]
fn v1_workflow_emits_review_packet_adapter_evidence() {
    let collection = descriptor_collection().expect("crate-owned descriptor collection");
    let adapter = review_packet_adapter();
    adapter
        .validate_collection_with_codes(
            &collection,
            AdapterCompatibilityCodes {
                unsupported_kind: "RUNE-ADAPT-003",
                unsupported_invariant: "RUNE-ADAPT-004",
                unsupported_trace_link: "RUNE-ADAPT-005",
                unsupported_extension: "RUNE-ADAPT-006",
            },
        )
        .expect("adapter-compatible adopter collection");
    let packet = ReviewPacketDocument::from_collection(&adapter, &collection);
    let actual = serde_json::to_string_pretty(&packet).expect("serialize review packet");
    let expected = include_str!("fixtures/adopter_review_packet.json");

    assert_eq!(normalize_newlines(&actual), normalize_newlines(expected));
}

fn normalize_newlines(value: &str) -> String {
    value.replace("\r\n", "\n").trim_end().to_owned()
}
