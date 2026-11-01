//! Bounded consumer of existing RUNE metadata contracts; no runtime authority.
use rune_adapters::{AdapterCatalog, AdapterCompatibilityCodes, ReviewPacketDocument};
use rune_core::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub const MAX_REQUEST: usize = 262_144;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    kind: String,
    profile: String,
    input_json: String,
}

fn parse<T: for<'a> Deserialize<'a>>(text: &str) -> Result<(T, Vec<String>), String> {
    let mut ignored = Vec::new();
    let mut de = serde_json::Deserializer::from_str(text);
    let doc = serde_ignored::deserialize(&mut de, |path| ignored.push(path.to_string()))
        .map_err(|e| format!("RUNE-WEB-JSON {e}"))?;
    de.end().map_err(|e| format!("RUNE-WEB-JSON {e}"))?;
    Ok((doc, ignored))
}

fn encode(doc: &impl Serialize) -> Result<String, String> {
    serde_json::to_string_pretty(doc).map_err(|e| format!("RUNE-WEB-SERIALIZE {e}"))
}

fn validate_descriptor(draft: DescriptorDraft) -> Result<DescriptorDocument, String> {
    draft.validate_with_codes("RUNE-WEB-ID", "RUNE-WEB-VERSION", "RUNE-WEB-KIND")
}

fn profile_check(profile: &ProfileDescriptor, doc: &DescriptorDocument) -> Result<(), String> {
    profile.validate_descriptor_with_codes(
        doc,
        ProfileCompatibilityCodes {
            unsupported_version: "RUNE-WEB-PROFILE-VERSION",
            unsupported_kind: "RUNE-WEB-PROFILE-KIND",
            unsupported_invariant: "RUNE-WEB-PROFILE-INVARIANT",
            unsupported_trace_link: "RUNE-WEB-PROFILE-TRACE",
            unsupported_extension: "RUNE-WEB-PROFILE-EXTENSION",
        },
    )
}

fn work(text: &str) -> Result<Value, String> {
    if text.len() > MAX_REQUEST {
        return Err("RUNE-WEB-LIMIT request exceeds 256 KiB UTF-8".into());
    }
    let req: Request = serde_json::from_str(text).map_err(|e| format!("RUNE-WEB-REQUEST {e}"))?;
    if req.input_json.len() > 131_072 {
        return Err("RUNE-WEB-LIMIT input exceeds 128 KiB UTF-8".into());
    }
    let catalog = ProfileCatalog::approved();
    let profile = catalog
        .find(&req.profile)
        .ok_or("RUNE-WEB-PROFILE unknown approved profile")?;
    let (normalized, ignored, check, artifact, inventory, review) = match req.kind.as_str() {
        "descriptor" => {
            let (draft, ignored) = parse::<DescriptorDraft>(&req.input_json)?;
            let doc = validate_descriptor(draft)?;
            profile_check(profile, &doc)?;
            let artifact = match profile.profile_id.as_str() {
                "rune.documentation_packet_json" => encode(
                    &DocumentationPacketDocument::from_descriptor(profile, doc.clone()),
                )?,
                "rune.data_contract_json" => {
                    encode(&DataContractDocument::from_descriptor(profile, doc.clone()))?
                }
                _ => encode(&GeneratedArtifactDocument {
                    profile_id: profile.profile_id.clone(),
                    profile_version: profile.profile_version.clone(),
                    output_artifact_kind: profile.output_artifact_kind.clone(),
                    descriptor: doc.clone(),
                })?,
            };
            (
                encode(&doc)?,
                ignored,
                encode(&CheckReportDocument::compatible(profile, &doc))?,
                artifact,
                None,
                json!({"status":"not-applicable","message":"Review adapter accepts collections only."}),
            )
        }
        "collection" => {
            let (draft, mut ignored) = parse::<DescriptorCollectionDraft>(&req.input_json)?;
            if draft.descriptors.len() > 128 {
                return Err("RUNE-WEB-LIMIT collection exceeds 128 descriptors".into());
            }
            let doc = draft.validate_with_codes(
                "RUNE-WEB-COLLECTION-ID",
                "RUNE-WEB-COLLECTION-VERSION",
                "RUNE-WEB-DUPLICATE",
            )?;
            // CollectionDraft alone checks collection identity/duplicates. Recheck each
            // member through DescriptorDraft before claiming descriptor compatibility.
            let raw: Value = serde_json::from_str(&req.input_json).map_err(|e| e.to_string())?;
            for (i, item) in raw["descriptors"]
                .as_array()
                .into_iter()
                .flatten()
                .enumerate()
            {
                let (member, member_ignored) = parse::<DescriptorDraft>(&item.to_string())?;
                ignored.extend(
                    member_ignored
                        .into_iter()
                        .map(|p| format!("descriptors.{i}.{p}")),
                );
                validate_descriptor(member).map_err(|e| format!("descriptors.{i}: {e}"))?;
            }
            for member in &doc.descriptors {
                profile_check(profile, member)?;
            }
            ignored.sort();
            ignored.dedup();
            let artifact = match profile.profile_id.as_str() {
                "rune.documentation_packet_json" => encode(
                    &DocumentationPacketDocument::from_collection(profile, doc.clone()),
                )?,
                "rune.data_contract_json" => {
                    encode(&DataContractDocument::from_collection(profile, doc.clone()))?
                }
                _ => encode(&GeneratedCollectionArtifactDocument {
                    profile_id: profile.profile_id.clone(),
                    profile_version: profile.profile_version.clone(),
                    output_artifact_kind: profile.output_artifact_kind.clone(),
                    collection: doc.clone(),
                })?,
            };
            let adapter_catalog = AdapterCatalog::approved();
            let adapter = adapter_catalog
                .find("rune.review_packet_json")
                .ok_or("RUNE-WEB-ADAPTER unavailable")?;
            let review = match adapter.validate_collection_with_codes(
                &doc,
                AdapterCompatibilityCodes {
                    unsupported_kind: "RUNE-WEB-ADAPTER-KIND",
                    unsupported_invariant: "RUNE-WEB-ADAPTER-INVARIANT",
                    unsupported_trace_link: "RUNE-WEB-ADAPTER-TRACE",
                    unsupported_extension: "RUNE-WEB-ADAPTER-EXTENSION",
                },
            ) {
                Ok(()) => {
                    json!({"status":"compatible","artifact_json":encode(&ReviewPacketDocument::from_collection(adapter, &doc))?})
                }
                Err(message) => json!({"status":"unsupported","message":message}),
            };
            (
                encode(&doc)?,
                ignored,
                encode(&CheckCollectionReportDocument::compatible(profile, &doc))?,
                artifact,
                Some(encode(&doc.inventory())?),
                review,
            )
        }
        _ => return Err("RUNE-WEB-KIND choose descriptor or collection".into()),
    };
    Ok(
        json!({"schema":"rune.browser-report.v1","ok":true,"kind":req.kind,"profile":req.profile,
        "input_json":req.input_json,"normalized_json":normalized,"ignored_fields":ignored,"check_json":check,
        "artifact_json":artifact,"inventory_json":inventory,"review":review,
        "boundary":"Metadata and profile compatibility only. Field constraints and invariant text are retained declarations, not executed checks. No Rust compilation, source scanning, runtime inspection, authorization, mutation, or host execution.",
        "json_policy":"Duplicate known object keys are rejected by typed deserialization. Unknown fields are ignored and listed; duplicate unknown keys may be listed once. Original input is retained separately; normalized output contains only recognized fields."}),
    )
}

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn replay(text: &str) -> String {
    match work(text) {
        Ok(value) => value.to_string(),
        Err(message) => {
            json!({"schema":"rune.browser-report.v1","ok":false,"diagnostic":{"message":message}})
                .to_string()
        }
    }
}

pub fn examples() -> String {
    let entity = json!({"id":"sample.entity","version":"v0","kind":"entity","rust_type":"SampleEntity","fields":[{"name":"count","rust_type":"u64","metadata":{"required":true,"min":"0","max":"18446744073709551615"}}],"invariants":[],"trace_links":[{"relation":"requirement","target":"SAMPLE-01"}],"extensions":[]});
    let mut annotated = entity.clone();
    annotated["invariants"] = json!([{"id":"SAMPLE-INV-01","text":"Count is retained metadata; this text is not executed."}]);
    let collection = json!({"collection_id":"sample.collection","collection_version":"v0","descriptors":[entity.clone(),json!({"id":"sample.event","version":"v0","kind":"event","rust_type":"SampleEvent","fields":[],"invariants":[],"trace_links":[],"extensions":[]})]});
    let mut with_invariant = collection.clone();
    with_invariant["descriptors"][0] = annotated.clone();
    let mut unknown = entity.clone();
    unknown["unrecognized"] = json!({"exact_integer":18446744073709551615_u64});
    let mut examples = serde_json::Map::new();
    for (name, kind, profile, input) in [
        (
            "descriptor",
            "descriptor",
            "rune.neutral_descriptor_json",
            entity.clone(),
        ),
        (
            "documentation",
            "descriptor",
            "rune.documentation_packet_json",
            annotated,
        ),
        (
            "data-contract",
            "descriptor",
            "rune.data_contract_json",
            entity,
        ),
        (
            "collection",
            "collection",
            "rune.neutral_descriptor_json",
            collection,
        ),
        (
            "adapter-limit",
            "collection",
            "rune.documentation_packet_json",
            with_invariant,
        ),
        (
            "unknown-fields",
            "descriptor",
            "rune.neutral_descriptor_json",
            unknown,
        ),
    ] {
        examples.insert(
            name.into(),
            json!({"kind":kind,"profile":profile,"input_json":encode(&input).unwrap()}),
        );
    }
    Value::Object(examples).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn example(name: &str) -> Value {
        serde_json::from_str::<Value>(&examples()).unwrap()[name].clone()
    }
    fn result(req: Value) -> Value {
        serde_json::from_str(&replay(&req.to_string())).unwrap()
    }
    #[test]
    fn all_examples_generate() {
        for (_, req) in serde_json::from_str::<Value>(&examples())
            .unwrap()
            .as_object()
            .unwrap()
        {
            assert_eq!(result(req.clone())["ok"], true);
        }
    }
    #[test]
    fn adapter_fails_without_discarding_profile_output() {
        let r = result(example("adapter-limit"));
        assert_eq!(r["review"]["status"], "unsupported");
        assert!(
            r["artifact_json"]
                .as_str()
                .unwrap()
                .contains("sample.collection")
        );
    }
    #[test]
    fn exact_input_and_unknown_disclosure() {
        let mut req = example("unknown-fields");
        let raw = req["input_json"].as_str().unwrap().replace('\n', "\r\n");
        req["input_json"] = json!(raw);
        let r = result(req);
        assert_eq!(r["input_json"], raw);
        assert_eq!(r["ignored_fields"], json!(["unrecognized"]));
        assert!(
            r["input_json"]
                .as_str()
                .unwrap()
                .contains("18446744073709551615")
        );
    }
    #[test]
    fn collection_member_identity_checked() {
        let mut req = example("collection");
        let mut c: Value = serde_json::from_str(req["input_json"].as_str().unwrap()).unwrap();
        c["descriptors"][0]["id"] = json!("");
        req["input_json"] = json!(c.to_string());
        assert_eq!(result(req)["ok"], false);
    }
    #[test]
    fn unsupported_version_fails() {
        let mut req = example("descriptor");
        req["input_json"] = json!(req["input_json"].as_str().unwrap().replace("v0", "v99"));
        assert!(
            result(req)["diagnostic"]["message"]
                .as_str()
                .unwrap()
                .contains("PROFILE-VERSION")
        );
    }
    #[test]
    fn duplicate_known_keys_fail() {
        let mut req = example("descriptor");
        req["input_json"] = json!("{\"id\":\"a\",\"id\":\"b\"}");
        assert_eq!(result(req)["ok"], false);
    }
    #[test]
    fn limits_and_bad_json_fail() {
        assert!(replay(&" ".repeat(MAX_REQUEST + 1)).contains("LIMIT"));
        assert!(replay("{broken").contains("REQUEST"));
    }
}
