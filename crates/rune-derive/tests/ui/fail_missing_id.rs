use rune_derive::RuneContract as DeriveRuneContract;

#[derive(DeriveRuneContract)]
#[rune(version = "v0", kind = "entity")]
struct MissingIdContract {
    id: String,
}

fn main() {}
