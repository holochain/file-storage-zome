use hdk::entry_definition::ValidatingEntryType;
use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
    json::JsonString,
    error::HolochainError,
};

#[derive(Debug, Serialize, Deserialize, DefaultJson)]
pub struct Chunk {

}

pub fn def() -> ValidatingEntryType {
    entry!(
        name: "manifest",
        description: "",
        sharing: Sharing::Public,
        native_type: Chunk,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_name: Chunk, _ctx: hdk::ValidationData| {
            Ok(())
        },

        links: [
        ]
    )
}
