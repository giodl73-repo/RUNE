use rune_core::RuneContract;
use rune_derive::RuneContract as DeriveRuneContract;

#[derive(DeriveRuneContract)]
#[rune(
    id = "example.event",
    version = "v0",
    kind = "event",
    requirement = "RUNE-REQ-013"
)]
struct ExampleEvent {
    id: String,
}

fn main() {
    let descriptor = ExampleEvent::descriptor();
    assert_eq!(descriptor.id, "example.event");
    assert_eq!(descriptor.trace_links[0].target, "RUNE-REQ-013");
}
