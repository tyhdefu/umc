use std::io;

use crate::Program;
use crate::binary::{DecodeError, EncodeError};

pub fn encode<W: io::Write>(program: &Program, dst: W) -> Result<(), EncodeError> {
    todo!()
}

pub fn decode<R: io::Read>(src: R) -> Result<Program, DecodeError> {
    todo!()
}
