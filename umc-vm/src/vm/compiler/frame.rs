use rustc_hash::FxHashMap;
use umc_model::{RegIndex, RegWidth};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrameSlot(pub u16);

impl FrameSlot {
    pub const BYTES: i32 = 8;

    pub(crate) fn new(index: u16) -> Self {
        Self(index)
    }

    pub(crate) fn byte_offset(&self) -> i32 {
        self.0 as i32 * Self::BYTES
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SlotKey {
    Unsigned { index: RegIndex, width: RegWidth },
    Signed { index: RegIndex, width: RegWidth },
    Float { index: RegIndex, width: RegWidth }, // raw IEEE bits, f32 in low 32
    Instr { index: RegIndex },
    MemBlock { index: RegIndex },
    MemOffset { index: RegIndex },
    ConstBlock { label: u32 },
    ConstOffset { label: u32 },
}

// memory address operand: register or pre-inisitalised label
#[derive(Debug, Clone, Copy)]
pub enum MemKey {
    Reg(RegIndex),
    Const(u32),
}

#[derive(Debug, Clone)]
pub struct SlotInfo {
    pub key: SlotKey,
    pub live_in: bool, // read before written anywhere in block
    pub dirty: bool,   // written before in block
}

pub struct FrameLayout {
    pub slots: Vec<SlotInfo>,
    pub lookup: FxHashMap<SlotKey, FrameSlot>,
}

impl FrameLayout {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            lookup: Default::default(),
        }
    }

    fn slot_for(&mut self, key: SlotKey, live_in: bool) -> (FrameSlot, &mut SlotInfo) {
        let slots = &mut self.slots;
        let slot = *self.lookup.entry(key).or_insert_with(|| {
            slots.push(SlotInfo {
                key,
                live_in,
                dirty: false,
            });
            FrameSlot::new((slots.len() - 1) as u16)
        });
        (slot, &mut self.slots[slot.0 as usize])
    }

    pub fn read_slot(&mut self, key: SlotKey) -> FrameSlot {
        let (slot, info) = self.slot_for(key, true);
        if !info.dirty {
            info.live_in = true;
        }
        slot
    }

    pub fn write_slot(&mut self, key: SlotKey) -> FrameSlot {
        // marks slot as dirty
        let (slot, info) = self.slot_for(key, false);
        info.dirty = true;
        slot
    }

    // address = block slot then offset slot, both keys only ever allocated here so stay adjacent
    // returns block slot
    fn mem_pair(&mut self, key: MemKey, write: bool) -> FrameSlot {
        let (b, o) = match key {
            MemKey::Reg(index) => (SlotKey::MemBlock { index }, SlotKey::MemOffset { index }),
            MemKey::Const(label) => (
                SlotKey::ConstBlock { label },
                SlotKey::ConstOffset { label },
            ),
        };
        let (bs, os) = if write {
            (self.write_slot(b), self.write_slot(o))
        } else {
            (self.read_slot(b), self.read_slot(o))
        };
        debug_assert_eq!(os.0, bs.0 + 1);
        bs
    }

    pub fn mem_read_slot(&mut self, key: MemKey) -> FrameSlot {
        self.mem_pair(key, false)
    }
    pub fn mem_write_slot(&mut self, index: RegIndex) -> FrameSlot {
        self.mem_pair(MemKey::Reg(index), true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layout() -> FrameLayout {
        FrameLayout {
            slots: Vec::new(),
            lookup: FxHashMap::default(),
        }
    }

    fn u(index: RegIndex, width: RegWidth) -> SlotKey {
        SlotKey::Unsigned { index, width }
    }

    #[test]
    fn same_key_reuses_slot_and_widths_are_distinct() {
        let mut l = layout();
        let a = l.read_slot(u(0, 64));
        let b = l.write_slot(u(0, 64));
        let c = l.read_slot(u(0, 32));
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_eq!((a.byte_offset(), c.byte_offset()), (0, 8));
        assert_eq!(l.slots.len(), 2);
    }

    #[test]
    fn read_before_write_is_live_in() {
        let mut l = layout();
        let s = l.read_slot(u(1, 64));
        l.write_slot(u(1, 64));
        let info = &l.slots[s.0 as usize];
        assert!(info.live_in && info.dirty);
    }

    #[test]
    fn write_before_read_is_not_live_in() {
        let mut l = layout();
        let s = l.write_slot(u(1, 64));
        l.read_slot(u(1, 64));
        let info = &l.slots[s.0 as usize];
        assert!(!info.live_in && info.dirty);
    }
}
