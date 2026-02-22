//! A safe memory manager implementation that catches invalid attempts to read/write out of bounds, or use-after free

use std::usize;

use crate::vm::memory::{
    AllocateError, MemoryAccessError, MemoryAddress, MemoryManager, Serializable, SerializableArb,
};
use crate::vm::types::UMCOffset;

#[derive(Debug)]
pub struct SafeMemoryManager {
    blocks: Vec<Option<SimpleMemoryBlock>>,
}

impl SafeMemoryManager {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    fn get_block(&self, address: &SafeAddress) -> Option<&SimpleMemoryBlock> {
        match self.blocks.get(address.block_id) {
            Some(v) => v.as_ref(),
            None => None,
        }
    }

    fn get_block_mut(&mut self, address: &SafeAddress) -> Option<&mut SimpleMemoryBlock> {
        match self.blocks.get_mut(address.block_id) {
            Some(v) => v.as_mut(),
            None => None,
        }
    }
}

#[derive(Debug)]
struct SimpleMemoryBlock(Box<[u8]>);
impl SimpleMemoryBlock {
    /// Allocate a new block of memory with the given size
    pub fn allocate(size: usize) -> Self {
        Self(vec![0; size].into_boxed_slice())
    }

    /// Allocated initialised data
    pub fn allocate_initalised(data: Vec<u8>) -> Self {
        Self(data.into_boxed_slice())
    }
}

/// A safe memory address
#[derive(Clone, Debug, PartialEq)]
pub struct SafeAddress {
    block_id: usize,
    offset: usize,
}

impl SafeAddress {
    pub const NULL: SafeAddress = SafeAddress {
        block_id: usize::MAX,
        offset: 0,
    };

    pub fn from_id(id: usize) -> Self {
        Self {
            block_id: id,
            offset: 0,
        }
    }
}

impl UMCOffset for SafeAddress {
    fn offset(&mut self, offset: isize) {
        self.offset = self.offset.saturating_add_signed(offset);
    }
}

impl PartialOrd for SafeAddress {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.block_id == other.block_id {
            return Some(self.offset.cmp(&other.offset));
        }
        None
    }
}

impl MemoryAddress for SafeAddress {}

impl MemoryManager for SafeMemoryManager {
    type Address = SafeAddress;

    fn allocate(&mut self, bytes: usize) -> Result<Self::Address, AllocateError> {
        // Append a new block
        let block = SimpleMemoryBlock::allocate(bytes);
        self.blocks.push(Some(block));
        let id = self.blocks.len() - 1;
        Ok(SafeAddress::from_id(id))
    }

    fn allocate_initalised(&mut self, data: Vec<u8>) -> Result<Self::Address, AllocateError> {
        let block = SimpleMemoryBlock::allocate_initalised(data);
        self.blocks.push(Some(block));
        let id = self.blocks.len() - 1;
        Ok(SafeAddress::from_id(id))
    }

    fn free(&mut self, address: &Self::Address) {
        // Deallocate by removing the allocated block
        self.blocks.get_mut(address.block_id).map(|b| *b = None);
    }

    fn load_prim<V: Serializable>(
        &self,
        address: &Self::Address,
    ) -> Result<V, MemoryAccessError<Self::Address>> {
        let block = self
            .get_block(address)
            .ok_or_else(|| MemoryAccessError::InvalidAddress(address.clone()))?;
        let slice = block
            .0
            .get(address.offset..)
            .ok_or_else(|| MemoryAccessError::OutOfBounds(address.clone()))?;
        V::read_from(slice).map_err(|_| MemoryAccessError::OutOfBounds(address.clone()))
    }

    fn load<V: SerializableArb>(
        &self,
        bitwidth: usize,
        address: &Self::Address,
    ) -> Result<V, MemoryAccessError<Self::Address>> {
        println!("Blocks {:?}", self.blocks);
        let block = self
            .get_block(address)
            .ok_or_else(|| MemoryAccessError::InvalidAddress(address.clone()))?;
        let slice = block
            .0
            .get(address.offset..)
            .ok_or_else(|| MemoryAccessError::OutOfBounds(address.clone()))?;
        V::read_from(slice, bitwidth).map_err(|_| MemoryAccessError::OutOfBounds(address.clone()))
    }

    fn store_prim<V: Serializable>(
        &mut self,
        v: V,
        address: &Self::Address,
    ) -> Result<(), MemoryAccessError<Self::Address>> {
        let block = self
            .get_block_mut(address)
            .ok_or_else(|| MemoryAccessError::InvalidAddress(address.clone()))?;
        let slice = block
            .0
            .get_mut(address.offset..)
            .ok_or_else(|| MemoryAccessError::OutOfBounds(address.clone()))?;
        v.write_to(slice)
            .map_err(|_| MemoryAccessError::OutOfBounds(address.clone()))
    }

    fn store<V: SerializableArb>(
        &mut self,
        v: V,
        address: &Self::Address,
    ) -> Result<(), MemoryAccessError<Self::Address>> {
        let block = self
            .get_block_mut(address)
            .ok_or_else(|| MemoryAccessError::InvalidAddress(address.clone()))?;
        let slice = block
            .0
            .get_mut(address.offset..)
            .ok_or_else(|| MemoryAccessError::OutOfBounds(address.clone()))?;
        v.write_to(slice)
            .map_err(|_| MemoryAccessError::OutOfBounds(address.clone()))
    }
}

#[cfg(test)]
mod test {
    use crate::vm::memory::MemoryManager;
    use crate::vm::memory::safe::SafeMemoryManager;
    use crate::vm::types::UMCOffset;

    #[test]
    fn store_load_u32() {
        let mut mm = SafeMemoryManager::new();
        let address = mm.allocate(size_of::<u32>()).expect("Failed to allocate");
        mm.store_prim(42u32, &address).expect("Failed to store");
        let value: u32 = mm.load_prim(&address).expect("Failed to load");
        assert_eq!(42, value);
    }

    #[test]
    fn store_load_multiple_u32s() {
        let mut mm = SafeMemoryManager::new();
        let a1 = mm
            .allocate(2 * size_of::<u32>())
            .expect("Failed to allocate");
        mm.store_prim::<u32>(258_921, &a1)
            .expect("Failed to store first value");

        let mut a2 = a1.clone();
        a2.offset(size_of::<u32>() as isize);

        println!("a1 {:?} a2 {:?}", a1, a2);
        mm.store_prim::<u32>(42, &a2)
            .expect("Failed to store second value");

        assert_eq!(258_921, mm.load_prim::<u32>(&a1).unwrap());
        assert_eq!(42, mm.load_prim::<u32>(&a2).unwrap());
    }

    #[test]
    fn store_i32_and_u64() {
        let mut mm = SafeMemoryManager::new();
        let a1 = mm
            .allocate(size_of::<i32>() + size_of::<u64>())
            .expect("Failed to allocate");

        mm.store_prim::<i32>(-10, &a1)
            .expect("Failed to store first value");

        let mut a2 = a1.clone();
        a2.offset(size_of::<i32>() as isize);

        mm.store_prim::<u64>(42_000, &a2)
            .expect("Failed to store second value");

        assert_eq!(-10, mm.load_prim::<i32>(&a1).unwrap());
        assert_eq!(42_000, mm.load_prim::<u64>(&a2).unwrap());
    }
}
