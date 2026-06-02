use rune_core::{ContractRegistration, DescriptorCollectionDocument, RuneContract};
use rune_derive::RuneContract as DeriveRuneContract;

pub const COLLECTION_ID: &str = "example.adopter_contracts";
pub const COLLECTION_VERSION: &str = "v0";

#[derive(DeriveRuneContract)]
#[rune(
    id = "example.adopter.account",
    version = "v0",
    kind = "entity",
    requirement = "RUNE-REQ-060"
)]
pub struct Account {
    pub id: String,
    pub email: String,
}

#[derive(DeriveRuneContract)]
#[rune(
    id = "example.adopter.open_account",
    version = "v0",
    kind = "command",
    requirement = "RUNE-REQ-060"
)]
pub struct OpenAccount {
    pub account_id: String,
    pub email: String,
}

pub const RUNE_CONTRACTS: &[ContractRegistration] = &[
    ContractRegistration {
        name: "Account",
        descriptor: Account::descriptor,
    },
    ContractRegistration {
        name: "OpenAccount",
        descriptor: OpenAccount::descriptor,
    },
];

pub fn descriptor_collection() -> Result<DescriptorCollectionDocument, String> {
    DescriptorCollectionDocument::from_registrations(
        COLLECTION_ID,
        COLLECTION_VERSION,
        RUNE_CONTRACTS,
        "RUNE-REG-001",
    )
}
