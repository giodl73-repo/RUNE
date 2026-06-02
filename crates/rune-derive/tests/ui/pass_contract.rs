use rune_core::RuneContract;
use rune_derive::RuneContract as DeriveRuneContract;

#[derive(DeriveRuneContract)]
#[rune(
    id = "example.event",
    version = "v0",
    kind = "event",
    requirement = "RUNE-REQ-013",
    invariant(id = "example.event.id.present", text = "id must be present"),
    extension(namespace = "example", name = "formula", value = "n/a")
)]
struct ExampleEvent {
    #[rune_field(
        required = true,
        unit = "event-id",
        min = "1",
        max = "64",
        sensitivity = "public",
        example = "evt_123",
        stability = "stable",
        alias = "event_id"
    )]
    id: String,
}

fn main() {
    let descriptor = ExampleEvent::descriptor();
    assert_eq!(descriptor.id, "example.event");
    assert_eq!(descriptor.fields[0].metadata.required, Some(true));
    assert_eq!(descriptor.fields[0].metadata.aliases[0], "event_id");
    assert_eq!(descriptor.invariants[0].id, "example.event.id.present");
    assert_eq!(descriptor.trace_links[0].target, "RUNE-REQ-013");
    assert_eq!(descriptor.extensions[0].namespace, "example");
}
