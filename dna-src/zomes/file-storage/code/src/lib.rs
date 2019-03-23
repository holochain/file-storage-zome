#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

mod chunk;
mod manifest;

define_zome! {
    entries: [
        chunk::def(),
        manifest::def()
    ]

    genesis: || { Ok(()) }

    functions: [
    ]

    traits: {
        hc_public []
    }
}
