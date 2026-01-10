//! Translated between binary encodings to the checked instruction form and vice versa
//!

use std::io;

use byteorder::{ReadBytesExt, WriteBytesExt};

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
    /// The magic number found at the start of the file was missing or wrong
    BadMagicNumber,
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
pub fn encode<W: io::Write>(p: &Program, mut dst: W) -> Result<(), EncodeError> {
    let header = BinaryHeader {
        version: v0::VERSION,
    };
    header.write(&mut dst)?;
    v0::encode(p, dst)
}

/// Decode a binary compiled UMC into a checked program
pub fn decode<R: io::Read>(mut src: R) -> Result<Program, DecodeError> {
    let header = BinaryHeader::read(&mut src)?;
    if !header.version.can_be_decoded_by(&v0::VERSION) {
        return Err(DecodeError::UnsupportedVersion(header.version));
    }
    v0::decode(src)
}

#[derive(Debug)]
pub struct BinaryFormatVersion {
    major: u8,
    minor: u8,
}

impl BinaryFormatVersion {
    pub fn can_be_decoded_by(&self, decoder_version: &BinaryFormatVersion) -> bool {
        if decoder_version.major != self.major {
            return true;
        }
        decoder_version.minor >= self.minor
    }
}

struct BinaryHeader {
    version: BinaryFormatVersion,
    // Syscall table version?
}

impl BinaryHeader {
    pub const MAGIC_NUMBER_LEN: usize = 16;
    pub const MAGIC_NUMBER: &[u8; Self::MAGIC_NUMBER_LEN] = b"\x7FUMC Bytecode\0\0\0";

    fn write<W: io::Write>(&self, dst: &mut W) -> Result<(), EncodeError> {
        dst.write_all(Self::MAGIC_NUMBER)?;

        dst.write_u8(self.version.major)?;
        dst.write_u8(self.version.minor)?;

        Ok(())
    }

    fn read<R: io::Read>(src: &mut R) -> Result<BinaryHeader, DecodeError> {
        let mut buf = [0; Self::MAGIC_NUMBER_LEN];
        src.read_exact(&mut buf)?;

        if buf.as_slice() != Self::MAGIC_NUMBER.as_slice() {
            return Err(DecodeError::BadMagicNumber);
        }

        let major = src.read_u8()?;
        let minor = src.read_u8()?;

        let version = BinaryFormatVersion { major, minor };
        Ok(Self { version })
    }
}

pub trait BinaryEncodable {}
