use rune_adapters::{
    AdapterCatalog, AdapterCompatibilityCodes, ReviewPacketDocument, review_packet_adapter,
};
use rune_core::{
    CheckCollectionReportDocument, CheckReportDocument, CollectionEvidenceBundleDocument,
    DataContractDocument, DescriptorCollectionDocument, DescriptorCollectionDraft, DescriptorDraft,
    DescriptorKindInventoryDocument, DiscoveryManifestDraft, DocumentationPacketDocument,
    GeneratedArtifactDocument, GeneratedCollectionArtifactDocument, ProfileCatalog,
    ProfileCompatibilityCodes, SemanticRegistryCapabilities, SemanticRegistryDocument,
    SemanticRegistryDraft, StateGraphCapabilities, StateGraphDraft, StateGraphValidationCodes,
};
use serde::Serialize;
use std::path::Path;

fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "status".to_owned());

    let result = match command.as_str() {
        "status" => {
            println!("RUNE v1 surface: rune-core, rune-derive, rune-cli, rune-adapters");
            println!("approved stage: v1 implementation waves");
            println!("contract kinds: entity,event,command,state,artifact,source,evidence,other");
            println!(
                "approved commands: status,inspect --fixture <path>,inspect-collection --fixture <path>,inventory-collection --fixture <path>,discover --manifest <path>,evidence-collection --profile rune.neutral_descriptor_json (--fixture <path> | --manifest <path>),adapt-collection --adapter rune.review_packet_json --fixture <path>,adapter list,check --profile <profile-id> --fixture <path>,check-collection --profile <profile-id> --fixture <path>,check-registry --fixture <path>,inspect-registry --fixture <path>,check-state-graph --fixture <path> --registry <path>,generate --profile <profile-id> --fixture <path>,generate-collection --profile <profile-id> --fixture <path>,profile list"
            );
            println!(
                "approved profiles: rune.neutral_descriptor_json,rune.documentation_packet_json,rune.data_contract_json"
            );
            println!("approved adapters: rune.review_packet_json");
            println!("deferred surfaces: arbitrary crate discovery, product-specific adapters");
            Ok(())
        }
        "inspect" => inspect(args.collect()),
        "inspect-collection" => inspect_collection(args.collect()),
        "inventory-collection" => inventory_collection(args.collect()),
        "discover" => discover(args.collect()),
        "evidence-collection" => evidence_collection(args.collect()),
        "adapt-collection" => adapt_collection(args.collect()),
        "adapter" => adapter(args.collect()),
        "check" => check(args.collect()),
        "check-collection" => check_collection(args.collect()),
        "check-registry" => check_registry(args.collect()),
        "inspect-registry" => inspect_registry(args.collect()),
        "check-state-graph" => check_state_graph(args.collect()),
        "generate" => generate(args.collect()),
        "generate-collection" => generate_collection(args.collect()),
        "profile" => profile(args.collect()),
        other => Err(format!("unknown command: {other}")),
    };

    if let Err(message) = result {
        eprintln!("{message}");
        std::process::exit(2);
    }
}

#[derive(Serialize)]
struct SemanticRegistryCheckReportDocument {
    status: String,
    registry_id: String,
    registry_version: String,
    scope: String,
    collection_count: usize,
    profile_count: usize,
    adapter_count: usize,
    capabilities: SemanticRegistryCapabilities,
}

#[derive(Serialize)]
struct SemanticRegistryInspectionDocument {
    status: String,
    registry: SemanticRegistryDocument,
    collections: Vec<SemanticRegistryCollectionInspectionDocument>,
}

#[derive(Serialize)]
struct SemanticRegistryCollectionInspectionDocument {
    collection_id: String,
    collection_version: String,
    source_ref: String,
    owner: String,
    descriptor_count: usize,
    kinds: Vec<DescriptorKindInventoryDocument>,
}

#[derive(Serialize)]
struct StateGraphCheckReportDocument {
    status: String,
    state_graph_id: String,
    state_graph_version: String,
    registry_id: String,
    registry_version: String,
    node_count: usize,
    transition_count: usize,
    evidence_ref_count: usize,
    capabilities: StateGraphCapabilities,
}

fn inspect_collection(args: Vec<String>) -> Result<(), String> {
    let fixture = parse_fixture_arg(&args, "usage: rune inspect-collection --fixture <path>")?;
    let collection = read_collection_fixture(&fixture, "RUNE-COLL-INSP-900")?;
    let inspected = collection.validate_with_codes(
        "RUNE-COLL-INSP-001",
        "RUNE-COLL-INSP-002",
        "RUNE-COLL-INSP-003",
    )?;
    let output = serde_json::to_string_pretty(&inspected).map_err(|error| {
        format!("RUNE-COLL-INSP-900 error serializing collection inspection output: {error}")
    })?;
    println!("{output}");
    Ok(())
}

fn inventory_collection(args: Vec<String>) -> Result<(), String> {
    let fixture = parse_fixture_arg(&args, "usage: rune inventory-collection --fixture <path>")?;
    let collection = read_collection_fixture(&fixture, "RUNE-COLL-INV-900")?;
    let collection = collection.validate_with_codes(
        "RUNE-COLL-INV-001",
        "RUNE-COLL-INV-002",
        "RUNE-COLL-INV-003",
    )?;
    let output = serde_json::to_string_pretty(&collection.inventory()).map_err(|error| {
        format!("RUNE-COLL-INV-900 error serializing collection inventory output: {error}")
    })?;
    println!("{output}");
    Ok(())
}

fn discover(args: Vec<String>) -> Result<(), String> {
    let manifest_path = parse_manifest_arg(&args)?;
    let discovered = build_discovered_collection(&manifest_path)?;
    let output = serde_json::to_string_pretty(&discovered).map_err(|error| {
        format!("RUNE-DISC-900 error serializing discovered collection output: {error}")
    })?;
    println!("{output}");
    Ok(())
}

fn build_discovered_collection(
    manifest_path: &str,
) -> Result<DescriptorCollectionDocument, String> {
    let manifest_draft = read_discovery_manifest(manifest_path, "RUNE-DISC-900")?;
    let manifest = manifest_draft.validate_with_codes(
        "RUNE-DISC-001",
        "RUNE-DISC-002",
        "RUNE-DISC-003",
        "RUNE-DISC-004",
    )?;

    let manifest_dir = Path::new(&manifest_path).parent().unwrap_or(Path::new(""));
    let mut source_collections = Vec::with_capacity(manifest.entries.len());
    for entry in &manifest.entries {
        if entry.source_kind != "descriptor_collection_fixture" {
            return Err(format!(
                "RUNE-DISC-005 unsupported discovery source kind: {}",
                entry.source_kind
            ));
        }
        let source_path = manifest_dir.join(&entry.source);
        let source_collection =
            read_collection_fixture(source_path.to_string_lossy().as_ref(), "RUNE-DISC-006")?;
        source_collections.push(source_collection.validate_with_codes(
            "RUNE-DISC-006",
            "RUNE-DISC-006",
            "RUNE-DISC-006",
        )?);
    }

    let discovered = DescriptorCollectionDocument::from_discovered_collections(
        &manifest,
        &source_collections,
        "RUNE-DISC-007",
    )?;
    Ok(discovered)
}

fn evidence_collection(args: Vec<String>) -> Result<(), String> {
    let options = parse_evidence_collection_args(&args)?;
    let catalog = ProfileCatalog::approved();
    let Some(profile) = catalog.find(&options.profile) else {
        return Err(format!(
            "RUNE-EVID-003 unsupported profile: {}",
            options.profile
        ));
    };

    let collection = match options.source_kind.as_str() {
        "descriptor_collection_fixture" => {
            let collection = read_collection_fixture(&options.source, "RUNE-EVID-900")?;
            collection.validate_with_codes("RUNE-EVID-001", "RUNE-EVID-002", "RUNE-EVID-008")?
        }
        "discovery_manifest" => build_discovered_collection(&options.source)?,
        _ => {
            return Err(format!(
                "RUNE-EVID-004 unsupported evidence source kind: {}",
                options.source_kind
            ));
        }
    };
    for descriptor in &collection.descriptors {
        profile.validate_descriptor_with_codes(
            descriptor,
            ProfileCompatibilityCodes {
                unsupported_version: "RUNE-EVID-007",
                unsupported_kind: "RUNE-EVID-003",
                unsupported_invariant: "RUNE-EVID-004",
                unsupported_trace_link: "RUNE-EVID-005",
                unsupported_extension: "RUNE-EVID-006",
            },
        )?;
    }

    let bundle = CollectionEvidenceBundleDocument::from_collection(
        &options.source_kind,
        &evidence_source_label(&options.source),
        profile,
        collection,
    );
    let output = serde_json::to_string_pretty(&bundle).map_err(|error| {
        format!("RUNE-EVID-900 error serializing collection evidence bundle: {error}")
    })?;
    println!("{output}");
    Ok(())
}

fn adapt_collection(args: Vec<String>) -> Result<(), String> {
    let options = parse_adapter_fixture_args(
        &args,
        "usage: rune adapt-collection --adapter rune.review_packet_json --fixture <path>",
    )?;
    let catalog = AdapterCatalog::approved();
    let Some(adapter) = catalog.find(&options.adapter) else {
        return Err(format!(
            "RUNE-ADAPT-001 unsupported adapter: {}",
            options.adapter
        ));
    };

    let collection = read_collection_fixture(&options.fixture, "RUNE-ADAPT-900")?;
    let collection =
        collection.validate_with_codes("RUNE-ADAPT-002", "RUNE-ADAPT-002", "RUNE-ADAPT-007")?;
    adapter.validate_collection_with_codes(
        &collection,
        AdapterCompatibilityCodes {
            unsupported_kind: "RUNE-ADAPT-003",
            unsupported_invariant: "RUNE-ADAPT-004",
            unsupported_trace_link: "RUNE-ADAPT-005",
            unsupported_extension: "RUNE-ADAPT-006",
        },
    )?;

    let output = if adapter.adapter_id == "rune.review_packet_json" {
        serde_json::to_string_pretty(&ReviewPacketDocument::from_collection(
            &review_packet_adapter(),
            &collection,
        ))
    } else {
        return Err(format!(
            "RUNE-ADAPT-001 unsupported adapter: {}",
            adapter.adapter_id
        ));
    }
    .map_err(|error| format!("RUNE-ADAPT-900 error serializing adapter output: {error}"))?;
    println!("{output}");
    Ok(())
}

fn inspect(args: Vec<String>) -> Result<(), String> {
    let fixture = parse_fixture_arg(&args, "usage: rune inspect --fixture <path>")?;
    let descriptor = read_descriptor_fixture(&fixture, "RUNE-INSP-900")?;
    let inspected =
        descriptor.validate_with_codes("RUNE-INSP-001", "RUNE-INSP-002", "RUNE-INSP-003")?;
    let output = serde_json::to_string_pretty(&inspected)
        .map_err(|error| format!("RUNE-INSP-900 error serializing inspection output: {error}"))?;
    println!("{output}");
    Ok(())
}

fn check_collection(args: Vec<String>) -> Result<(), String> {
    let options = parse_profile_fixture_args(
        &args,
        "usage: rune check-collection --profile rune.neutral_descriptor_json --fixture <path>",
    )?;
    let catalog = ProfileCatalog::approved();
    let Some(profile) = catalog.find(&options.profile) else {
        return Err(format!(
            "RUNE-COLL-CHECK-003 unsupported profile: {}",
            options.profile
        ));
    };

    let collection = read_collection_fixture(&options.fixture, "RUNE-COLL-CHECK-900")?;
    let collection = collection.validate_with_codes(
        "RUNE-COLL-CHECK-001",
        "RUNE-COLL-CHECK-002",
        "RUNE-COLL-CHECK-008",
    )?;
    for descriptor in &collection.descriptors {
        profile.validate_descriptor_with_codes(
            descriptor,
            ProfileCompatibilityCodes {
                unsupported_version: "RUNE-COLL-CHECK-007",
                unsupported_kind: "RUNE-COLL-CHECK-003",
                unsupported_invariant: "RUNE-COLL-CHECK-004",
                unsupported_trace_link: "RUNE-COLL-CHECK-005",
                unsupported_extension: "RUNE-COLL-CHECK-006",
            },
        )?;
    }

    let output = serde_json::to_string_pretty(&CheckCollectionReportDocument::compatible(
        profile,
        &collection,
    ))
    .map_err(|error| {
        format!("RUNE-COLL-CHECK-900 error serializing collection check report: {error}")
    })?;
    println!("{output}");
    Ok(())
}

fn check_registry(args: Vec<String>) -> Result<(), String> {
    let fixture = parse_fixture_arg(&args, "usage: rune check-registry --fixture <path>")?;
    let registry = read_semantic_registry_fixture(&fixture, "RUNE-REGISTRY-900")?;
    let registry = registry.validate_with_codes(
        "RUNE-REGISTRY-001",
        "RUNE-REGISTRY-002",
        "RUNE-REGISTRY-003",
        "RUNE-REGISTRY-004",
        "RUNE-REGISTRY-007",
    )?;
    check_registry_catalog_refs(&registry)?;
    let collections = load_registry_collection_refs(&registry, &fixture)?;

    let output = serde_json::to_string_pretty(&SemanticRegistryCheckReportDocument {
        status: "ok".to_owned(),
        registry_id: registry.registry_id,
        registry_version: registry.registry_version,
        scope: registry.scope,
        collection_count: registry.collections.len(),
        profile_count: registry.profiles.len(),
        adapter_count: registry.adapters.len(),
        capabilities: registry.capabilities,
    })
    .map_err(|error| {
        format!("RUNE-REGISTRY-900 error serializing registry check report: {error}")
    })?;
    println!("{output}");
    drop(collections);
    Ok(())
}

fn inspect_registry(args: Vec<String>) -> Result<(), String> {
    let fixture = parse_fixture_arg(&args, "usage: rune inspect-registry --fixture <path>")?;
    let registry = read_semantic_registry_fixture(&fixture, "RUNE-REGISTRY-900")?;
    let registry = registry.validate_with_codes(
        "RUNE-REGISTRY-001",
        "RUNE-REGISTRY-002",
        "RUNE-REGISTRY-003",
        "RUNE-REGISTRY-004",
        "RUNE-REGISTRY-007",
    )?;
    check_registry_catalog_refs(&registry)?;
    let collections = load_registry_collection_refs(&registry, &fixture)?;
    let collection_summaries = collections
        .into_iter()
        .zip(registry.collections.iter())
        .map(|(collection, collection_ref)| {
            let inventory = collection.inventory();
            SemanticRegistryCollectionInspectionDocument {
                collection_id: collection.collection_id,
                collection_version: collection.collection_version,
                source_ref: collection_ref.source_ref.clone(),
                owner: collection_ref.owner.clone(),
                descriptor_count: inventory.descriptor_count,
                kinds: inventory.kinds,
            }
        })
        .collect();

    let output = serde_json::to_string_pretty(&SemanticRegistryInspectionDocument {
        status: "ok".to_owned(),
        registry,
        collections: collection_summaries,
    })
    .map_err(|error| {
        format!("RUNE-REGISTRY-900 error serializing registry inspection report: {error}")
    })?;
    println!("{output}");
    Ok(())
}

fn check_state_graph(args: Vec<String>) -> Result<(), String> {
    let options = parse_state_graph_args(&args)?;
    let registry = read_semantic_registry_fixture(&options.registry, "RUNE-STATE-900")?;
    let registry = registry.validate_with_codes(
        "RUNE-REGISTRY-001",
        "RUNE-REGISTRY-002",
        "RUNE-REGISTRY-003",
        "RUNE-REGISTRY-004",
        "RUNE-REGISTRY-007",
    )?;
    check_registry_catalog_refs(&registry)?;
    let collections = load_registry_collection_refs(&registry, &options.registry)?;
    let graph = read_state_graph_fixture(&options.fixture, "RUNE-STATE-900")?;
    let graph = graph.validate_with_codes(&registry, &collections, state_graph_codes())?;

    let output = serde_json::to_string_pretty(&StateGraphCheckReportDocument {
        status: "ok".to_owned(),
        state_graph_id: graph.state_graph_id,
        state_graph_version: graph.state_graph_version,
        registry_id: graph.registry_ref.registry_id,
        registry_version: graph.registry_ref.registry_version,
        node_count: graph.nodes.len(),
        transition_count: graph.transitions.len(),
        evidence_ref_count: graph.evidence_refs.len(),
        capabilities: graph.capabilities,
    })
    .map_err(|error| {
        format!("RUNE-STATE-900 error serializing state graph check report: {error}")
    })?;
    println!("{output}");
    Ok(())
}

fn state_graph_codes() -> StateGraphValidationCodes {
    StateGraphValidationCodes {
        missing_identity: "RUNE-STATE-001",
        missing_registry_ref: "RUNE-STATE-002",
        unknown_node_descriptor: "RUNE-STATE-003",
        unknown_transition_node: "RUNE-STATE-004",
        unsupported_transition_descriptor: "RUNE-STATE-005",
        live_state_requested: "RUNE-STATE-006",
        invalid_evidence_ref: "RUNE-STATE-007",
        invalid_ownership_ref: "RUNE-STATE-008",
        duplicate_graph_id: "RUNE-STATE-009",
    }
}

fn load_registry_collection_refs(
    registry: &SemanticRegistryDocument,
    registry_fixture: &str,
) -> Result<Vec<DescriptorCollectionDocument>, String> {
    let registry_dir = Path::new(registry_fixture)
        .parent()
        .unwrap_or(Path::new(""));
    let mut collections = Vec::with_capacity(registry.collections.len());
    for collection_ref in &registry.collections {
        let source_path = registry_dir.join(&collection_ref.source_ref);
        let collection =
            read_collection_fixture(source_path.to_string_lossy().as_ref(), "RUNE-REGISTRY-005")?
                .validate_with_codes(
                "RUNE-REGISTRY-005",
                "RUNE-REGISTRY-005",
                "RUNE-REGISTRY-005",
            )?;
        if collection.collection_id != collection_ref.collection_id
            || collection.collection_version != collection_ref.collection_version
        {
            return Err(format!(
                "RUNE-REGISTRY-005 collection source ref mismatch: declared {}@{} but source has {}@{}",
                collection_ref.collection_id,
                collection_ref.collection_version,
                collection.collection_id,
                collection.collection_version
            ));
        }
        collections.push(collection);
    }

    Ok(collections)
}

fn check_registry_catalog_refs(registry: &SemanticRegistryDocument) -> Result<(), String> {
    let profiles = ProfileCatalog::approved();
    for profile_ref in &registry.profiles {
        let Some(profile) = profiles.find(&profile_ref.profile_id) else {
            return Err(format!(
                "RUNE-REGISTRY-006 unknown profile in semantic registry: {}",
                profile_ref.profile_id
            ));
        };
        if profile.profile_version != profile_ref.profile_version {
            return Err(format!(
                "RUNE-REGISTRY-006 unsupported profile version in semantic registry: {}@{}",
                profile_ref.profile_id, profile_ref.profile_version
            ));
        }
    }

    let adapters = AdapterCatalog::approved();
    for adapter_ref in &registry.adapters {
        let Some(adapter) = adapters.find(&adapter_ref.adapter_id) else {
            return Err(format!(
                "RUNE-REGISTRY-006 unknown adapter in semantic registry: {}",
                adapter_ref.adapter_id
            ));
        };
        if adapter.adapter_version != adapter_ref.adapter_version {
            return Err(format!(
                "RUNE-REGISTRY-006 unsupported adapter version in semantic registry: {}@{}",
                adapter_ref.adapter_id, adapter_ref.adapter_version
            ));
        }
    }

    Ok(())
}

fn generate(args: Vec<String>) -> Result<(), String> {
    let options = parse_generate_args(&args)?;
    let catalog = ProfileCatalog::approved();
    let Some(profile) = catalog.find(&options.profile) else {
        return Err(format!(
            "RUNE-GEN-003 unsupported profile: {}",
            options.profile
        ));
    };

    let descriptor = read_descriptor_fixture(&options.fixture, "RUNE-GEN-900")?;
    let descriptor =
        descriptor.validate_with_codes("RUNE-GEN-001", "RUNE-GEN-002", "RUNE-GEN-003")?;
    profile.validate_descriptor_with_codes(
        &descriptor,
        ProfileCompatibilityCodes {
            unsupported_version: "RUNE-GEN-007",
            unsupported_kind: "RUNE-GEN-003",
            unsupported_invariant: "RUNE-GEN-004",
            unsupported_trace_link: "RUNE-GEN-005",
            unsupported_extension: "RUNE-GEN-006",
        },
    )?;
    let output = if profile.profile_id == "rune.documentation_packet_json" {
        serde_json::to_string_pretty(&DocumentationPacketDocument::from_descriptor(
            profile, descriptor,
        ))
    } else if profile.profile_id == "rune.data_contract_json" {
        serde_json::to_string_pretty(&DataContractDocument::from_descriptor(profile, descriptor))
    } else {
        serde_json::to_string_pretty(&GeneratedArtifactDocument {
            profile_id: profile.profile_id.clone(),
            profile_version: profile.profile_version.clone(),
            output_artifact_kind: profile.output_artifact_kind.clone(),
            descriptor,
        })
    }
    .map_err(|error| format!("RUNE-GEN-900 error serializing generated artifact: {error}"))?;
    println!("{output}");
    Ok(())
}

fn generate_collection(args: Vec<String>) -> Result<(), String> {
    let options = parse_profile_fixture_args(
        &args,
        "usage: rune generate-collection --profile rune.neutral_descriptor_json --fixture <path>",
    )?;
    let catalog = ProfileCatalog::approved();
    let Some(profile) = catalog.find(&options.profile) else {
        return Err(format!(
            "RUNE-COLL-GEN-003 unsupported profile: {}",
            options.profile
        ));
    };

    let collection = read_collection_fixture(&options.fixture, "RUNE-COLL-GEN-900")?;
    let collection = collection.validate_with_codes(
        "RUNE-COLL-GEN-001",
        "RUNE-COLL-GEN-002",
        "RUNE-COLL-GEN-008",
    )?;
    for descriptor in &collection.descriptors {
        profile.validate_descriptor_with_codes(
            descriptor,
            ProfileCompatibilityCodes {
                unsupported_version: "RUNE-COLL-GEN-007",
                unsupported_kind: "RUNE-COLL-GEN-003",
                unsupported_invariant: "RUNE-COLL-GEN-004",
                unsupported_trace_link: "RUNE-COLL-GEN-005",
                unsupported_extension: "RUNE-COLL-GEN-006",
            },
        )?;
    }

    let output = if profile.profile_id == "rune.documentation_packet_json" {
        serde_json::to_string_pretty(&DocumentationPacketDocument::from_collection(
            profile, collection,
        ))
    } else if profile.profile_id == "rune.data_contract_json" {
        serde_json::to_string_pretty(&DataContractDocument::from_collection(profile, collection))
    } else {
        serde_json::to_string_pretty(&GeneratedCollectionArtifactDocument {
            profile_id: profile.profile_id.clone(),
            profile_version: profile.profile_version.clone(),
            output_artifact_kind: profile.output_artifact_kind.clone(),
            collection,
        })
    }
    .map_err(|error| {
        format!("RUNE-COLL-GEN-900 error serializing generated collection artifact: {error}")
    })?;
    println!("{output}");
    Ok(())
}

fn check(args: Vec<String>) -> Result<(), String> {
    let options = parse_profile_fixture_args(
        &args,
        "usage: rune check --profile rune.neutral_descriptor_json --fixture <path>",
    )?;
    let catalog = ProfileCatalog::approved();
    let Some(profile) = catalog.find(&options.profile) else {
        return Err(format!(
            "RUNE-CHECK-003 unsupported profile: {}",
            options.profile
        ));
    };

    let descriptor = read_descriptor_fixture(&options.fixture, "RUNE-CHECK-900")?;
    let descriptor =
        descriptor.validate_with_codes("RUNE-CHECK-001", "RUNE-CHECK-002", "RUNE-CHECK-003")?;
    profile.validate_descriptor_with_codes(
        &descriptor,
        ProfileCompatibilityCodes {
            unsupported_version: "RUNE-CHECK-007",
            unsupported_kind: "RUNE-CHECK-003",
            unsupported_invariant: "RUNE-CHECK-004",
            unsupported_trace_link: "RUNE-CHECK-005",
            unsupported_extension: "RUNE-CHECK-006",
        },
    )?;

    let output =
        serde_json::to_string_pretty(&CheckReportDocument::compatible(profile, &descriptor))
            .map_err(|error| format!("RUNE-CHECK-900 error serializing check report: {error}"))?;
    println!("{output}");
    Ok(())
}

fn profile(args: Vec<String>) -> Result<(), String> {
    match args.as_slice() {
        [subcommand] if subcommand == "list" => {
            let output =
                serde_json::to_string_pretty(&ProfileCatalog::approved()).map_err(|error| {
                    format!("RUNE-PROF-900 error serializing profile list: {error}")
                })?;
            println!("{output}");
            Ok(())
        }
        _ => Err("usage: rune profile list".to_owned()),
    }
}

fn adapter(args: Vec<String>) -> Result<(), String> {
    match args.as_slice() {
        [subcommand] if subcommand == "list" => {
            let output =
                serde_json::to_string_pretty(&AdapterCatalog::approved()).map_err(|error| {
                    format!("RUNE-ADAPT-900 error serializing adapter list: {error}")
                })?;
            println!("{output}");
            Ok(())
        }
        _ => Err("usage: rune adapter list".to_owned()),
    }
}

fn parse_fixture_arg(args: &[String], usage: &'static str) -> Result<String, String> {
    match args {
        [flag, value] if flag == "--fixture" => Ok(value.clone()),
        _ => Err(usage.to_owned()),
    }
}

fn parse_manifest_arg(args: &[String]) -> Result<String, String> {
    match args {
        [flag, value] if flag == "--manifest" => Ok(value.clone()),
        _ => Err("usage: rune discover --manifest <path>".to_owned()),
    }
}

fn parse_evidence_collection_args(args: &[String]) -> Result<EvidenceCollectionOptions, String> {
    match args {
        [profile_flag, profile, fixture_flag, fixture]
            if profile_flag == "--profile" && fixture_flag == "--fixture" =>
        {
            Ok(EvidenceCollectionOptions {
                profile: profile.clone(),
                source_kind: "descriptor_collection_fixture".to_owned(),
                source: fixture.clone(),
            })
        }
        [profile_flag, profile, manifest_flag, manifest]
            if profile_flag == "--profile" && manifest_flag == "--manifest" =>
        {
            Ok(EvidenceCollectionOptions {
                profile: profile.clone(),
                source_kind: "discovery_manifest".to_owned(),
                source: manifest.clone(),
            })
        }
        _ => Err(
            "usage: rune evidence-collection --profile rune.neutral_descriptor_json (--fixture <path> | --manifest <path>)"
                .to_owned(),
        ),
    }
}

fn evidence_source_label(source: &str) -> String {
    Path::new(source)
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or(source)
        .to_owned()
}

struct ProfileFixtureOptions {
    profile: String,
    fixture: String,
}

struct AdapterFixtureOptions {
    adapter: String,
    fixture: String,
}

struct EvidenceCollectionOptions {
    profile: String,
    source_kind: String,
    source: String,
}

struct StateGraphOptions {
    fixture: String,
    registry: String,
}

fn parse_state_graph_args(args: &[String]) -> Result<StateGraphOptions, String> {
    match args {
        [fixture_flag, fixture, registry_flag, registry]
            if fixture_flag == "--fixture" && registry_flag == "--registry" =>
        {
            Ok(StateGraphOptions {
                fixture: fixture.clone(),
                registry: registry.clone(),
            })
        }
        _ => Err("usage: rune check-state-graph --fixture <path> --registry <path>".to_owned()),
    }
}

fn parse_generate_args(args: &[String]) -> Result<ProfileFixtureOptions, String> {
    parse_profile_fixture_args(
        args,
        "usage: rune generate --profile <profile-id> --fixture <path>",
    )
}

fn parse_profile_fixture_args(
    args: &[String],
    usage: &'static str,
) -> Result<ProfileFixtureOptions, String> {
    match args {
        [profile_flag, profile, fixture_flag, fixture]
            if profile_flag == "--profile" && fixture_flag == "--fixture" =>
        {
            Ok(ProfileFixtureOptions {
                profile: profile.clone(),
                fixture: fixture.clone(),
            })
        }
        _ => Err(usage.to_owned()),
    }
}

fn parse_adapter_fixture_args(
    args: &[String],
    usage: &'static str,
) -> Result<AdapterFixtureOptions, String> {
    match args {
        [adapter_flag, adapter, fixture_flag, fixture]
            if adapter_flag == "--adapter" && fixture_flag == "--fixture" =>
        {
            Ok(AdapterFixtureOptions {
                adapter: adapter.clone(),
                fixture: fixture.clone(),
            })
        }
        _ => Err(usage.to_owned()),
    }
}

fn read_descriptor_fixture(path: &str, io_code: &'static str) -> Result<DescriptorDraft, String> {
    let content = std::fs::read_to_string(Path::new(path))
        .map_err(|error| format!("{io_code} error reading fixture: {error}"))?;
    serde_json::from_str(&content)
        .map_err(|error| format!("{io_code} error parsing fixture JSON: {error}"))
}

fn read_collection_fixture(
    path: &str,
    io_code: &'static str,
) -> Result<DescriptorCollectionDraft, String> {
    let content = std::fs::read_to_string(Path::new(path))
        .map_err(|error| format!("{io_code} error reading fixture: {error}"))?;
    serde_json::from_str(&content)
        .map_err(|error| format!("{io_code} error parsing fixture JSON: {error}"))
}

fn read_discovery_manifest(
    path: &str,
    io_code: &'static str,
) -> Result<DiscoveryManifestDraft, String> {
    let content = std::fs::read_to_string(Path::new(path))
        .map_err(|error| format!("{io_code} error reading discovery manifest: {error}"))?;
    serde_json::from_str(&content)
        .map_err(|error| format!("{io_code} error parsing discovery manifest JSON: {error}"))
}

fn read_semantic_registry_fixture(
    path: &str,
    io_code: &'static str,
) -> Result<SemanticRegistryDraft, String> {
    let content = std::fs::read_to_string(Path::new(path))
        .map_err(|error| format!("{io_code} error reading semantic registry fixture: {error}"))?;
    serde_json::from_str(&content)
        .map_err(|error| format!("{io_code} error parsing semantic registry fixture JSON: {error}"))
}

fn read_state_graph_fixture(path: &str, io_code: &'static str) -> Result<StateGraphDraft, String> {
    let content = std::fs::read_to_string(Path::new(path))
        .map_err(|error| format!("{io_code} error reading state graph fixture: {error}"))?;
    serde_json::from_str(&content)
        .map_err(|error| format!("{io_code} error parsing state graph fixture JSON: {error}"))
}
