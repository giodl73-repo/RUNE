//! Neutral RUNE contract descriptors.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContractKind {
    Entity,
    Event,
    Command,
    State,
    Artifact,
    Evidence,
    Source,
    Other(&'static str),
}

impl ContractKind {
    pub fn as_str(self) -> &'static str {
        match self {
            ContractKind::Entity => "entity",
            ContractKind::Event => "event",
            ContractKind::Command => "command",
            ContractKind::State => "state",
            ContractKind::Artifact => "artifact",
            ContractKind::Evidence => "evidence",
            ContractKind::Source => "source",
            ContractKind::Other(value) => value,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldDescriptor {
    pub name: &'static str,
    pub rust_type: &'static str,
    pub metadata: FieldMetadataDescriptor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldMetadataDescriptor {
    pub required: Option<bool>,
    pub unit: Option<&'static str>,
    pub min: Option<&'static str>,
    pub max: Option<&'static str>,
    pub sensitivity: Option<&'static str>,
    pub example: Option<&'static str>,
    pub stability: Option<&'static str>,
    pub aliases: &'static [&'static str],
}

impl FieldMetadataDescriptor {
    pub const EMPTY: Self = Self {
        required: None,
        unit: None,
        min: None,
        max: None,
        sensitivity: None,
        example: None,
        stability: None,
        aliases: &[],
    };
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvariantDescriptor {
    pub id: &'static str,
    pub text: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TraceLink {
    pub relation: &'static str,
    pub target: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExtensionDescriptor {
    pub namespace: &'static str,
    pub name: &'static str,
    pub value: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ContractDescriptor {
    pub id: &'static str,
    pub version: &'static str,
    pub rust_type: &'static str,
    pub kind: ContractKind,
    pub fields: &'static [FieldDescriptor],
    pub invariants: &'static [InvariantDescriptor],
    pub trace_links: &'static [TraceLink],
    pub extensions: &'static [ExtensionDescriptor],
}

pub trait RuneContract {
    fn descriptor() -> ContractDescriptor;
}

#[derive(Clone, Copy)]
pub struct ContractRegistration {
    pub name: &'static str,
    pub descriptor: fn() -> ContractDescriptor,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DescriptorDocument {
    pub id: String,
    pub version: String,
    pub kind: String,
    pub rust_type: String,
    pub fields: Vec<FieldDocument>,
    pub invariants: Vec<InvariantDocument>,
    pub trace_links: Vec<TraceLinkDocument>,
    pub extensions: Vec<ExtensionDocument>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DescriptorCollectionDocument {
    pub collection_id: String,
    pub collection_version: String,
    pub descriptors: Vec<DescriptorDocument>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DescriptorCollectionInventoryDocument {
    pub collection_id: String,
    pub collection_version: String,
    pub descriptor_count: usize,
    pub kinds: Vec<DescriptorKindInventoryDocument>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DescriptorKindInventoryDocument {
    pub kind: String,
    pub descriptor_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CollectionEvidenceBundleDocument {
    pub source_kind: String,
    pub source: String,
    pub profile_id: String,
    pub profile_version: String,
    pub collection: DescriptorCollectionDocument,
    pub check_report: CheckCollectionReportDocument,
    pub inventory: DescriptorCollectionInventoryDocument,
    pub generated_artifact: GeneratedCollectionArtifactDocument,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DocumentationPacketDocument {
    pub profile_id: String,
    pub profile_version: String,
    pub output_artifact_kind: String,
    pub source_kind: String,
    pub source_id: String,
    pub source_version: String,
    pub descriptor_count: usize,
    pub descriptors: Vec<DocumentationDescriptorSummary>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DocumentationDescriptorSummary {
    pub id: String,
    pub version: String,
    pub kind: String,
    pub rust_type: String,
    pub field_count: usize,
    pub fields: Vec<FieldDocument>,
    pub invariant_count: usize,
    pub trace_link_count: usize,
    pub trace_links: Vec<TraceLinkDocument>,
    pub extension_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DataContractDocument {
    pub profile_id: String,
    pub profile_version: String,
    pub output_artifact_kind: String,
    pub source_kind: String,
    pub source_id: String,
    pub source_version: String,
    pub contract_count: usize,
    pub contracts: Vec<DataContractSummary>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DataContractSummary {
    pub id: String,
    pub version: String,
    pub kind: String,
    pub rust_type: String,
    pub fields: Vec<FieldDocument>,
    pub invariants: Vec<InvariantDocument>,
    pub trace_links: Vec<TraceLinkDocument>,
    pub extensions: Vec<ExtensionDocument>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct DiscoveryManifestDraft {
    pub manifest_id: Option<String>,
    pub manifest_version: Option<String>,
    pub collection_id: Option<String>,
    pub collection_version: Option<String>,
    #[serde(default)]
    pub entries: Vec<DiscoveryManifestEntry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DiscoveryManifestDocument {
    pub manifest_id: String,
    pub manifest_version: String,
    pub collection_id: String,
    pub collection_version: String,
    pub entries: Vec<DiscoveryManifestEntry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DiscoveryManifestEntry {
    pub name: String,
    pub source_kind: String,
    pub source: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct SemanticRegistryDraft {
    pub registry_id: Option<String>,
    pub registry_version: Option<String>,
    pub owner: Option<String>,
    pub scope: Option<String>,
    #[serde(default)]
    pub collections: Vec<SemanticRegistryCollectionRef>,
    #[serde(default)]
    pub profiles: Vec<SemanticRegistryProfileRef>,
    #[serde(default)]
    pub adapters: Vec<SemanticRegistryAdapterRef>,
    #[serde(default)]
    pub capabilities: SemanticRegistryCapabilities,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticRegistryDocument {
    pub registry_id: String,
    pub registry_version: String,
    pub owner: String,
    pub scope: String,
    pub collections: Vec<SemanticRegistryCollectionRef>,
    pub profiles: Vec<SemanticRegistryProfileRef>,
    pub adapters: Vec<SemanticRegistryAdapterRef>,
    pub capabilities: SemanticRegistryCapabilities,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticRegistryCollectionRef {
    pub collection_id: String,
    pub collection_version: String,
    pub source_ref: String,
    pub owner: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticRegistryProfileRef {
    pub profile_id: String,
    pub profile_version: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticRegistryAdapterRef {
    pub adapter_id: String,
    pub adapter_version: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticRegistryCapabilities {
    #[serde(default)]
    pub read: bool,
    #[serde(default)]
    pub query: bool,
    #[serde(default)]
    pub generate: bool,
    #[serde(default)]
    pub mutate: bool,
    #[serde(default)]
    pub runtime: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct StateGraphDraft {
    pub state_graph_id: Option<String>,
    pub state_graph_version: Option<String>,
    pub registry_ref: Option<StateGraphRegistryRef>,
    #[serde(default)]
    pub nodes: Vec<StateGraphNode>,
    #[serde(default)]
    pub transitions: Vec<StateGraphTransition>,
    #[serde(default)]
    pub ownership: Vec<StateGraphOwnership>,
    #[serde(default)]
    pub evidence_refs: Vec<StateGraphEvidenceRef>,
    #[serde(default)]
    pub capabilities: StateGraphCapabilities,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StateGraphDocument {
    pub state_graph_id: String,
    pub state_graph_version: String,
    pub registry_ref: StateGraphRegistryRef,
    pub nodes: Vec<StateGraphNode>,
    pub transitions: Vec<StateGraphTransition>,
    pub ownership: Vec<StateGraphOwnership>,
    pub evidence_refs: Vec<StateGraphEvidenceRef>,
    pub capabilities: StateGraphCapabilities,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StateGraphRegistryRef {
    pub registry_id: String,
    pub registry_version: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StateGraphNode {
    pub node_id: String,
    pub descriptor_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StateGraphTransition {
    pub transition_id: String,
    pub from_node_id: String,
    pub to_node_id: String,
    pub descriptor_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StateGraphOwnership {
    pub owner: String,
    pub node_ids: Vec<String>,
    pub transition_ids: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StateGraphEvidenceRef {
    pub evidence_kind: String,
    pub source_ref: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct StateGraphCapabilities {
    #[serde(default)]
    pub retained: bool,
    #[serde(default)]
    pub live_state: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateGraphValidationCodes {
    pub missing_identity: &'static str,
    pub missing_registry_ref: &'static str,
    pub unknown_node_descriptor: &'static str,
    pub unknown_transition_node: &'static str,
    pub unsupported_transition_descriptor: &'static str,
    pub live_state_requested: &'static str,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct DescriptorCollectionDraft {
    pub collection_id: Option<String>,
    pub collection_version: Option<String>,
    #[serde(default)]
    pub descriptors: Vec<DescriptorDocument>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct DescriptorDraft {
    pub id: Option<String>,
    pub version: Option<String>,
    pub kind: Option<String>,
    pub rust_type: Option<String>,
    #[serde(default)]
    pub fields: Vec<FieldDocument>,
    #[serde(default)]
    pub invariants: Vec<InvariantDocument>,
    #[serde(default)]
    pub trace_links: Vec<TraceLinkDocument>,
    #[serde(default)]
    pub extensions: Vec<ExtensionDocument>,
}

impl DescriptorDraft {
    pub fn validate_with_codes(
        self,
        missing_id_code: &'static str,
        missing_version_code: &'static str,
        kind_code: &'static str,
    ) -> Result<DescriptorDocument, String> {
        let id = required_non_empty(self.id, missing_id_code, "descriptor identity is missing")?;
        let version = required_non_empty(
            self.version,
            missing_version_code,
            "descriptor version is missing",
        )?;
        let kind = required_non_empty(self.kind, kind_code, "descriptor kind is missing")?;
        if !is_supported_kind(&kind) {
            return Err(format!("{kind_code} unsupported descriptor kind: {kind}"));
        }

        Ok(DescriptorDocument {
            id,
            version,
            kind,
            rust_type: self.rust_type.unwrap_or_default(),
            fields: self.fields,
            invariants: self.invariants,
            trace_links: self.trace_links,
            extensions: self.extensions,
        })
    }
}

impl DescriptorDocument {
    pub fn from_contract<T: RuneContract>() -> Self {
        Self::from_descriptor(T::descriptor())
    }

    pub fn from_descriptor(descriptor: ContractDescriptor) -> Self {
        Self {
            id: descriptor.id.to_owned(),
            version: descriptor.version.to_owned(),
            kind: descriptor.kind.as_str().to_owned(),
            rust_type: descriptor.rust_type.to_owned(),
            fields: descriptor
                .fields
                .iter()
                .map(|field| FieldDocument {
                    name: field.name.to_owned(),
                    rust_type: field.rust_type.to_owned(),
                    metadata: FieldMetadataDocument {
                        required: field.metadata.required,
                        unit: field.metadata.unit.map(str::to_owned),
                        min: field.metadata.min.map(str::to_owned),
                        max: field.metadata.max.map(str::to_owned),
                        sensitivity: field.metadata.sensitivity.map(str::to_owned),
                        example: field.metadata.example.map(str::to_owned),
                        stability: field.metadata.stability.map(str::to_owned),
                        aliases: field
                            .metadata
                            .aliases
                            .iter()
                            .map(|alias| (*alias).to_owned())
                            .collect(),
                    },
                })
                .collect(),
            invariants: descriptor
                .invariants
                .iter()
                .map(|invariant| InvariantDocument {
                    id: invariant.id.to_owned(),
                    text: invariant.text.to_owned(),
                })
                .collect(),
            trace_links: descriptor
                .trace_links
                .iter()
                .map(|trace_link| TraceLinkDocument {
                    relation: trace_link.relation.to_owned(),
                    target: trace_link.target.to_owned(),
                })
                .collect(),
            extensions: descriptor
                .extensions
                .iter()
                .map(|extension| ExtensionDocument {
                    namespace: extension.namespace.to_owned(),
                    name: extension.name.to_owned(),
                    value: extension.value.to_owned(),
                })
                .collect(),
        }
    }
}

impl DescriptorCollectionDocument {
    pub fn from_registrations(
        collection_id: impl Into<String>,
        collection_version: impl Into<String>,
        registrations: &[ContractRegistration],
        duplicate_id_code: &'static str,
    ) -> Result<Self, String> {
        Ok(Self {
            collection_id: collection_id.into(),
            collection_version: collection_version.into(),
            descriptors: collect_known_contract_documents(registrations, duplicate_id_code)?,
        })
    }

    pub fn inventory(&self) -> DescriptorCollectionInventoryDocument {
        let mut kind_counts = BTreeMap::new();
        for descriptor in &self.descriptors {
            *kind_counts.entry(descriptor.kind.clone()).or_insert(0) += 1;
        }

        DescriptorCollectionInventoryDocument {
            collection_id: self.collection_id.clone(),
            collection_version: self.collection_version.clone(),
            descriptor_count: self.descriptors.len(),
            kinds: kind_counts
                .into_iter()
                .map(|(kind, descriptor_count)| DescriptorKindInventoryDocument {
                    kind,
                    descriptor_count,
                })
                .collect(),
        }
    }

    pub fn from_discovered_collections(
        manifest: &DiscoveryManifestDocument,
        collections: &[DescriptorCollectionDocument],
        duplicate_id_code: &'static str,
    ) -> Result<Self, String> {
        let mut descriptors = Vec::new();

        for collection in collections {
            for descriptor in &collection.descriptors {
                if descriptors
                    .iter()
                    .any(|existing: &DescriptorDocument| existing.id == descriptor.id)
                {
                    return Err(format!(
                        "{duplicate_id_code} duplicate descriptor id in discovered collection: {}",
                        descriptor.id
                    ));
                }
                descriptors.push(descriptor.clone());
            }
        }

        Ok(Self {
            collection_id: manifest.collection_id.clone(),
            collection_version: manifest.collection_version.clone(),
            descriptors,
        })
    }
}

impl DescriptorCollectionDraft {
    pub fn validate_with_codes(
        self,
        missing_collection_id_code: &'static str,
        missing_collection_version_code: &'static str,
        duplicate_descriptor_id_code: &'static str,
    ) -> Result<DescriptorCollectionDocument, String> {
        let collection_id = required_non_empty(
            self.collection_id,
            missing_collection_id_code,
            "descriptor collection identity is missing",
        )?;
        let collection_version = required_non_empty(
            self.collection_version,
            missing_collection_version_code,
            "descriptor collection version is missing",
        )?;

        for (index, descriptor) in self.descriptors.iter().enumerate() {
            if self
                .descriptors
                .iter()
                .skip(index + 1)
                .any(|candidate| candidate.id == descriptor.id)
            {
                return Err(format!(
                    "{duplicate_descriptor_id_code} duplicate descriptor id in collection: {}",
                    descriptor.id
                ));
            }
        }

        Ok(DescriptorCollectionDocument {
            collection_id,
            collection_version,
            descriptors: self.descriptors,
        })
    }
}

impl DiscoveryManifestDraft {
    pub fn validate_with_codes(
        self,
        missing_manifest_id_code: &'static str,
        missing_manifest_version_code: &'static str,
        missing_collection_id_code: &'static str,
        missing_collection_version_code: &'static str,
    ) -> Result<DiscoveryManifestDocument, String> {
        let manifest_id = required_non_empty(
            self.manifest_id,
            missing_manifest_id_code,
            "discovery manifest identity is missing",
        )?;
        let manifest_version = required_non_empty(
            self.manifest_version,
            missing_manifest_version_code,
            "discovery manifest version is missing",
        )?;
        let collection_id = required_non_empty(
            self.collection_id,
            missing_collection_id_code,
            "discovery output collection identity is missing",
        )?;
        let collection_version = required_non_empty(
            self.collection_version,
            missing_collection_version_code,
            "discovery output collection version is missing",
        )?;

        Ok(DiscoveryManifestDocument {
            manifest_id,
            manifest_version,
            collection_id,
            collection_version,
            entries: self.entries,
        })
    }
}

impl SemanticRegistryDraft {
    pub fn validate_with_codes(
        self,
        missing_registry_id_code: &'static str,
        missing_registry_version_code: &'static str,
        duplicate_collection_code: &'static str,
        unsupported_scope_code: &'static str,
        runtime_capability_code: &'static str,
    ) -> Result<SemanticRegistryDocument, String> {
        let registry_id = required_non_empty(
            self.registry_id,
            missing_registry_id_code,
            "semantic registry identity is missing",
        )?;
        let registry_version = required_non_empty(
            self.registry_version,
            missing_registry_version_code,
            "semantic registry version is missing",
        )?;
        let scope = required_non_empty(
            self.scope,
            unsupported_scope_code,
            "semantic registry scope is missing",
        )?;
        if !is_supported_registry_scope(&scope) {
            return Err(format!(
                "{unsupported_scope_code} unsupported semantic registry scope: {scope}"
            ));
        }
        if self.capabilities.runtime {
            return Err(format!(
                "{runtime_capability_code} runtime capability requires approved runtime host boundary"
            ));
        }
        for (index, collection) in self.collections.iter().enumerate() {
            if self.collections.iter().skip(index + 1).any(|candidate| {
                candidate.collection_id == collection.collection_id
                    && candidate.collection_version == collection.collection_version
            }) {
                return Err(format!(
                    "{duplicate_collection_code} duplicate collection ref in semantic registry: {}@{}",
                    collection.collection_id, collection.collection_version
                ));
            }
        }

        Ok(SemanticRegistryDocument {
            registry_id,
            registry_version,
            owner: self.owner.unwrap_or_default(),
            scope,
            collections: self.collections,
            profiles: self.profiles,
            adapters: self.adapters,
            capabilities: self.capabilities,
        })
    }
}

impl StateGraphDraft {
    pub fn validate_with_codes(
        self,
        registry: &SemanticRegistryDocument,
        collections: &[DescriptorCollectionDocument],
        codes: StateGraphValidationCodes,
    ) -> Result<StateGraphDocument, String> {
        let state_graph_id = required_non_empty(
            self.state_graph_id,
            codes.missing_identity,
            "state graph identity is missing",
        )?;
        let state_graph_version = required_non_empty(
            self.state_graph_version,
            codes.missing_identity,
            "state graph version is missing",
        )?;
        let registry_ref = self.registry_ref.ok_or_else(|| {
            format!(
                "{} state graph registry reference is missing",
                codes.missing_registry_ref
            )
        })?;
        if registry_ref.registry_id != registry.registry_id
            || registry_ref.registry_version != registry.registry_version
        {
            return Err(format!(
                "{} state graph registry reference does not match semantic registry: {}@{}",
                codes.missing_registry_ref, registry_ref.registry_id, registry_ref.registry_version
            ));
        }
        if self.capabilities.live_state {
            return Err(format!(
                "{} live state requires approved runtime host boundary",
                codes.live_state_requested
            ));
        }

        let descriptor_kinds = descriptor_kinds_by_id(collections);
        for node in &self.nodes {
            if !descriptor_kinds.contains_key(&node.descriptor_id) {
                return Err(format!(
                    "{} node references unknown descriptor id: {}",
                    codes.unknown_node_descriptor, node.descriptor_id
                ));
            }
        }

        let node_ids: BTreeSet<&str> = self
            .nodes
            .iter()
            .map(|node| node.node_id.as_str())
            .collect();
        for transition in &self.transitions {
            if !node_ids.contains(transition.from_node_id.as_str())
                || !node_ids.contains(transition.to_node_id.as_str())
            {
                return Err(format!(
                    "{} transition references unknown source or target node: {}",
                    codes.unknown_transition_node, transition.transition_id
                ));
            }
            match descriptor_kinds.get(&transition.descriptor_id) {
                Some(kind) if kind == "command" || kind == "event" => {}
                Some(kind) => {
                    return Err(format!(
                        "{} transition descriptor must be command or event: {} ({})",
                        codes.unsupported_transition_descriptor, transition.descriptor_id, kind
                    ));
                }
                None => {
                    return Err(format!(
                        "{} transition references unknown command/event descriptor: {}",
                        codes.unsupported_transition_descriptor, transition.descriptor_id
                    ));
                }
            }
        }

        Ok(StateGraphDocument {
            state_graph_id,
            state_graph_version,
            registry_ref,
            nodes: self.nodes,
            transitions: self.transitions,
            ownership: self.ownership,
            evidence_refs: self.evidence_refs,
            capabilities: self.capabilities,
        })
    }
}

fn descriptor_kinds_by_id(
    collections: &[DescriptorCollectionDocument],
) -> BTreeMap<String, String> {
    let mut descriptor_kinds = BTreeMap::new();
    for collection in collections {
        for descriptor in &collection.descriptors {
            descriptor_kinds.insert(descriptor.id.clone(), descriptor.kind.clone());
        }
    }
    descriptor_kinds
}

pub fn collect_known_contract_documents(
    registrations: &[ContractRegistration],
    duplicate_id_code: &'static str,
) -> Result<Vec<DescriptorDocument>, String> {
    let mut documents = Vec::with_capacity(registrations.len());

    for registration in registrations {
        let descriptor = (registration.descriptor)();
        let document = DescriptorDocument::from_descriptor(descriptor);
        if documents
            .iter()
            .any(|existing: &DescriptorDocument| existing.id == document.id)
        {
            return Err(format!(
                "{duplicate_id_code} duplicate contract id in explicit registry: {}",
                document.id
            ));
        }
        documents.push(document);
    }

    Ok(documents)
}

pub fn is_supported_kind(kind: &str) -> bool {
    matches!(
        kind,
        "entity" | "event" | "command" | "state" | "artifact" | "source" | "evidence" | "other"
    )
}

pub fn is_supported_registry_scope(scope: &str) -> bool {
    matches!(scope, "crate" | "workspace" | "process" | "retained_bundle")
}

fn required_non_empty(
    value: Option<String>,
    code: &'static str,
    message: &'static str,
) -> Result<String, String> {
    match value {
        Some(value) if !value.trim().is_empty() => Ok(value),
        _ => Err(format!("{code} {message}")),
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FieldDocument {
    pub name: String,
    pub rust_type: String,
    #[serde(default, skip_serializing_if = "FieldMetadataDocument::is_empty")]
    pub metadata: FieldMetadataDocument,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct FieldMetadataDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
}

impl FieldMetadataDocument {
    pub fn is_empty(&self) -> bool {
        self.required.is_none()
            && self.unit.is_none()
            && self.min.is_none()
            && self.max.is_none()
            && self.sensitivity.is_none()
            && self.example.is_none()
            && self.stability.is_none()
            && self.aliases.is_empty()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InvariantDocument {
    pub id: String,
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TraceLinkDocument {
    pub relation: String,
    pub target: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExtensionDocument {
    pub namespace: String,
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GeneratedArtifactDocument {
    pub profile_id: String,
    pub profile_version: String,
    pub output_artifact_kind: String,
    pub descriptor: DescriptorDocument,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GeneratedCollectionArtifactDocument {
    pub profile_id: String,
    pub profile_version: String,
    pub output_artifact_kind: String,
    pub collection: DescriptorCollectionDocument,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CheckReportDocument {
    pub status: String,
    pub profile_id: String,
    pub profile_version: String,
    pub descriptor_id: String,
    pub descriptor_version: String,
    pub descriptor_kind: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CheckCollectionReportDocument {
    pub status: String,
    pub profile_id: String,
    pub profile_version: String,
    pub collection_id: String,
    pub collection_version: String,
    pub descriptors: Vec<CheckReportDocument>,
}

impl CheckReportDocument {
    pub fn compatible(profile: &ProfileDescriptor, descriptor: &DescriptorDocument) -> Self {
        Self {
            status: "ok".to_owned(),
            profile_id: profile.profile_id.clone(),
            profile_version: profile.profile_version.clone(),
            descriptor_id: descriptor.id.clone(),
            descriptor_version: descriptor.version.clone(),
            descriptor_kind: descriptor.kind.clone(),
        }
    }
}

impl CheckCollectionReportDocument {
    pub fn compatible(
        profile: &ProfileDescriptor,
        collection: &DescriptorCollectionDocument,
    ) -> Self {
        Self {
            status: "ok".to_owned(),
            profile_id: profile.profile_id.clone(),
            profile_version: profile.profile_version.clone(),
            collection_id: collection.collection_id.clone(),
            collection_version: collection.collection_version.clone(),
            descriptors: collection
                .descriptors
                .iter()
                .map(|descriptor| CheckReportDocument::compatible(profile, descriptor))
                .collect(),
        }
    }
}

impl CollectionEvidenceBundleDocument {
    pub fn from_collection(
        source_kind: &str,
        source: &str,
        profile: &ProfileDescriptor,
        collection: DescriptorCollectionDocument,
    ) -> Self {
        Self {
            source_kind: source_kind.to_owned(),
            source: source.to_owned(),
            profile_id: profile.profile_id.clone(),
            profile_version: profile.profile_version.clone(),
            check_report: CheckCollectionReportDocument::compatible(profile, &collection),
            inventory: collection.inventory(),
            generated_artifact: GeneratedCollectionArtifactDocument {
                profile_id: profile.profile_id.clone(),
                profile_version: profile.profile_version.clone(),
                output_artifact_kind: profile.output_artifact_kind.clone(),
                collection: collection.clone(),
            },
            collection,
        }
    }
}

impl DocumentationPacketDocument {
    pub fn from_descriptor(profile: &ProfileDescriptor, descriptor: DescriptorDocument) -> Self {
        Self {
            profile_id: profile.profile_id.clone(),
            profile_version: profile.profile_version.clone(),
            output_artifact_kind: profile.output_artifact_kind.clone(),
            source_kind: "descriptor".to_owned(),
            source_id: descriptor.id.clone(),
            source_version: descriptor.version.clone(),
            descriptor_count: 1,
            descriptors: vec![DocumentationDescriptorSummary::from_descriptor(descriptor)],
        }
    }

    pub fn from_collection(
        profile: &ProfileDescriptor,
        collection: DescriptorCollectionDocument,
    ) -> Self {
        Self {
            profile_id: profile.profile_id.clone(),
            profile_version: profile.profile_version.clone(),
            output_artifact_kind: profile.output_artifact_kind.clone(),
            source_kind: "collection".to_owned(),
            source_id: collection.collection_id,
            source_version: collection.collection_version,
            descriptor_count: collection.descriptors.len(),
            descriptors: collection
                .descriptors
                .into_iter()
                .map(DocumentationDescriptorSummary::from_descriptor)
                .collect(),
        }
    }
}

impl DocumentationDescriptorSummary {
    fn from_descriptor(descriptor: DescriptorDocument) -> Self {
        Self {
            id: descriptor.id,
            version: descriptor.version,
            kind: descriptor.kind,
            rust_type: descriptor.rust_type,
            field_count: descriptor.fields.len(),
            fields: descriptor.fields,
            invariant_count: descriptor.invariants.len(),
            trace_link_count: descriptor.trace_links.len(),
            trace_links: descriptor.trace_links,
            extension_count: descriptor.extensions.len(),
        }
    }
}

impl DataContractDocument {
    pub fn from_descriptor(profile: &ProfileDescriptor, descriptor: DescriptorDocument) -> Self {
        Self {
            profile_id: profile.profile_id.clone(),
            profile_version: profile.profile_version.clone(),
            output_artifact_kind: profile.output_artifact_kind.clone(),
            source_kind: "descriptor".to_owned(),
            source_id: descriptor.id.clone(),
            source_version: descriptor.version.clone(),
            contract_count: 1,
            contracts: vec![DataContractSummary::from_descriptor(descriptor)],
        }
    }

    pub fn from_collection(
        profile: &ProfileDescriptor,
        collection: DescriptorCollectionDocument,
    ) -> Self {
        Self {
            profile_id: profile.profile_id.clone(),
            profile_version: profile.profile_version.clone(),
            output_artifact_kind: profile.output_artifact_kind.clone(),
            source_kind: "collection".to_owned(),
            source_id: collection.collection_id,
            source_version: collection.collection_version,
            contract_count: collection.descriptors.len(),
            contracts: collection
                .descriptors
                .into_iter()
                .map(DataContractSummary::from_descriptor)
                .collect(),
        }
    }
}

impl DataContractSummary {
    fn from_descriptor(descriptor: DescriptorDocument) -> Self {
        Self {
            id: descriptor.id,
            version: descriptor.version,
            kind: descriptor.kind,
            rust_type: descriptor.rust_type,
            fields: descriptor.fields,
            invariants: descriptor.invariants,
            trace_links: descriptor.trace_links,
            extensions: descriptor.extensions,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProfileCatalog {
    pub profiles: Vec<ProfileDescriptor>,
}

impl ProfileCatalog {
    pub fn approved() -> Self {
        Self {
            profiles: vec![
                neutral_descriptor_profile(),
                documentation_packet_profile(),
                data_contract_profile(),
            ],
        }
    }

    pub fn find(&self, profile_id: &str) -> Option<&ProfileDescriptor> {
        self.profiles
            .iter()
            .find(|profile| profile.profile_id == profile_id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProfileDescriptor {
    pub profile_id: String,
    pub profile_version: String,
    pub input_descriptor_versions: Vec<String>,
    pub supported_kinds: Vec<String>,
    pub supported_concepts: Vec<String>,
    pub unsupported_policy: String,
    pub output_artifact_kind: String,
}

pub struct ProfileCompatibilityCodes {
    pub unsupported_version: &'static str,
    pub unsupported_kind: &'static str,
    pub unsupported_invariant: &'static str,
    pub unsupported_trace_link: &'static str,
    pub unsupported_extension: &'static str,
}

impl ProfileDescriptor {
    pub fn validate_descriptor_with_codes(
        &self,
        descriptor: &DescriptorDocument,
        codes: ProfileCompatibilityCodes,
    ) -> Result<(), String> {
        if !self
            .input_descriptor_versions
            .iter()
            .any(|version| version == &descriptor.version)
        {
            return Err(format!(
                "{} profile {} does not support descriptor version: {}",
                codes.unsupported_version, self.profile_id, descriptor.version
            ));
        }

        if !self
            .supported_kinds
            .iter()
            .any(|kind| kind == &descriptor.kind)
        {
            return Err(format!(
                "{} profile {} does not support descriptor kind: {}",
                codes.unsupported_kind, self.profile_id, descriptor.kind
            ));
        }

        if !descriptor.invariants.is_empty() && !self.supports_concept("invariants") {
            return Err(format!(
                "{} profile {} cannot represent invariants",
                codes.unsupported_invariant, self.profile_id
            ));
        }

        if !descriptor.trace_links.is_empty() && !self.supports_concept("trace_links") {
            return Err(format!(
                "{} profile {} cannot represent trace links",
                codes.unsupported_trace_link, self.profile_id
            ));
        }

        if !descriptor.extensions.is_empty() && !self.supports_concept("extensions") {
            return Err(format!(
                "{} profile {} cannot represent extensions",
                codes.unsupported_extension, self.profile_id
            ));
        }

        Ok(())
    }

    fn supports_concept(&self, concept: &str) -> bool {
        self.supported_concepts
            .iter()
            .any(|supported| supported == concept)
    }
}

pub fn neutral_descriptor_profile() -> ProfileDescriptor {
    ProfileDescriptor {
        profile_id: "rune.neutral_descriptor_json".to_owned(),
        profile_version: "v0".to_owned(),
        input_descriptor_versions: vec!["v0".to_owned()],
        supported_kinds: vec![
            "entity".to_owned(),
            "event".to_owned(),
            "command".to_owned(),
            "state".to_owned(),
            "artifact".to_owned(),
            "source".to_owned(),
            "evidence".to_owned(),
        ],
        supported_concepts: vec![
            "fields".to_owned(),
            "invariants".to_owned(),
            "trace_links".to_owned(),
            "extensions".to_owned(),
        ],
        unsupported_policy: "error".to_owned(),
        output_artifact_kind: "rune.descriptor.json".to_owned(),
    }
}

pub fn documentation_packet_profile() -> ProfileDescriptor {
    ProfileDescriptor {
        profile_id: "rune.documentation_packet_json".to_owned(),
        profile_version: "v0".to_owned(),
        input_descriptor_versions: vec!["v0".to_owned()],
        supported_kinds: vec![
            "entity".to_owned(),
            "event".to_owned(),
            "command".to_owned(),
            "state".to_owned(),
            "artifact".to_owned(),
            "source".to_owned(),
            "evidence".to_owned(),
        ],
        supported_concepts: vec![
            "fields".to_owned(),
            "invariants".to_owned(),
            "trace_links".to_owned(),
            "extensions".to_owned(),
        ],
        unsupported_policy: "error".to_owned(),
        output_artifact_kind: "rune.documentation_packet.json".to_owned(),
    }
}

pub fn data_contract_profile() -> ProfileDescriptor {
    ProfileDescriptor {
        profile_id: "rune.data_contract_json".to_owned(),
        profile_version: "v0".to_owned(),
        input_descriptor_versions: vec!["v0".to_owned()],
        supported_kinds: vec![
            "entity".to_owned(),
            "event".to_owned(),
            "command".to_owned(),
            "state".to_owned(),
            "artifact".to_owned(),
            "source".to_owned(),
            "evidence".to_owned(),
        ],
        supported_concepts: vec![
            "fields".to_owned(),
            "invariants".to_owned(),
            "trace_links".to_owned(),
            "extensions".to_owned(),
        ],
        unsupported_policy: "error".to_owned(),
        output_artifact_kind: "rune.data_contract.json".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Customer;

    impl RuneContract for Customer {
        fn descriptor() -> ContractDescriptor {
            ContractDescriptor {
                id: "example.customer",
                version: "v0",
                rust_type: "Customer",
                kind: ContractKind::Entity,
                fields: &[FieldDescriptor {
                    name: "id",
                    rust_type: "String",
                    metadata: FieldMetadataDescriptor {
                        required: Some(true),
                        unit: None,
                        min: None,
                        max: None,
                        sensitivity: Some("internal"),
                        example: Some("cus_123"),
                        stability: Some("stable"),
                        aliases: &["customer_id"],
                    },
                }],
                invariants: &[InvariantDescriptor {
                    id: "customer.id.present",
                    text: "Customer id is present.",
                }],
                trace_links: &[TraceLink {
                    relation: "requirement",
                    target: "RUNE-REQ-010",
                }],
                extensions: &[ExtensionDescriptor {
                    namespace: "example",
                    name: "profile",
                    value: "demo",
                }],
            }
        }
    }

    struct CustomerAgain;

    impl RuneContract for CustomerAgain {
        fn descriptor() -> ContractDescriptor {
            ContractDescriptor {
                id: "example.customer_again",
                version: "v0",
                rust_type: "CustomerAgain",
                kind: ContractKind::Entity,
                fields: &[],
                invariants: &[],
                trace_links: &[],
                extensions: &[],
            }
        }
    }

    #[test]
    fn descriptor_exposes_approved_shape() {
        let descriptor = Customer::descriptor();

        assert_eq!(descriptor.id, "example.customer");
        assert_eq!(descriptor.version, "v0");
        assert_eq!(descriptor.kind, ContractKind::Entity);
        assert_eq!(descriptor.fields[0].name, "id");
        assert_eq!(descriptor.fields[0].metadata.required, Some(true));
        assert_eq!(descriptor.fields[0].metadata.aliases[0], "customer_id");
        assert_eq!(descriptor.invariants[0].id, "customer.id.present");
        assert_eq!(descriptor.trace_links[0].target, "RUNE-REQ-010");
        assert_eq!(descriptor.extensions[0].namespace, "example");
    }

    #[test]
    fn descriptor_document_preserves_runtime_descriptor_shape() {
        let document = DescriptorDocument::from_contract::<Customer>();

        assert_eq!(document.id, "example.customer");
        assert_eq!(document.version, "v0");
        assert_eq!(document.kind, "entity");
        assert_eq!(document.fields[0].name, "id");
        assert_eq!(document.fields[0].metadata.required, Some(true));
        assert_eq!(document.fields[0].metadata.aliases[0], "customer_id");
        assert_eq!(document.invariants[0].id, "customer.id.present");
        assert_eq!(document.trace_links[0].target, "RUNE-REQ-010");
        assert_eq!(document.extensions[0].namespace, "example");
    }

    #[test]
    fn known_contract_registry_collects_documents_in_registration_order() {
        let registrations = [
            ContractRegistration {
                name: "customer",
                descriptor: Customer::descriptor,
            },
            ContractRegistration {
                name: "customer-again",
                descriptor: CustomerAgain::descriptor,
            },
        ];

        let documents = collect_known_contract_documents(&registrations, "RUNE-REG-001")
            .expect("known contract registry collection");

        assert_eq!(documents[0].id, "example.customer");
        assert_eq!(documents[1].id, "example.customer_again");
    }

    #[test]
    fn known_contract_registry_rejects_duplicate_ids() {
        let registrations = [
            ContractRegistration {
                name: "customer",
                descriptor: Customer::descriptor,
            },
            ContractRegistration {
                name: "duplicate-customer",
                descriptor: Customer::descriptor,
            },
        ];

        let error = collect_known_contract_documents(&registrations, "RUNE-REG-001")
            .expect_err("duplicate contract ids should fail");

        assert!(error.contains("RUNE-REG-001"));
        assert!(error.contains("duplicate contract id"));
    }

    #[test]
    fn descriptor_collection_document_preserves_registry_metadata() {
        let registrations = [
            ContractRegistration {
                name: "customer",
                descriptor: Customer::descriptor,
            },
            ContractRegistration {
                name: "customer-again",
                descriptor: CustomerAgain::descriptor,
            },
        ];

        let collection = DescriptorCollectionDocument::from_registrations(
            "example.known_contracts",
            "v0",
            &registrations,
            "RUNE-REG-001",
        )
        .expect("descriptor collection");

        assert_eq!(collection.collection_id, "example.known_contracts");
        assert_eq!(collection.collection_version, "v0");
        assert_eq!(collection.descriptors[0].id, "example.customer");
        assert_eq!(collection.descriptors[1].id, "example.customer_again");
    }

    #[test]
    fn descriptor_collection_inventory_counts_descriptor_kinds() {
        let collection = DescriptorCollectionDocument {
            collection_id: "example.known_contracts".to_owned(),
            collection_version: "v0".to_owned(),
            descriptors: vec![
                DescriptorDocument::from_contract::<Customer>(),
                DescriptorDocument::from_contract::<CustomerAgain>(),
            ],
        };

        let inventory = collection.inventory();

        assert_eq!(inventory.collection_id, "example.known_contracts");
        assert_eq!(inventory.descriptor_count, 2);
        assert_eq!(inventory.kinds[0].kind, "entity");
        assert_eq!(inventory.kinds[0].descriptor_count, 2);
    }

    #[test]
    fn collection_evidence_bundle_preserves_all_evidence_surfaces() {
        let collection = DescriptorCollectionDocument {
            collection_id: "example.known_contracts".to_owned(),
            collection_version: "v0".to_owned(),
            descriptors: vec![
                DescriptorDocument::from_contract::<Customer>(),
                DescriptorDocument::from_contract::<CustomerAgain>(),
            ],
        };
        let catalog = ProfileCatalog::approved();
        let profile = catalog
            .find("rune.neutral_descriptor_json")
            .expect("neutral profile");

        let bundle = CollectionEvidenceBundleDocument::from_collection(
            "descriptor_collection_fixture",
            "known_contracts.json",
            profile,
            collection,
        );

        assert_eq!(bundle.source_kind, "descriptor_collection_fixture");
        assert_eq!(bundle.collection.collection_id, "example.known_contracts");
        assert_eq!(bundle.check_report.status, "ok");
        assert_eq!(bundle.inventory.descriptor_count, 2);
        assert_eq!(
            bundle.generated_artifact.output_artifact_kind,
            "rune.descriptor.json"
        );
    }

    #[test]
    fn semantic_registry_draft_validates_crate_registry() {
        let registry = SemanticRegistryDraft {
            registry_id: Some("example.registry".to_owned()),
            registry_version: Some("v0".to_owned()),
            owner: Some("example-crate".to_owned()),
            scope: Some("crate".to_owned()),
            collections: vec![SemanticRegistryCollectionRef {
                collection_id: "example.known_contracts".to_owned(),
                collection_version: "v0".to_owned(),
                source_ref: "docs/rune/contracts.json".to_owned(),
                owner: "example-crate".to_owned(),
            }],
            profiles: vec![SemanticRegistryProfileRef {
                profile_id: "rune.neutral_descriptor_json".to_owned(),
                profile_version: "v0".to_owned(),
            }],
            adapters: Vec::new(),
            capabilities: SemanticRegistryCapabilities {
                read: true,
                query: true,
                generate: true,
                mutate: false,
                runtime: false,
            },
        }
        .validate_with_codes(
            "RUNE-REGISTRY-001",
            "RUNE-REGISTRY-002",
            "RUNE-REGISTRY-003",
            "RUNE-REGISTRY-004",
            "RUNE-REGISTRY-007",
        )
        .expect("semantic registry validates");

        assert_eq!(registry.registry_id, "example.registry");
        assert_eq!(registry.scope, "crate");
        assert_eq!(
            registry.collections[0].collection_id,
            "example.known_contracts"
        );
        assert!(registry.capabilities.read);
        assert!(!registry.capabilities.runtime);
    }

    #[test]
    fn semantic_registry_rejects_duplicate_collection_refs() {
        let duplicate = SemanticRegistryCollectionRef {
            collection_id: "example.known_contracts".to_owned(),
            collection_version: "v0".to_owned(),
            source_ref: "docs/rune/contracts.json".to_owned(),
            owner: "example-crate".to_owned(),
        };

        let error = SemanticRegistryDraft {
            registry_id: Some("example.registry".to_owned()),
            registry_version: Some("v0".to_owned()),
            owner: Some("example-crate".to_owned()),
            scope: Some("workspace".to_owned()),
            collections: vec![duplicate.clone(), duplicate],
            profiles: Vec::new(),
            adapters: Vec::new(),
            capabilities: SemanticRegistryCapabilities::default(),
        }
        .validate_with_codes(
            "RUNE-REGISTRY-001",
            "RUNE-REGISTRY-002",
            "RUNE-REGISTRY-003",
            "RUNE-REGISTRY-004",
            "RUNE-REGISTRY-007",
        )
        .expect_err("duplicate collection refs should fail");

        assert!(error.contains("RUNE-REGISTRY-003"));
        assert!(error.contains("duplicate collection ref"));
    }

    #[test]
    fn semantic_registry_rejects_unsupported_scope() {
        let error = SemanticRegistryDraft {
            registry_id: Some("example.registry".to_owned()),
            registry_version: Some("v0".to_owned()),
            owner: Some("example-crate".to_owned()),
            scope: Some("cargo_graph".to_owned()),
            collections: Vec::new(),
            profiles: Vec::new(),
            adapters: Vec::new(),
            capabilities: SemanticRegistryCapabilities::default(),
        }
        .validate_with_codes(
            "RUNE-REGISTRY-001",
            "RUNE-REGISTRY-002",
            "RUNE-REGISTRY-003",
            "RUNE-REGISTRY-004",
            "RUNE-REGISTRY-007",
        )
        .expect_err("unsupported registry scopes should fail");

        assert!(error.contains("RUNE-REGISTRY-004"));
        assert!(error.contains("cargo_graph"));
    }

    #[test]
    fn semantic_registry_blocks_runtime_capability_without_host_boundary() {
        let error = SemanticRegistryDraft {
            registry_id: Some("example.registry".to_owned()),
            registry_version: Some("v0".to_owned()),
            owner: Some("example-crate".to_owned()),
            scope: Some("process".to_owned()),
            collections: Vec::new(),
            profiles: Vec::new(),
            adapters: Vec::new(),
            capabilities: SemanticRegistryCapabilities {
                runtime: true,
                ..SemanticRegistryCapabilities::default()
            },
        }
        .validate_with_codes(
            "RUNE-REGISTRY-001",
            "RUNE-REGISTRY-002",
            "RUNE-REGISTRY-003",
            "RUNE-REGISTRY-004",
            "RUNE-REGISTRY-007",
        )
        .expect_err("runtime capability should remain blocked");

        assert!(error.contains("RUNE-REGISTRY-007"));
        assert!(error.contains("runtime capability"));
    }

    #[test]
    fn retained_semantic_registry_fixture_validates() {
        let fixture = include_str!("../../rune-cli/tests/fixtures/semantic_registry_crate.json");
        let draft: SemanticRegistryDraft =
            serde_json::from_str(fixture).expect("semantic registry fixture parses");
        let registry = draft
            .validate_with_codes(
                "RUNE-REGISTRY-001",
                "RUNE-REGISTRY-002",
                "RUNE-REGISTRY-003",
                "RUNE-REGISTRY-004",
                "RUNE-REGISTRY-007",
            )
            .expect("semantic registry fixture validates");

        assert_eq!(registry.registry_id, "example.registry");
        assert_eq!(
            registry.collections[0].source_ref,
            "docs/rune/contracts.json"
        );
    }

    #[test]
    fn retained_semantic_registry_workspace_fixture_validates() {
        let fixture =
            include_str!("../../rune-cli/tests/fixtures/semantic_registry_workspace.json");
        let draft: SemanticRegistryDraft =
            serde_json::from_str(fixture).expect("workspace registry fixture parses");
        let registry = draft
            .validate_with_codes(
                "RUNE-REGISTRY-001",
                "RUNE-REGISTRY-002",
                "RUNE-REGISTRY-003",
                "RUNE-REGISTRY-004",
                "RUNE-REGISTRY-007",
            )
            .expect("workspace registry fixture validates");

        assert_eq!(registry.scope, "workspace");
        assert_eq!(registry.collections.len(), 2);
        assert_eq!(registry.adapters[0].adapter_id, "rune.review_packet_json");
    }

    #[test]
    fn retained_semantic_registry_duplicate_fixture_fails_closed() {
        let fixture = include_str!(
            "../../rune-cli/tests/fixtures/semantic_registry_duplicate_collection.json"
        );
        let draft: SemanticRegistryDraft =
            serde_json::from_str(fixture).expect("duplicate registry fixture parses");
        let error = draft
            .validate_with_codes(
                "RUNE-REGISTRY-001",
                "RUNE-REGISTRY-002",
                "RUNE-REGISTRY-003",
                "RUNE-REGISTRY-004",
                "RUNE-REGISTRY-007",
            )
            .expect_err("duplicate registry fixture should fail");

        assert!(error.contains("RUNE-REGISTRY-003"));
    }

    #[test]
    fn retained_semantic_registry_runtime_fixture_fails_closed() {
        let fixture =
            include_str!("../../rune-cli/tests/fixtures/semantic_registry_runtime_blocked.json");
        let draft: SemanticRegistryDraft =
            serde_json::from_str(fixture).expect("runtime registry fixture parses");
        let error = draft
            .validate_with_codes(
                "RUNE-REGISTRY-001",
                "RUNE-REGISTRY-002",
                "RUNE-REGISTRY-003",
                "RUNE-REGISTRY-004",
                "RUNE-REGISTRY-007",
            )
            .expect_err("runtime registry fixture should fail");

        assert!(error.contains("RUNE-REGISTRY-007"));
    }

    #[test]
    fn retained_state_graph_fixture_validates_against_registry_collections() {
        let registry = workspace_registry_fixture();
        let collections = workspace_registry_collections();
        let fixture = include_str!("../../rune-cli/tests/fixtures/state_graph_workspace.json");
        let draft: StateGraphDraft =
            serde_json::from_str(fixture).expect("state graph fixture parses");
        let graph = draft
            .validate_with_codes(&registry, &collections, state_graph_codes())
            .expect("state graph fixture validates");

        assert_eq!(graph.state_graph_id, "example.customer_state_graph");
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(
            graph.transitions[0].descriptor_id,
            "example.create_customer"
        );
        assert!(graph.capabilities.retained);
        assert!(!graph.capabilities.live_state);
    }

    #[test]
    fn retained_state_graph_unknown_descriptor_fixture_fails_closed() {
        let registry = workspace_registry_fixture();
        let collections = workspace_registry_collections();
        let fixture =
            include_str!("../../rune-cli/tests/fixtures/state_graph_unknown_descriptor.json");
        let draft: StateGraphDraft =
            serde_json::from_str(fixture).expect("state graph fixture parses");
        let error = draft
            .validate_with_codes(&registry, &collections, state_graph_codes())
            .expect_err("unknown node descriptor should fail");

        assert!(error.contains("RUNE-STATE-003"));
    }

    #[test]
    fn retained_state_graph_unknown_transition_node_fixture_fails_closed() {
        let registry = workspace_registry_fixture();
        let collections = workspace_registry_collections();
        let fixture = include_str!("../../rune-cli/tests/fixtures/state_graph_unknown_node.json");
        let draft: StateGraphDraft =
            serde_json::from_str(fixture).expect("state graph fixture parses");
        let error = draft
            .validate_with_codes(&registry, &collections, state_graph_codes())
            .expect_err("unknown transition node should fail");

        assert!(error.contains("RUNE-STATE-004"));
    }

    #[test]
    fn retained_state_graph_unsupported_transition_fixture_fails_closed() {
        let registry = workspace_registry_fixture();
        let collections = workspace_registry_collections();
        let fixture =
            include_str!("../../rune-cli/tests/fixtures/state_graph_unsupported_transition.json");
        let draft: StateGraphDraft =
            serde_json::from_str(fixture).expect("state graph fixture parses");
        let error = draft
            .validate_with_codes(&registry, &collections, state_graph_codes())
            .expect_err("unsupported transition descriptor should fail");

        assert!(error.contains("RUNE-STATE-005"));
        assert!(error.contains("example.customer"));
    }

    #[test]
    fn retained_state_graph_live_state_fixture_fails_closed() {
        let registry = workspace_registry_fixture();
        let collections = workspace_registry_collections();
        let fixture = include_str!("../../rune-cli/tests/fixtures/state_graph_live_blocked.json");
        let draft: StateGraphDraft =
            serde_json::from_str(fixture).expect("state graph fixture parses");
        let error = draft
            .validate_with_codes(&registry, &collections, state_graph_codes())
            .expect_err("live state should fail closed");

        assert!(error.contains("RUNE-STATE-006"));
    }

    fn state_graph_codes() -> StateGraphValidationCodes {
        StateGraphValidationCodes {
            missing_identity: "RUNE-STATE-001",
            missing_registry_ref: "RUNE-STATE-002",
            unknown_node_descriptor: "RUNE-STATE-003",
            unknown_transition_node: "RUNE-STATE-004",
            unsupported_transition_descriptor: "RUNE-STATE-005",
            live_state_requested: "RUNE-STATE-006",
        }
    }

    fn workspace_registry_fixture() -> SemanticRegistryDocument {
        let fixture =
            include_str!("../../rune-cli/tests/fixtures/semantic_registry_workspace.json");
        let draft: SemanticRegistryDraft =
            serde_json::from_str(fixture).expect("workspace registry fixture parses");
        draft
            .validate_with_codes(
                "RUNE-REGISTRY-001",
                "RUNE-REGISTRY-002",
                "RUNE-REGISTRY-003",
                "RUNE-REGISTRY-004",
                "RUNE-REGISTRY-007",
            )
            .expect("workspace registry fixture validates")
    }

    fn workspace_registry_collections() -> Vec<DescriptorCollectionDocument> {
        let known_fixture =
            include_str!("../../rune-cli/tests/fixtures/known_contract_descriptor_collection.json");
        let adapter_fixture = include_str!(
            "../../rune-cli/tests/fixtures/semantic_registry_adapter_contracts_collection.json"
        );
        [known_fixture, adapter_fixture]
            .into_iter()
            .map(|fixture| {
                let draft: DescriptorCollectionDraft =
                    serde_json::from_str(fixture).expect("collection fixture parses");
                draft
                    .validate_with_codes("RUNE-COLL-001", "RUNE-COLL-002", "RUNE-COLL-003")
                    .expect("collection fixture validates")
            })
            .collect()
    }

    #[test]
    fn documentation_packet_profile_is_in_approved_catalog() {
        let catalog = ProfileCatalog::approved();
        let profile = catalog
            .find("rune.documentation_packet_json")
            .expect("documentation packet profile");

        assert_eq!(profile.profile_version, "v0");
        assert_eq!(
            profile.output_artifact_kind,
            "rune.documentation_packet.json"
        );
        assert!(profile.supported_kinds.iter().any(|kind| kind == "command"));
    }

    #[test]
    fn data_contract_profile_is_in_approved_catalog() {
        let catalog = ProfileCatalog::approved();
        let profile = catalog
            .find("rune.data_contract_json")
            .expect("data contract profile");

        assert_eq!(profile.profile_version, "v0");
        assert_eq!(profile.output_artifact_kind, "rune.data_contract.json");
        assert!(
            profile
                .supported_concepts
                .iter()
                .any(|kind| kind == "fields")
        );
        assert!(
            profile
                .supported_concepts
                .iter()
                .any(|kind| kind == "invariants")
        );
    }

    #[test]
    fn data_contract_profile_preserves_metadata() {
        let profile = data_contract_profile();
        let document = DataContractDocument::from_descriptor(
            &profile,
            DescriptorDocument::from_contract::<Customer>(),
        );

        assert_eq!(document.profile_id, "rune.data_contract_json");
        assert_eq!(document.contract_count, 1);
        assert_eq!(document.contracts[0].id, "example.customer");
        assert_eq!(document.contracts[0].fields[0].name, "id");
        assert_eq!(
            document.contracts[0].fields[0].metadata.sensitivity,
            Some("internal".to_owned())
        );
        assert_eq!(
            document.contracts[0].invariants[0].id,
            "customer.id.present"
        );
        assert_eq!(document.contracts[0].extensions[0].namespace, "example");
    }

    #[test]
    fn documentation_packet_summarizes_collection_descriptors() {
        let collection = DescriptorCollectionDocument {
            collection_id: "example.known_contracts".to_owned(),
            collection_version: "v0".to_owned(),
            descriptors: vec![
                DescriptorDocument::from_contract::<Customer>(),
                DescriptorDocument::from_contract::<CustomerAgain>(),
            ],
        };
        let profile = documentation_packet_profile();

        let packet = DocumentationPacketDocument::from_collection(&profile, collection);

        assert_eq!(packet.source_kind, "collection");
        assert_eq!(packet.source_id, "example.known_contracts");
        assert_eq!(packet.descriptor_count, 2);
        assert_eq!(packet.descriptors[0].id, "example.customer");
        assert_eq!(packet.descriptors[0].field_count, 1);
        assert_eq!(packet.descriptors[0].trace_link_count, 1);
    }

    #[test]
    fn discovery_manifest_draft_validates_into_document() {
        let draft = DiscoveryManifestDraft {
            manifest_id: Some("example.discovery".to_owned()),
            manifest_version: Some("v0".to_owned()),
            collection_id: Some("example.discovered_contracts".to_owned()),
            collection_version: Some("v0".to_owned()),
            entries: vec![DiscoveryManifestEntry {
                name: "known-contracts".to_owned(),
                source_kind: "descriptor_collection_fixture".to_owned(),
                source: "known_contracts.json".to_owned(),
            }],
        };

        let manifest = draft
            .validate_with_codes(
                "RUNE-DISC-001",
                "RUNE-DISC-002",
                "RUNE-DISC-003",
                "RUNE-DISC-004",
            )
            .expect("valid discovery manifest");

        assert_eq!(manifest.manifest_id, "example.discovery");
        assert_eq!(
            manifest.entries[0].source_kind,
            "descriptor_collection_fixture"
        );
    }

    #[test]
    fn discovery_manifest_draft_rejects_missing_identity() {
        let draft = DiscoveryManifestDraft {
            manifest_id: None,
            manifest_version: Some("v0".to_owned()),
            collection_id: Some("example.discovered_contracts".to_owned()),
            collection_version: Some("v0".to_owned()),
            entries: Vec::new(),
        };

        let error = draft
            .validate_with_codes(
                "RUNE-DISC-001",
                "RUNE-DISC-002",
                "RUNE-DISC-003",
                "RUNE-DISC-004",
            )
            .expect_err("missing discovery manifest identity should fail");

        assert!(error.contains("RUNE-DISC-001"));
    }

    #[test]
    fn discovered_collections_preserve_manifest_source_order() {
        let manifest = DiscoveryManifestDocument {
            manifest_id: "example.discovery".to_owned(),
            manifest_version: "v0".to_owned(),
            collection_id: "example.discovered_contracts".to_owned(),
            collection_version: "v0".to_owned(),
            entries: Vec::new(),
        };
        let first = DescriptorCollectionDocument {
            collection_id: "example.first".to_owned(),
            collection_version: "v0".to_owned(),
            descriptors: vec![DescriptorDocument::from_contract::<Customer>()],
        };
        let second = DescriptorCollectionDocument {
            collection_id: "example.second".to_owned(),
            collection_version: "v0".to_owned(),
            descriptors: vec![DescriptorDocument::from_contract::<CustomerAgain>()],
        };

        let discovered = DescriptorCollectionDocument::from_discovered_collections(
            &manifest,
            &[first, second],
            "RUNE-DISC-007",
        )
        .expect("discovered collection");

        assert_eq!(discovered.collection_id, "example.discovered_contracts");
        assert_eq!(discovered.descriptors[0].id, "example.customer");
        assert_eq!(discovered.descriptors[1].id, "example.customer_again");
    }

    #[test]
    fn discovered_collections_reject_duplicate_descriptor_ids() {
        let manifest = DiscoveryManifestDocument {
            manifest_id: "example.discovery".to_owned(),
            manifest_version: "v0".to_owned(),
            collection_id: "example.discovered_contracts".to_owned(),
            collection_version: "v0".to_owned(),
            entries: Vec::new(),
        };
        let collection = DescriptorCollectionDocument {
            collection_id: "example.known_contracts".to_owned(),
            collection_version: "v0".to_owned(),
            descriptors: vec![DescriptorDocument::from_contract::<Customer>()],
        };

        let error = DescriptorCollectionDocument::from_discovered_collections(
            &manifest,
            &[collection.clone(), collection],
            "RUNE-DISC-007",
        )
        .expect_err("duplicate discovered descriptor ids should fail");

        assert!(error.contains("RUNE-DISC-007"));
        assert!(error.contains("duplicate descriptor id"));
    }

    #[test]
    fn descriptor_collection_draft_validates_into_document() {
        let draft = DescriptorCollectionDraft {
            collection_id: Some("example.known_contracts".to_owned()),
            collection_version: Some("v0".to_owned()),
            descriptors: vec![DescriptorDocument::from_contract::<Customer>()],
        };

        let collection = draft
            .validate_with_codes("RUNE-COLL-001", "RUNE-COLL-002", "RUNE-COLL-003")
            .expect("valid descriptor collection draft");

        assert_eq!(collection.collection_id, "example.known_contracts");
        assert_eq!(collection.collection_version, "v0");
        assert_eq!(collection.descriptors[0].id, "example.customer");
    }

    #[test]
    fn descriptor_collection_draft_rejects_missing_identity() {
        let draft = DescriptorCollectionDraft {
            collection_id: None,
            collection_version: Some("v0".to_owned()),
            descriptors: vec![DescriptorDocument::from_contract::<Customer>()],
        };

        let error = draft
            .validate_with_codes("RUNE-COLL-001", "RUNE-COLL-002", "RUNE-COLL-003")
            .expect_err("missing collection identity should fail");

        assert!(error.contains("RUNE-COLL-001"));
    }

    #[test]
    fn descriptor_collection_draft_rejects_missing_version() {
        let draft = DescriptorCollectionDraft {
            collection_id: Some("example.known_contracts".to_owned()),
            collection_version: None,
            descriptors: vec![DescriptorDocument::from_contract::<Customer>()],
        };

        let error = draft
            .validate_with_codes("RUNE-COLL-001", "RUNE-COLL-002", "RUNE-COLL-003")
            .expect_err("missing collection version should fail");

        assert!(error.contains("RUNE-COLL-002"));
    }

    #[test]
    fn descriptor_collection_draft_rejects_duplicate_descriptor_ids() {
        let draft = DescriptorCollectionDraft {
            collection_id: Some("example.known_contracts".to_owned()),
            collection_version: Some("v0".to_owned()),
            descriptors: vec![
                DescriptorDocument::from_contract::<Customer>(),
                DescriptorDocument::from_contract::<Customer>(),
            ],
        };

        let error = draft
            .validate_with_codes("RUNE-COLL-001", "RUNE-COLL-002", "RUNE-COLL-003")
            .expect_err("duplicate descriptor ids should fail");

        assert!(error.contains("RUNE-COLL-003"));
        assert!(error.contains("duplicate descriptor id"));
    }

    #[test]
    fn descriptor_draft_validates_into_document() {
        let draft = DescriptorDraft {
            id: Some("example.customer".to_owned()),
            version: Some("v0".to_owned()),
            kind: Some("entity".to_owned()),
            rust_type: Some("Customer".to_owned()),
            fields: vec![FieldDocument {
                name: "id".to_owned(),
                rust_type: "String".to_owned(),
                metadata: FieldMetadataDocument::default(),
            }],
            invariants: Vec::new(),
            trace_links: Vec::new(),
            extensions: Vec::new(),
        };

        let document = draft
            .validate_with_codes("RUNE-INSP-001", "RUNE-INSP-002", "RUNE-INSP-003")
            .expect("valid descriptor draft");

        assert_eq!(document.id, "example.customer");
        assert_eq!(document.fields[0].name, "id");
    }

    #[test]
    fn descriptor_draft_rejects_missing_identity() {
        let draft = DescriptorDraft {
            id: None,
            version: Some("v0".to_owned()),
            kind: Some("entity".to_owned()),
            rust_type: None,
            fields: Vec::new(),
            invariants: Vec::new(),
            trace_links: Vec::new(),
            extensions: Vec::new(),
        };

        let error = draft
            .validate_with_codes("RUNE-INSP-001", "RUNE-INSP-002", "RUNE-INSP-003")
            .expect_err("missing identity should fail");

        assert!(error.contains("RUNE-INSP-001"));
    }

    #[test]
    fn approved_profile_catalog_exposes_neutral_descriptor_profile() {
        let catalog = ProfileCatalog::approved();
        let profile = catalog
            .find("rune.neutral_descriptor_json")
            .expect("neutral descriptor profile");

        assert_eq!(profile.profile_version, "v0");
        assert_eq!(profile.output_artifact_kind, "rune.descriptor.json");
        assert!(profile.supported_kinds.iter().any(|kind| kind == "entity"));
        assert!(
            profile
                .supported_concepts
                .iter()
                .any(|concept| concept == "trace_links")
        );
    }

    #[test]
    fn profile_rejects_unsupported_descriptor_kind() {
        let profile = neutral_descriptor_profile();
        let descriptor = DescriptorDocument {
            id: "example.custom".to_owned(),
            version: "v0".to_owned(),
            kind: "other".to_owned(),
            rust_type: "Custom".to_owned(),
            fields: Vec::new(),
            invariants: Vec::new(),
            trace_links: Vec::new(),
            extensions: Vec::new(),
        };

        let error = profile
            .validate_descriptor_with_codes(&descriptor, generate_compatibility_codes())
            .expect_err("other is not supported by neutral descriptor profile");

        assert!(error.contains("RUNE-GEN-003"));
        assert!(error.contains("does not support descriptor kind"));
    }

    #[test]
    fn profile_rejects_unsupported_descriptor_version() {
        let profile = neutral_descriptor_profile();
        let descriptor = DescriptorDocument {
            id: "example.customer".to_owned(),
            version: "v9".to_owned(),
            kind: "entity".to_owned(),
            rust_type: "Customer".to_owned(),
            fields: Vec::new(),
            invariants: Vec::new(),
            trace_links: Vec::new(),
            extensions: Vec::new(),
        };

        let error = profile
            .validate_descriptor_with_codes(&descriptor, generate_compatibility_codes())
            .expect_err("v9 is not supported by neutral descriptor profile");

        assert!(error.contains("RUNE-GEN-007"));
        assert!(error.contains("does not support descriptor version"));
    }

    #[test]
    fn profile_rejects_unsupported_invariants() {
        let mut profile = neutral_descriptor_profile();
        profile
            .supported_concepts
            .retain(|concept| concept != "invariants");
        let descriptor = DescriptorDocument {
            id: "example.customer".to_owned(),
            version: "v0".to_owned(),
            kind: "entity".to_owned(),
            rust_type: "Customer".to_owned(),
            fields: Vec::new(),
            invariants: vec![InvariantDocument {
                id: "customer.id.present".to_owned(),
                text: "Customer id is present.".to_owned(),
            }],
            trace_links: Vec::new(),
            extensions: Vec::new(),
        };

        let error = profile
            .validate_descriptor_with_codes(&descriptor, generate_compatibility_codes())
            .expect_err("profile without invariant support should fail");

        assert!(error.contains("RUNE-GEN-004"));
        assert!(error.contains("cannot represent invariants"));
    }

    #[test]
    fn profile_rejects_unsupported_trace_links() {
        let mut profile = neutral_descriptor_profile();
        profile
            .supported_concepts
            .retain(|concept| concept != "trace_links");
        let descriptor = DescriptorDocument {
            id: "example.customer".to_owned(),
            version: "v0".to_owned(),
            kind: "entity".to_owned(),
            rust_type: "Customer".to_owned(),
            fields: Vec::new(),
            invariants: Vec::new(),
            trace_links: vec![TraceLinkDocument {
                relation: "requirement".to_owned(),
                target: "RUNE-REQ-013".to_owned(),
            }],
            extensions: Vec::new(),
        };

        let error = profile
            .validate_descriptor_with_codes(&descriptor, generate_compatibility_codes())
            .expect_err("profile without trace link support should fail");

        assert!(error.contains("RUNE-GEN-005"));
        assert!(error.contains("cannot represent trace links"));
    }

    #[test]
    fn profile_rejects_unsupported_extensions() {
        let mut profile = neutral_descriptor_profile();
        profile
            .supported_concepts
            .retain(|concept| concept != "extensions");
        let descriptor = DescriptorDocument {
            id: "example.customer".to_owned(),
            version: "v0".to_owned(),
            kind: "entity".to_owned(),
            rust_type: "Customer".to_owned(),
            fields: Vec::new(),
            invariants: Vec::new(),
            trace_links: Vec::new(),
            extensions: vec![ExtensionDocument {
                namespace: "example".to_owned(),
                name: "profile".to_owned(),
                value: "demo".to_owned(),
            }],
        };

        let error = profile
            .validate_descriptor_with_codes(&descriptor, generate_compatibility_codes())
            .expect_err("profile without extension support should fail");

        assert!(error.contains("RUNE-GEN-006"));
        assert!(error.contains("cannot represent extensions"));
    }

    #[test]
    fn check_report_preserves_compatibility_context() {
        let profile = neutral_descriptor_profile();
        let descriptor = DescriptorDocument {
            id: "example.customer".to_owned(),
            version: "v0".to_owned(),
            kind: "entity".to_owned(),
            rust_type: "Customer".to_owned(),
            fields: Vec::new(),
            invariants: Vec::new(),
            trace_links: Vec::new(),
            extensions: Vec::new(),
        };

        let report = CheckReportDocument::compatible(&profile, &descriptor);

        assert_eq!(report.status, "ok");
        assert_eq!(report.profile_id, "rune.neutral_descriptor_json");
        assert_eq!(report.descriptor_id, "example.customer");
    }

    #[test]
    fn check_collection_report_preserves_collection_context() {
        let profile = neutral_descriptor_profile();
        let collection = DescriptorCollectionDocument {
            collection_id: "example.known_contracts".to_owned(),
            collection_version: "v0".to_owned(),
            descriptors: vec![DescriptorDocument {
                id: "example.customer".to_owned(),
                version: "v0".to_owned(),
                kind: "entity".to_owned(),
                rust_type: "Customer".to_owned(),
                fields: Vec::new(),
                invariants: Vec::new(),
                trace_links: Vec::new(),
                extensions: Vec::new(),
            }],
        };

        let report = CheckCollectionReportDocument::compatible(&profile, &collection);

        assert_eq!(report.status, "ok");
        assert_eq!(report.profile_id, "rune.neutral_descriptor_json");
        assert_eq!(report.collection_id, "example.known_contracts");
        assert_eq!(report.descriptors[0].descriptor_id, "example.customer");
    }

    #[test]
    fn generated_collection_artifact_preserves_profile_and_collection_context() {
        let profile = neutral_descriptor_profile();
        let collection = DescriptorCollectionDocument {
            collection_id: "example.known_contracts".to_owned(),
            collection_version: "v0".to_owned(),
            descriptors: vec![DescriptorDocument {
                id: "example.customer".to_owned(),
                version: "v0".to_owned(),
                kind: "entity".to_owned(),
                rust_type: "Customer".to_owned(),
                fields: Vec::new(),
                invariants: Vec::new(),
                trace_links: Vec::new(),
                extensions: Vec::new(),
            }],
        };

        let artifact = GeneratedCollectionArtifactDocument {
            profile_id: profile.profile_id,
            profile_version: profile.profile_version,
            output_artifact_kind: profile.output_artifact_kind,
            collection,
        };

        assert_eq!(artifact.profile_id, "rune.neutral_descriptor_json");
        assert_eq!(artifact.collection.collection_id, "example.known_contracts");
        assert_eq!(artifact.collection.descriptors[0].id, "example.customer");
    }

    fn generate_compatibility_codes() -> ProfileCompatibilityCodes {
        ProfileCompatibilityCodes {
            unsupported_version: "RUNE-GEN-007",
            unsupported_kind: "RUNE-GEN-003",
            unsupported_invariant: "RUNE-GEN-004",
            unsupported_trace_link: "RUNE-GEN-005",
            unsupported_extension: "RUNE-GEN-006",
        }
    }
}
