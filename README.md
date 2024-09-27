**NB:** This is an out of date zome, please see the [holochain-open-dev/file-storage repo](https://github.com/holochain-open-dev/file-storage)

# file-storage-zome
A zome that can be mixed in to any DNA to provide basic file storage capabilities. 

This aims to be a community maintained project and will receive support but minimal contributions from the dev team 

## Requirements

There are several considerations for an efficient implementation of file/blob storage:
- There is a limit of 4GB of WASM memory that can be used at any time
- Files must be chunked to provide uniform data distribution in the DHT
- There must be a way to validate that the file you receive is as expected

## Specification

Files should be chunked outside the zome itself. A javascript reference implementation should be included in this repo which performs the chunking and restructuring operations

A manifest should describe a single file and how to retrieve all of the chunks. The hash of the manifest therefore becomes the address for retrieving the file.
The manifest must contain:

- An ordered list of the hashes/addresses of the chunks
- Any additional metadata (e.g. description, file type etc)

validation logic must ensure that a manifest can only be stored in the DHT once all of the chunks are also stored

---

The zome should expose the following interface:

- `store_chunk(data: RawString) -> ZomeApiResult<Address>`
- `store_manifest(manifest: Manifest) -> ZomeApiResult<Address>`
- `get_chunk(address: Address) -> ZomeApiResult<RawString>`
- `get_manifest(address: Address) -> ZomeApiResult<Manifest>`

The javascript interface should expose higher level functions to the end user such as :

- `storeFile(bytes: UInt8Array): string | Error` - returns the address of the manifest as a string
- `getFile(address: string): UInt8Array | Error` - given the manifest address returns the restructured chunks

## Contributing

We follow the "fork-and-pull" Git workflow.

    Fork the repo on GitHub
    Clone the project to your own machine
    Commit changes to your own branch
    Push your work back up to your fork
    Submit a Pull request so that we can review your changes

NOTE: Be sure to merge the latest from "upstream" before making a pull request
