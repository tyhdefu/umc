use std::fmt::{Debug, Display};

use crate::vm::types::UMCOffset;
use crate::vm::types::uint::ArbitraryUnsignedInt;

pub mod safe;

#[derive(Debug)]
pub struct AllocateError {
    requested_bytes: usize,
}

impl Display for AllocateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to allocate {} bytes", self.requested_bytes)
    }
}

#[derive(Debug)]
pub enum MemoryAccessError {
    /// The address supplied was either never allocated or already freed
    InvalidAddress,
    /// The address did correspond to some "block" of memory, but the read/write was out of bounds
    OutOfBounds,
}

/// Required trait implementations for a MemoryAddress implementation
pub trait MemoryAddress: UMCOffset + PartialOrd + Clone + Debug {}

/// Manages the virtual memory for the VM
pub trait MemoryManager {
    type Address: MemoryAddress;

    /// Allocates a block of memory with a certain number of bytes
    /// This may fail if the requested number of bytes could not be allocated, or may appear to succeed
    fn allocate(&mut self, bytes: usize) -> Result<Self::Address, AllocateError>;

    /// Free a block of memory
    /// It is only correct to free a memory address once, but implementations may be forgiving
    fn free(&mut self, address: &Self::Address);

    /// Load a primitive value from virtual memory
    /// This may fail if the given address was invalid, or may cause implementation-defined behaviour
    fn load_prim<V: Serializable>(&self, address: &Self::Address) -> Result<V, MemoryAccessError>;

    /// Load a specific bitwidth value from virtual memory
    /// This may fail if the given address was invalid, or may cause implementation-defined behaviour
    fn load<V: SerializableArb>(
        &self,
        bitwidth: usize,
        address: &Self::Address,
    ) -> Result<V, MemoryAccessError>;

    /// Store a primitive value into virtual memory
    /// This may fail if the given address was invalid, or may cause implementation-defined behaviour
    fn store_prim<V: Serializable>(
        &mut self,
        v: V,
        address: &Self::Address,
    ) -> Result<(), MemoryAccessError>;

    /// Load a specific bitwidth value from virtual memory
    /// This may fail if the given address was invalid, or may cause implementation-defined behaviour
    fn store<V: SerializableArb>(
        &mut self,
        v: V,
        address: &Self::Address,
    ) -> Result<(), MemoryAccessError>;
}

pub trait Serializable: Sized {
    /// Read the value from a slice of bytes
    /// Fails if there is not enough bytes
    fn read_from(bytes: &[u8]) -> Result<Self, ()>;

    /// Write the bytes to the byte buffer
    /// Fails if the buffer cannot fit the value
    fn write_to(&self, bytes: &mut [u8]) -> Result<(), ()>;
}

pub trait SerializableArb: Sized {
    /// Reads a given bitwidth value from the slice of bytes
    /// Fails if there is not enough bytes
    fn read_from(bytes: &[u8], bitwidth: usize) -> Result<Self, ()>;

    /// Writes a given value (includes its bitwidth) to the slice of bytes
    /// Fails if the buffer cannot fit the value
    fn write_to(&self, bytes: &mut [u8]) -> Result<(), ()>;
}

// Serializable Implementations

macro_rules! impl_serialize_prim {
    ($p:ty) => {
        impl Serializable for $p {
            fn read_from(bytes: &[u8]) -> Result<Self, ()> {
                let slice = bytes.get(..size_of::<Self>()).ok_or(())?;
                Ok(Self::from_be_bytes(slice.try_into().unwrap()))
            }

            fn write_to(&self, bytes: &mut [u8]) -> Result<(), ()> {
                let slice = bytes.get_mut(..size_of::<Self>()).ok_or(())?;
                slice.clone_from_slice(self.to_be_bytes().as_slice());
                Ok(())
            }
        }
    };
}

impl_serialize_prim!(u32);
impl_serialize_prim!(u64);

impl_serialize_prim!(usize);

impl_serialize_prim!(i32);
impl_serialize_prim!(i64);

impl_serialize_prim!(f32);
impl_serialize_prim!(f64);

impl SerializableArb for ArbitraryUnsignedInt {
    fn read_from(bytes: &[u8], bitwidth: usize) -> Result<Self, ()> {
        ArbitraryUnsignedInt::from_bytes(bitwidth as u32, bytes)
    }

    fn write_to(&self, bytes: &mut [u8]) -> Result<(), ()> {
        self.write_bytes(bytes)
    }
}
