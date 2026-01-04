//! Translated between binary encodings to the checked instruction form and vice versa
//!

use std::io;

use crate::{Program, parse::InstructionValidateError};

#[cfg(test)]
mod test;
mod v0;

#[derive(Debug)]
pub enum EncodeError {
    WriteError(io::Error),
}

impl From<io::Error> for EncodeError {
    fn from(value: io::Error) -> Self {
        Self::WriteError(value)
    }
}

#[derive(Debug)]
pub enum DecodeError {
    /// Failed to read (enough) bytes from the source to parse the bytes
    ReadError(io::Error),
    /// The UMC Header was not present or was malformed
    BadHeader,
    /// Some implementation-defined error in the bytecode
    Malformed(String),
    /// The UMC Header contained a version that is not supported by the decoder
    UnsupportedVersion(BinaryFormatVersion),
}

impl From<io::Error> for DecodeError {
    fn from(value: io::Error) -> Self {
        Self::ReadError(value)
    }
}

impl From<InstructionValidateError> for DecodeError {
    fn from(value: InstructionValidateError) -> Self {
        Self::Malformed(format!("Invalid Instruction: {:?}", value))
    }
}

/// Encode the program into a binary form and write to the destination
pub fn encode<W: io::Write>(p: &Program, dst: W) -> Result<(), EncodeError> {
    v0::encode(p, dst)
}

/// Decode a binary compiled UMC into a checked program
pub fn decode<R: io::Read>(src: R) -> Result<Program, DecodeError> {
    v0::decode(src)
}

pub const MAGIC_NUMBER: &[u8; 16] = b"UMC Bytecode\0\0\0\0";

#[derive(Debug)]
pub struct BinaryFormatVersion {
    major: u8,
    minor: u8,
}

struct BinaryHeader {
    version: BinaryFormatVersion,
    // Syscall table version?
}

pub trait BinaryEncodable {}
