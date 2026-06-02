use rune_derive::RuneContract as DeriveRuneContract;

#[derive(DeriveRuneContract)]
#[rune(id = "example.missing_version", kind = "entity")]
struct MissingVersionContract {
    id: String,
}

fn main() {}
