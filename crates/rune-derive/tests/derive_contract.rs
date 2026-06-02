use rune_core::{
    ContractKind, ContractRegistration, DescriptorCollectionDocument, DescriptorDocument,
    RuneContract,
};
use rune_derive::RuneContract as DeriveRuneContract;

const ANNOTATED_CUSTOMER_DESCRIPTOR_FIXTURE_FROM_MANIFEST: &str =
    "../rune-cli/tests/fixtures/annotated_customer_descriptor.json";
const ANNOTATED_CREATE_CUSTOMER_COMMAND_DESCRIPTOR_FIXTURE_FROM_MANIFEST: &str =
    "../rune-cli/tests/fixtures/annotated_create_customer_command_descriptor.json";
const ANNOTATED_CUSTOMER_CREATED_EVENT_DESCRIPTOR_FIXTURE_FROM_MANIFEST: &str =
    "../rune-cli/tests/fixtures/annotated_customer_created_event_descriptor.json";
const ANNOTATED_CUSTOMER_LIFECYCLE_STATE_DESCRIPTOR_FIXTURE_FROM_MANIFEST: &str =
    "../rune-cli/tests/fixtures/annotated_customer_lifecycle_state_descriptor.json";
const ANNOTATED_CONTRACT_EVIDENCE_ARTIFACT_DESCRIPTOR_FIXTURE_FROM_MANIFEST: &str =
    "../rune-cli/tests/fixtures/annotated_contract_evidence_artifact_descriptor.json";
const ANNOTATED_CONTRACT_SOURCE_REFERENCE_DESCRIPTOR_FIXTURE_FROM_MANIFEST: &str =
    "../rune-cli/tests/fixtures/annotated_contract_source_reference_descriptor.json";
const ANNOTATED_CONTRACT_VERIFICATION_EVIDENCE_DESCRIPTOR_FIXTURE_FROM_MANIFEST: &str =
    "../rune-cli/tests/fixtures/annotated_contract_verification_evidence_descriptor.json";
const KNOWN_CONTRACT_DESCRIPTOR_COLLECTION_FIXTURE_FROM_MANIFEST: &str =
    "../rune-cli/tests/fixtures/known_contract_descriptor_collection.json";

#[derive(DeriveRuneContract)]
#[rune(
    id = "example.customer",
    version = "v0",
    kind = "entity",
    requirement = "RUNE-REQ-034"
)]
#[allow(dead_code)]
struct Customer {
    id: String,
    email: String,
}

#[derive(DeriveRuneContract)]
#[rune(
    id = "example.create_customer",
    version = "v0",
    kind = "command",
    requirement = "RUNE-REQ-045"
)]
#[allow(dead_code)]
struct CreateCustomer {
    customer_id: String,
    email: String,
}

#[derive(DeriveRuneContract)]
#[rune(
    id = "example.customer_created",
    version = "v0",
    kind = "event",
    requirement = "RUNE-REQ-047"
)]
#[allow(dead_code)]
struct CustomerCreated {
    customer_id: String,
    occurred_at: String,
}

#[derive(DeriveRuneContract)]
#[rune(
    id = "example.customer_lifecycle",
    version = "v0",
    kind = "state",
    requirement = "RUNE-REQ-048"
)]
#[allow(dead_code)]
struct CustomerLifecycleState {
    customer_id: String,
    status: String,
    updated_at: String,
}

#[derive(DeriveRuneContract)]
#[rune(
    id = "example.contract_evidence",
    version = "v0",
    kind = "artifact",
    requirement = "RUNE-REQ-049"
)]
#[allow(dead_code)]
struct ContractEvidenceArtifact {
    descriptor_id: String,
    artifact_uri: String,
    artifact_kind: String,
}

#[derive(DeriveRuneContract)]
#[rune(
    id = "example.contract_source_reference",
    version = "v0",
    kind = "source",
    requirement = "RUNE-REQ-050"
)]
#[allow(dead_code)]
struct ContractSourceReference {
    descriptor_id: String,
    source_path: String,
    source_symbol: String,
}

#[derive(DeriveRuneContract)]
#[rune(
    id = "example.contract_verification_evidence",
    version = "v0",
    kind = "evidence",
    requirement = "RUNE-REQ-051"
)]
#[allow(dead_code)]
struct ContractVerificationEvidence {
    descriptor_id: String,
    verification_id: String,
    evidence_uri: String,
}

const KNOWN_CONTRACTS: &[ContractRegistration] = &[
    ContractRegistration {
        name: "Customer",
        descriptor: Customer::descriptor,
    },
    ContractRegistration {
        name: "CreateCustomer",
        descriptor: CreateCustomer::descriptor,
    },
    ContractRegistration {
        name: "CustomerCreated",
        descriptor: CustomerCreated::descriptor,
    },
    ContractRegistration {
        name: "CustomerLifecycleState",
        descriptor: CustomerLifecycleState::descriptor,
    },
    ContractRegistration {
        name: "ContractEvidenceArtifact",
        descriptor: ContractEvidenceArtifact::descriptor,
    },
    ContractRegistration {
        name: "ContractSourceReference",
        descriptor: ContractSourceReference::descriptor,
    },
    ContractRegistration {
        name: "ContractVerificationEvidence",
        descriptor: ContractVerificationEvidence::descriptor,
    },
];

#[test]
fn derive_emits_descriptor_shape() {
    let descriptor = Customer::descriptor();

    assert_eq!(descriptor.id, "example.customer");
    assert_eq!(descriptor.version, "v0");
    assert_eq!(descriptor.rust_type, "Customer");
    assert_eq!(descriptor.kind, ContractKind::Entity);
    assert_eq!(descriptor.fields.len(), 2);
    assert_eq!(descriptor.fields[0].name, "id");
    assert_eq!(descriptor.fields[1].name, "email");
    assert!(descriptor.invariants.is_empty());
    assert_eq!(descriptor.trace_links[0].relation, "requirement");
    assert_eq!(descriptor.trace_links[0].target, "RUNE-REQ-034");
    assert!(descriptor.extensions.is_empty());
}

#[test]
fn derive_emits_command_descriptor_shape() {
    let descriptor = CreateCustomer::descriptor();

    assert_eq!(descriptor.id, "example.create_customer");
    assert_eq!(descriptor.version, "v0");
    assert_eq!(descriptor.rust_type, "CreateCustomer");
    assert_eq!(descriptor.kind, ContractKind::Command);
    assert_eq!(descriptor.fields.len(), 2);
    assert_eq!(descriptor.fields[0].name, "customer_id");
    assert_eq!(descriptor.fields[1].name, "email");
    assert!(descriptor.invariants.is_empty());
    assert_eq!(descriptor.trace_links[0].relation, "requirement");
    assert_eq!(descriptor.trace_links[0].target, "RUNE-REQ-045");
    assert!(descriptor.extensions.is_empty());
}

#[test]
fn derive_emits_event_descriptor_shape() {
    let descriptor = CustomerCreated::descriptor();

    assert_eq!(descriptor.id, "example.customer_created");
    assert_eq!(descriptor.version, "v0");
    assert_eq!(descriptor.rust_type, "CustomerCreated");
    assert_eq!(descriptor.kind, ContractKind::Event);
    assert_eq!(descriptor.fields.len(), 2);
    assert_eq!(descriptor.fields[0].name, "customer_id");
    assert_eq!(descriptor.fields[1].name, "occurred_at");
    assert!(descriptor.invariants.is_empty());
    assert_eq!(descriptor.trace_links[0].relation, "requirement");
    assert_eq!(descriptor.trace_links[0].target, "RUNE-REQ-047");
    assert!(descriptor.extensions.is_empty());
}

#[test]
fn derive_emits_state_descriptor_shape() {
    let descriptor = CustomerLifecycleState::descriptor();

    assert_eq!(descriptor.id, "example.customer_lifecycle");
    assert_eq!(descriptor.version, "v0");
    assert_eq!(descriptor.rust_type, "CustomerLifecycleState");
    assert_eq!(descriptor.kind, ContractKind::State);
    assert_eq!(descriptor.fields.len(), 3);
    assert_eq!(descriptor.fields[0].name, "customer_id");
    assert_eq!(descriptor.fields[1].name, "status");
    assert_eq!(descriptor.fields[2].name, "updated_at");
    assert!(descriptor.invariants.is_empty());
    assert_eq!(descriptor.trace_links[0].relation, "requirement");
    assert_eq!(descriptor.trace_links[0].target, "RUNE-REQ-048");
    assert!(descriptor.extensions.is_empty());
}

#[test]
fn derive_emits_artifact_descriptor_shape() {
    let descriptor = ContractEvidenceArtifact::descriptor();

    assert_eq!(descriptor.id, "example.contract_evidence");
    assert_eq!(descriptor.version, "v0");
    assert_eq!(descriptor.rust_type, "ContractEvidenceArtifact");
    assert_eq!(descriptor.kind, ContractKind::Artifact);
    assert_eq!(descriptor.fields.len(), 3);
    assert_eq!(descriptor.fields[0].name, "descriptor_id");
    assert_eq!(descriptor.fields[1].name, "artifact_uri");
    assert_eq!(descriptor.fields[2].name, "artifact_kind");
    assert!(descriptor.invariants.is_empty());
    assert_eq!(descriptor.trace_links[0].relation, "requirement");
    assert_eq!(descriptor.trace_links[0].target, "RUNE-REQ-049");
    assert!(descriptor.extensions.is_empty());
}

#[test]
fn derive_emits_source_descriptor_shape() {
    let descriptor = ContractSourceReference::descriptor();

    assert_eq!(descriptor.id, "example.contract_source_reference");
    assert_eq!(descriptor.version, "v0");
    assert_eq!(descriptor.rust_type, "ContractSourceReference");
    assert_eq!(descriptor.kind, ContractKind::Source);
    assert_eq!(descriptor.fields.len(), 3);
    assert_eq!(descriptor.fields[0].name, "descriptor_id");
    assert_eq!(descriptor.fields[1].name, "source_path");
    assert_eq!(descriptor.fields[2].name, "source_symbol");
    assert!(descriptor.invariants.is_empty());
    assert_eq!(descriptor.trace_links[0].relation, "requirement");
    assert_eq!(descriptor.trace_links[0].target, "RUNE-REQ-050");
    assert!(descriptor.extensions.is_empty());
}

#[test]
fn derive_emits_evidence_descriptor_shape() {
    let descriptor = ContractVerificationEvidence::descriptor();

    assert_eq!(descriptor.id, "example.contract_verification_evidence");
    assert_eq!(descriptor.version, "v0");
    assert_eq!(descriptor.rust_type, "ContractVerificationEvidence");
    assert_eq!(descriptor.kind, ContractKind::Evidence);
    assert_eq!(descriptor.fields.len(), 3);
    assert_eq!(descriptor.fields[0].name, "descriptor_id");
    assert_eq!(descriptor.fields[1].name, "verification_id");
    assert_eq!(descriptor.fields[2].name, "evidence_uri");
    assert!(descriptor.invariants.is_empty());
    assert_eq!(descriptor.trace_links[0].relation, "requirement");
    assert_eq!(descriptor.trace_links[0].target, "RUNE-REQ-051");
    assert!(descriptor.extensions.is_empty());
}

#[test]
fn derive_document_matches_retained_descriptor_fixture() {
    let actual = customer_descriptor_json();
    let expected = include_str!("../../rune-cli/tests/fixtures/annotated_customer_descriptor.json");

    assert_eq!(normalize_newlines(&actual), normalize_newlines(expected));
}

#[test]
fn derive_command_document_matches_retained_descriptor_fixture() {
    let actual = create_customer_command_descriptor_json();
    let expected = include_str!(
        "../../rune-cli/tests/fixtures/annotated_create_customer_command_descriptor.json"
    );

    assert_eq!(normalize_newlines(&actual), normalize_newlines(expected));
}

#[test]
fn derive_event_document_matches_retained_descriptor_fixture() {
    let actual = customer_created_event_descriptor_json();
    let expected = include_str!(
        "../../rune-cli/tests/fixtures/annotated_customer_created_event_descriptor.json"
    );

    assert_eq!(normalize_newlines(&actual), normalize_newlines(expected));
}

#[test]
fn derive_state_document_matches_retained_descriptor_fixture() {
    let actual = customer_lifecycle_state_descriptor_json();
    let expected = include_str!(
        "../../rune-cli/tests/fixtures/annotated_customer_lifecycle_state_descriptor.json"
    );

    assert_eq!(normalize_newlines(&actual), normalize_newlines(expected));
}

#[test]
fn derive_artifact_document_matches_retained_descriptor_fixture() {
    let actual = contract_evidence_artifact_descriptor_json();
    let expected = include_str!(
        "../../rune-cli/tests/fixtures/annotated_contract_evidence_artifact_descriptor.json"
    );

    assert_eq!(normalize_newlines(&actual), normalize_newlines(expected));
}

#[test]
fn derive_source_document_matches_retained_descriptor_fixture() {
    let actual = contract_source_reference_descriptor_json();
    let expected = include_str!(
        "../../rune-cli/tests/fixtures/annotated_contract_source_reference_descriptor.json"
    );

    assert_eq!(normalize_newlines(&actual), normalize_newlines(expected));
}

#[test]
fn derive_evidence_document_matches_retained_descriptor_fixture() {
    let actual = contract_verification_evidence_descriptor_json();
    let expected = include_str!(
        "../../rune-cli/tests/fixtures/annotated_contract_verification_evidence_descriptor.json"
    );

    assert_eq!(normalize_newlines(&actual), normalize_newlines(expected));
}

#[test]
fn derive_document_serialization_is_deterministic() {
    assert_eq!(customer_descriptor_json(), customer_descriptor_json());
    assert_eq!(
        create_customer_command_descriptor_json(),
        create_customer_command_descriptor_json()
    );
    assert_eq!(
        customer_created_event_descriptor_json(),
        customer_created_event_descriptor_json()
    );
    assert_eq!(
        customer_lifecycle_state_descriptor_json(),
        customer_lifecycle_state_descriptor_json()
    );
    assert_eq!(
        contract_evidence_artifact_descriptor_json(),
        contract_evidence_artifact_descriptor_json()
    );
    assert_eq!(
        contract_source_reference_descriptor_json(),
        contract_source_reference_descriptor_json()
    );
    assert_eq!(
        contract_verification_evidence_descriptor_json(),
        contract_verification_evidence_descriptor_json()
    );
    assert_eq!(
        known_contract_descriptor_collection_json(),
        known_contract_descriptor_collection_json()
    );
}

#[test]
fn known_contract_collection_matches_retained_fixture() {
    let actual = known_contract_descriptor_collection_json();
    let expected =
        include_str!("../../rune-cli/tests/fixtures/known_contract_descriptor_collection.json");

    assert_eq!(normalize_newlines(&actual), normalize_newlines(expected));
}

#[test]
fn update_retained_customer_descriptor_when_requested() {
    if std::env::var_os("RUNE_UPDATE_EVIDENCE").is_none() {
        return;
    }

    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(ANNOTATED_CUSTOMER_DESCRIPTOR_FIXTURE_FROM_MANIFEST);
    std::fs::write(fixture, format!("{}\n", customer_descriptor_json()))
        .expect("write retained customer descriptor evidence");
}

#[test]
fn update_retained_known_contract_descriptors_when_requested() {
    if std::env::var_os("RUNE_UPDATE_EVIDENCE").is_none() {
        return;
    }

    write_fixture(
        ANNOTATED_CUSTOMER_DESCRIPTOR_FIXTURE_FROM_MANIFEST,
        &customer_descriptor_json(),
    );
    write_fixture(
        ANNOTATED_CREATE_CUSTOMER_COMMAND_DESCRIPTOR_FIXTURE_FROM_MANIFEST,
        &create_customer_command_descriptor_json(),
    );
    write_fixture(
        ANNOTATED_CUSTOMER_CREATED_EVENT_DESCRIPTOR_FIXTURE_FROM_MANIFEST,
        &customer_created_event_descriptor_json(),
    );
    write_fixture(
        ANNOTATED_CUSTOMER_LIFECYCLE_STATE_DESCRIPTOR_FIXTURE_FROM_MANIFEST,
        &customer_lifecycle_state_descriptor_json(),
    );
    write_fixture(
        ANNOTATED_CONTRACT_EVIDENCE_ARTIFACT_DESCRIPTOR_FIXTURE_FROM_MANIFEST,
        &contract_evidence_artifact_descriptor_json(),
    );
    write_fixture(
        ANNOTATED_CONTRACT_SOURCE_REFERENCE_DESCRIPTOR_FIXTURE_FROM_MANIFEST,
        &contract_source_reference_descriptor_json(),
    );
    write_fixture(
        ANNOTATED_CONTRACT_VERIFICATION_EVIDENCE_DESCRIPTOR_FIXTURE_FROM_MANIFEST,
        &contract_verification_evidence_descriptor_json(),
    );
    write_fixture(
        KNOWN_CONTRACT_DESCRIPTOR_COLLECTION_FIXTURE_FROM_MANIFEST,
        &known_contract_descriptor_collection_json(),
    );
}

fn customer_descriptor_json() -> String {
    let document = DescriptorDocument::from_contract::<Customer>();
    serde_json::to_string_pretty(&document).expect("serialize descriptor document")
}

fn create_customer_command_descriptor_json() -> String {
    let document = DescriptorDocument::from_contract::<CreateCustomer>();
    serde_json::to_string_pretty(&document).expect("serialize descriptor document")
}

fn customer_created_event_descriptor_json() -> String {
    let document = DescriptorDocument::from_contract::<CustomerCreated>();
    serde_json::to_string_pretty(&document).expect("serialize descriptor document")
}

fn customer_lifecycle_state_descriptor_json() -> String {
    let document = DescriptorDocument::from_contract::<CustomerLifecycleState>();
    serde_json::to_string_pretty(&document).expect("serialize descriptor document")
}

fn contract_evidence_artifact_descriptor_json() -> String {
    let document = DescriptorDocument::from_contract::<ContractEvidenceArtifact>();
    serde_json::to_string_pretty(&document).expect("serialize descriptor document")
}

fn contract_source_reference_descriptor_json() -> String {
    let document = DescriptorDocument::from_contract::<ContractSourceReference>();
    serde_json::to_string_pretty(&document).expect("serialize descriptor document")
}

fn contract_verification_evidence_descriptor_json() -> String {
    let document = DescriptorDocument::from_contract::<ContractVerificationEvidence>();
    serde_json::to_string_pretty(&document).expect("serialize descriptor document")
}

fn known_contract_descriptor_collection_json() -> String {
    let collection = DescriptorCollectionDocument::from_registrations(
        "example.known_contracts",
        "v0",
        KNOWN_CONTRACTS,
        "RUNE-REG-001",
    )
    .expect("collect contracts");
    serde_json::to_string_pretty(&collection).expect("serialize descriptor document collection")
}

fn write_fixture(path_from_manifest: &str, content: &str) {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path_from_manifest);
    std::fs::write(fixture, format!("{content}\n")).expect("write retained descriptor evidence");
}

fn normalize_newlines(value: &str) -> String {
    value.replace("\r\n", "\n").trim_end().to_owned()
}
