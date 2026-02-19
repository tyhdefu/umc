use std::io::{self, ErrorKind};

pub trait LEBEncodable: Sized {
    fn encode_leb128<W: io::Write>(self, writer: &mut W) -> io::Result<usize>;

    fn decode_leb128<R: io::Read>(reader: &mut R) -> io::Result<Self>;
}

impl LEBEncodable for u64 {
    fn encode_leb128<W: io::Write>(self, writer: &mut W) -> io::Result<usize> {
        leb128::write::unsigned(writer, self)
    }

    fn decode_leb128<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        match leb128::read::unsigned(reader) {
            Ok(x) => Ok(x),
            Err(leb128::read::Error::IoError(e)) => Err(e),
            Err(leb128::read::Error::Overflow) => Err(ErrorKind::InvalidData.into()),
        }
    }
}

impl LEBEncodable for u32 {
    fn encode_leb128<W: io::Write>(self, writer: &mut W) -> io::Result<usize> {
        (self as u64).encode_leb128(writer)
    }

    fn decode_leb128<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        u64::decode_leb128(reader)?
            .try_into()
            .map_err(|_| ErrorKind::InvalidData.into())
    }
}
