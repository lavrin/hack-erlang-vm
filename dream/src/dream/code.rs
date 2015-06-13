use std;
use super::beam;

#[derive(Debug)]
pub struct CodeChunk {
    pub id:                 String,
    pub len:                u32,

    // `info_fields_len` is at least 16 (4 bytes for each of instruction_set,
    // opcode_max, n_labels, n_functions), though might be more.
    // `code` starts at an offset of `.instruction_set` + `info_fields_len`.
    pub info_fields_len:    u32,

    pub instruction_set:    u32,
    pub opcode_max:         u32,
    pub n_labels:           u32,
    pub n_functions:        u32,

    // Possibly more data here depending on `info_fields_len` value.

    pub code:               Vec<u8>
}

#[derive(Debug)]
pub enum Error {
    UnexpectedChunk(String, String),
    InvalidChunk
}

fn unexpected_chunk(expected: &str, got: &str) -> Result<CodeChunk, Error> {
    Err ( Error::UnexpectedChunk(expected.to_string(), got.to_string()) )
}

impl CodeChunk {

    pub fn from_chunk(chunk: &beam::Chunk) -> Result<CodeChunk, Error> {
        if chunk.id != "Code"
            { return unexpected_chunk("Code", &chunk.id) }
        // Fields from `info_fields_len` to `n_functions` must be present!
        if chunk.data.len() < 5 * std::mem::size_of::<u32>()
            { return Err ( Error::InvalidChunk ) }
        Ok (CodeChunk {
                id: chunk.id.clone(),
                len: chunk.len,
                info_fields_len: u32_from_be(&chunk.data[0..4]),
                instruction_set: u32_from_be(&chunk.data[4..8]),
                opcode_max: u32_from_be(&chunk.data[8..12]),
                n_labels: u32_from_be(&chunk.data[12..16]),
                n_functions: u32_from_be(&chunk.data[16..20]),
                code: chunk.data[20..].to_vec()
        })
    }

}

// This is funny.
// Stable Rust doesn't allow to use #![feature(core)]:
//
//    this feature may not be used in the stable release channel
//
// which is needed to access std::raw::Slice (the slice implementation struct).
// This struct is basically a pair of a raw pointer to data and data length.
// This would allow for the following implementation:
//
//     fn u32_from_be(bytes: &[u8]) -> u32 {
//         let bytes: std::raw::Slice<u8> = unsafe { std::mem::transmute(&bytes) };
//         let _u32: *const u32 = bytes.data as *const u32;
//         u32::from_be(unsafe { *_u32 })
//     }
//
fn u32_from_be(bytes: &[u8]) -> u32 {
    if bytes.len() != 4 { panic!("expected 4 bytes") }
    let mut _u32: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
    u32::from_be(unsafe { *(&_u32 as *const u8 as *const u32) })
}
