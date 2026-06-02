use rune_derive::RuneContract as DeriveRuneContract;

#[derive(DeriveRuneContract)]
#[rune(id = "example.field", version = "v0")]
struct ExampleField {
    #[rune_field(format = "legacy")]
    value: String,
}

fn main() {}
