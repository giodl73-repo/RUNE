//! Downstream adapter surfaces for validated RUNE evidence.

use rune_core::DescriptorCollectionDocument;
use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AdapterCatalog {
    pub adapters: Vec<AdapterDescriptor>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AdapterDescriptor {
    pub adapter_id: String,
    pub adapter_version: String,
    pub accepted_input_artifact_kinds: Vec<String>,
    pub supported_kinds: Vec<String>,
    pub supported_concepts: Vec<String>,
    pub unsupported_policy: String,
    pub output_artifact_kind: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReviewPacketDocument {
    pub adapter_id: String,
    pub adapter_version: String,
    pub output_artifact_kind: String,
    pub source_collection_id: String,
    pub source_collection_version: String,
    pub item_count: usize,
    pub items: Vec<ReviewPacketItem>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReviewPacketItem {
    pub descriptor_id: String,
    pub descriptor_version: String,
    pub descriptor_kind: String,
    pub rust_type: String,
    pub field_count: usize,
    pub trace_link_count: usize,
    pub review_prompt: String,
}

pub struct AdapterCompatibilityCodes {
    pub unsupported_kind: &'static str,
    pub unsupported_invariant: &'static str,
    pub unsupported_trace_link: &'static str,
    pub unsupported_extension: &'static str,
}

impl AdapterCatalog {
    pub fn approved() -> Self {
        Self {
            adapters: vec![review_packet_adapter()],
        }
    }

    pub fn find(&self, adapter_id: &str) -> Option<&AdapterDescriptor> {
        self.adapters
            .iter()
            .find(|adapter| adapter.adapter_id == adapter_id)
    }
}

impl AdapterDescriptor {
    pub fn validate_collection_with_codes(
        &self,
        collection: &DescriptorCollectionDocument,
        codes: AdapterCompatibilityCodes,
    ) -> Result<(), String> {
        for descriptor in &collection.descriptors {
            if !self
                .supported_kinds
                .iter()
                .any(|kind| kind == &descriptor.kind)
            {
                return Err(format!(
                    "{} adapter {} does not support descriptor kind: {}",
                    codes.unsupported_kind, self.adapter_id, descriptor.kind
                ));
            }

            if !descriptor.invariants.is_empty() && !self.supports_concept("invariants") {
                return Err(format!(
                    "{} adapter {} cannot represent invariants",
                    codes.unsupported_invariant, self.adapter_id
                ));
            }

            if !descriptor.trace_links.is_empty() && !self.supports_concept("trace_links") {
                return Err(format!(
                    "{} adapter {} cannot represent trace links",
                    codes.unsupported_trace_link, self.adapter_id
                ));
            }

            if !descriptor.extensions.is_empty() && !self.supports_concept("extensions") {
                return Err(format!(
                    "{} adapter {} cannot represent extensions",
                    codes.unsupported_extension, self.adapter_id
                ));
            }
        }

        Ok(())
    }

    fn supports_concept(&self, concept: &str) -> bool {
        self.supported_concepts
            .iter()
            .any(|supported| supported == concept)
    }
}

impl ReviewPacketDocument {
    pub fn from_collection(
        adapter: &AdapterDescriptor,
        collection: &DescriptorCollectionDocument,
    ) -> Self {
        Self {
            adapter_id: adapter.adapter_id.clone(),
            adapter_version: adapter.adapter_version.clone(),
            output_artifact_kind: adapter.output_artifact_kind.clone(),
            source_collection_id: collection.collection_id.clone(),
            source_collection_version: collection.collection_version.clone(),
            item_count: collection.descriptors.len(),
            items: collection
                .descriptors
                .iter()
                .map(|descriptor| ReviewPacketItem {
                    descriptor_id: descriptor.id.clone(),
                    descriptor_version: descriptor.version.clone(),
                    descriptor_kind: descriptor.kind.clone(),
                    rust_type: descriptor.rust_type.clone(),
                    field_count: descriptor.fields.len(),
                    trace_link_count: descriptor.trace_links.len(),
                    review_prompt: format!(
                        "Review {} {} for identity, version, fields, traceability, and adapter readiness.",
                        descriptor.kind, descriptor.id
                    ),
                })
                .collect(),
        }
    }
}

pub fn review_packet_adapter() -> AdapterDescriptor {
    AdapterDescriptor {
        adapter_id: "rune.review_packet_json".to_owned(),
        adapter_version: "v0".to_owned(),
        accepted_input_artifact_kinds: vec!["rune.descriptor_collection.json".to_owned()],
        supported_kinds: vec![
            "entity".to_owned(),
            "event".to_owned(),
            "command".to_owned(),
            "state".to_owned(),
            "artifact".to_owned(),
            "source".to_owned(),
            "evidence".to_owned(),
        ],
        supported_concepts: vec!["fields".to_owned(), "trace_links".to_owned()],
        unsupported_policy: "error".to_owned(),
        output_artifact_kind: "rune.review_packet.json".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rune_core::{DescriptorDocument, FieldDocument, FieldMetadataDocument, TraceLinkDocument};

    #[test]
    fn approved_catalog_exposes_review_packet_adapter() {
        let catalog = AdapterCatalog::approved();
        let adapter = catalog
            .find("rune.review_packet_json")
            .expect("review packet adapter");

        assert_eq!(adapter.adapter_version, "v0");
        assert_eq!(adapter.output_artifact_kind, "rune.review_packet.json");
        assert!(adapter.supported_kinds.iter().any(|kind| kind == "entity"));
    }

    #[test]
    fn review_packet_preserves_collection_review_items() {
        let adapter = review_packet_adapter();
        let collection = DescriptorCollectionDocument {
            collection_id: "example.known_contracts".to_owned(),
            collection_version: "v0".to_owned(),
            descriptors: vec![descriptor("example.customer", "entity")],
        };

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
            .expect("adapter-compatible collection");
        let packet = ReviewPacketDocument::from_collection(&adapter, &collection);

        assert_eq!(packet.source_collection_id, "example.known_contracts");
        assert_eq!(packet.item_count, 1);
        assert_eq!(packet.items[0].descriptor_id, "example.customer");
        assert_eq!(packet.items[0].trace_link_count, 1);
    }

    #[test]
    fn adapter_rejects_unsupported_descriptor_kind() {
        let adapter = review_packet_adapter();
        let collection = DescriptorCollectionDocument {
            collection_id: "example.known_contracts".to_owned(),
            collection_version: "v0".to_owned(),
            descriptors: vec![descriptor("example.other", "other")],
        };

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
            .expect_err("unsupported kind should fail");

        assert!(error.contains("RUNE-ADAPT-003"));
    }

    fn descriptor(id: &str, kind: &str) -> DescriptorDocument {
        DescriptorDocument {
            id: id.to_owned(),
            version: "v0".to_owned(),
            kind: kind.to_owned(),
            rust_type: "Example".to_owned(),
            fields: vec![FieldDocument {
                name: "id".to_owned(),
                rust_type: "String".to_owned(),
                metadata: FieldMetadataDocument::default(),
            }],
            invariants: Vec::new(),
            trace_links: vec![TraceLinkDocument {
                relation: "requirement".to_owned(),
                target: "RUNE-REQ-066".to_owned(),
            }],
            extensions: Vec::new(),
        }
    }
}
