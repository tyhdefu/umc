//! Translated between binary encodings to the checked instruction form and vice versa
//!

use std::io;

use crate::Program;

mod v0;

pub enum EncodeError {
    WriteError(io::Error),
}

pub enum DecodeError {
    /// Failed to read (enough) bytes from the source to parse the bytes
    ReadError(io::Error),
    /// The UMC Header was not present or was malformed
    BadHeader,
    /// The UMC Header contained a version that is not supported by the decoder
    UnsupportedVersion(BinaryFormatVersion),
}

/// Encode the program into a binary form and write to the destination
pub fn encode<W: io::Write>(p: &Program, dst: W) -> Result<(), EncodeError> {
    v0::encode(p, dst)
}

/// Decode a binary compiled UMC into a checked program
pub fn decode<R: io::Read>(src: R) -> Result<Program, DecodeError> {
    todo!()
}

pub const MAGIC_NUMBER: &[u8; 16] = b"UMC Bytecode\0\0\0\0";

pub struct BinaryFormatVersion {
    major: u8,
    minor: u8,
}

struct BinaryHeader {
    version: BinaryFormatVersion,
    // Syscall table version?
}

pub trait BinaryEncodable {}
