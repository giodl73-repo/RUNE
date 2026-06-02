use rune_adopter::{COLLECTION_ID, COLLECTION_VERSION, RUNE_CONTRACTS, descriptor_collection};
use rune_core::{DescriptorCollectionDocument, collect_known_contract_documents};

#[test]
fn crate_owned_registry_preserves_order_and_metadata() {
    let collection = descriptor_collection().expect("crate-owned descriptor collection");

    assert_eq!(collection.collection_id, COLLECTION_ID);
    assert_eq!(collection.collection_version, COLLECTION_VERSION);
    assert_eq!(collection.descriptors[0].id, "example.adopter.account");
    assert_eq!(collection.descriptors[1].id, "example.adopter.open_account");
}

#[test]
fn crate_owned_registry_matches_retained_collection_evidence() {
    let collection = descriptor_collection().expect("crate-owned descriptor collection");
    let expected: DescriptorCollectionDocument =
        serde_json::from_str(include_str!("fixtures/adopter_contract_collection.json"))
            .expect("retained collection fixture");

    assert_eq!(collection, expected);
}

#[test]
fn duplicate_registry_entries_fail_closed() {
    let duplicate_registrations = [RUNE_CONTRACTS[0], RUNE_CONTRACTS[0]];

    let error = collect_known_contract_documents(&duplicate_registrations, "RUNE-REG-001")
        .expect_err("duplicate descriptor ids should fail");

    assert!(error.contains("RUNE-REG-001"));
    assert!(error.contains("duplicate contract id"));
}
