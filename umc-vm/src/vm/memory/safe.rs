//! A safe memory manager implementation that catches invalid attempts to read/write out of bounds, or use-after free

use crate::vm::memory::{AllocateError, MemoryAccessError, MemoryManager, Serializable};
use crate::vm::types::UMCOffset;

pub struct SafeMemoryManager {
    blocks: Vec<Option<SimpleMemoryBlock>>,
}

impl SafeMemoryManager {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    fn get_block(&self, address: &SimpleAddress) -> Option<&SimpleMemoryBlock> {
        match self.blocks.get(address.block_id) {
            Some(v) => v.as_ref(),
            None => None,
        }
    }

    fn get_block_mut(&mut self, address: &SimpleAddress) -> Option<&mut SimpleMemoryBlock> {
        match self.blocks.get_mut(address.block_id) {
            Some(v) => v.as_mut(),
            None => None,
        }
    }
}

struct SimpleMemoryBlock(Vec<u8>);
impl SimpleMemoryBlock {
    /// Allocate a new block of memory with the given size
    pub fn allocate(size: usize) -> Self {
        Self(vec![0; size])
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimpleAddress {
    block_id: usize,
    offset: usize,
}

impl SimpleAddress {
    pub fn from_id(id: usize) -> Self {
        Self {
            block_id: id,
            offset: 0,
        }
    }
}

impl PartialOrd for SimpleAddress {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.block_id == other.block_id {
            return Some(self.offset.cmp(&other.offset));
        }
        None
    }
}

impl MemoryManager for SafeMemoryManager {
    type Address = SimpleAddress;

    fn allocate(&mut self, bytes: usize) -> Result<Self::Address, AllocateError> {
        // Append a new block
        let block = SimpleMemoryBlock::allocate(bytes);
        self.blocks.push(Some(block));
        let id = self.blocks.len() - 1;
        Ok(SimpleAddress::from_id(id))
    }

    fn free(&mut self, address: Self::Address) {
        // Deallocate by removing the allocated block
        self.blocks.get_mut(address.block_id).map(|b| *b = None);
    }

    fn load<V: Serializable>(&self, address: &Self::Address) -> Result<V, MemoryAccessError> {
        let block = self
            .get_block(address)
            .ok_or(MemoryAccessError::InvalidAddress)?;
        let slice = block
            .0
            .get(address.offset..)
            .ok_or(MemoryAccessError::OutOfBounds)?;
        V::read_from(slice).map_err(|_| MemoryAccessError::OutOfBounds)
    }

    fn store<V: Serializable>(
        &mut self,
        v: V,
        address: &Self::Address,
    ) -> Result<(), MemoryAccessError> {
        let block = self
            .get_block_mut(address)
            .ok_or(MemoryAccessError::InvalidAddress)?;
        let slice = block
            .0
            .get_mut(address.offset..)
            .ok_or(MemoryAccessError::OutOfBounds)?;
        v.write_to(slice)
            .map_err(|_| MemoryAccessError::OutOfBounds)
    }
}

impl UMCOffset for SimpleAddress {
    fn offset(&mut self, offset: isize) {
        self.offset = self.offset.saturating_add_signed(offset);
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
        mm.store(42u32, &address).expect("Failed to store");
        let value: u32 = mm.load(&address).expect("Failed to load");
        assert_eq!(42, value);
    }

    #[test]
    fn store_load_multiple_u32s() {
        let mut mm = SafeMemoryManager::new();
        let a1 = mm
            .allocate(2 * size_of::<u32>())
            .expect("Failed to allocate");
        mm.store::<u32>(258_921, &a1)
            .expect("Failed to store first value");

        let mut a2 = a1.clone();
        a2.offset(size_of::<u32>() as isize);

        println!("a1 {:?} a2 {:?}", a1, a2);
        mm.store::<u32>(42, &a2)
            .expect("Failed to store second value");

        assert_eq!(258_921, mm.load::<u32>(&a1).unwrap());
        assert_eq!(42, mm.load::<u32>(&a2).unwrap());
    }

    #[test]
    fn store_i32_and_u64() {
        let mut mm = SafeMemoryManager::new();
        let a1 = mm
            .allocate(size_of::<i32>() + size_of::<u64>())
            .expect("Failed to allocate");

        mm.store::<i32>(-10, &a1)
            .expect("Failed to store first value");

        let mut a2 = a1.clone();
        a2.offset(size_of::<i32>() as isize);

        mm.store::<u64>(42_000, &a2)
            .expect("Failed to store second value");

        assert_eq!(-10, mm.load::<i32>(&a1).unwrap());
        assert_eq!(42_000, mm.load::<u64>(&a2).unwrap());
    }
}
