use rune_derive::RuneContract as DeriveRuneContract;

#[derive(DeriveRuneContract)]
#[rune(schema = "example.old")]
struct OldStyleContract {
    id: String,
}

fn main() {}
