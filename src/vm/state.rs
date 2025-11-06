use std::{collections::HashMap, hash::Hash};

use crate::model::{RegIndex, RegWidth};
use crate::vm::types::address::InstructionAddress;
use crate::vm::types::uint::ArbitraryUnsignedInt;

pub trait StoreFor<T: Copy> {
    fn read(&self, i: RegIndex) -> Option<T>;

    fn store(&mut self, i: RegIndex, val: T);

    fn read_multi(&self, i: RegIndex, count: usize) -> Option<&[T]>;

    fn store_multi(&mut self, i: RegIndex, vals: &[T]);
}

pub trait ArbStoreFor<T: Clone> {
    fn read_arb(&self, i: RegIndex, w: RegWidth) -> Option<&T>;

    fn store_arb(&mut self, i: RegIndex, w: RegWidth, val: T);

    fn read_multi_arb(&self, i: RegIndex, w: RegWidth, count: usize) -> Option<&[T]>;

    fn store_multi_arb(&mut self, i: RegIndex, w: RegWidth, vals: &[T]);
}

pub struct RegState {
    u32s: TStore<RegIndex, u32>,
    u64s: TStore<RegIndex, u64>,
    uas: TStore<(RegIndex, RegWidth), ArbitraryUnsignedInt>,
    i32s: TStore<RegIndex, i32>,
    i64s: TStore<RegIndex, i64>,
    addresses: TStore<RegIndex, InstructionAddress>,
}

struct TStore<K: Hash + Eq, V> {
    single: HashMap<K, V>,
    vector: HashMap<(K, usize), Box<[V]>>,
}

impl<K, V> TStore<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            single: HashMap::new(),
            vector: HashMap::new(),
        }
    }
}

impl<V> StoreFor<V> for TStore<RegIndex, V>
where
    V: Copy,
{
    fn read(&self, i: RegIndex) -> Option<V> {
        self.single.get(&i).copied()
    }

    fn store(&mut self, i: RegIndex, val: V) {
        self.single.insert(i, val);
    }

    fn read_multi(&self, i: RegIndex, count: usize) -> Option<&[V]> {
        self.vector.get(&(i, count)).map(|b| &b[..])
    }

    fn store_multi(&mut self, i: RegIndex, vals: &[V]) {
        use std::collections::hash_map::Entry;

        match self.vector.entry((i, vals.len())) {
            Entry::Occupied(o) => {
                o.into_mut().copy_from_slice(vals);
            }
            Entry::Vacant(v) => {
                let b: Box<[V]> = Box::from(vals);
                v.insert(b);
            }
        };
    }
}

impl<V> ArbStoreFor<V> for TStore<(RegIndex, RegWidth), V>
where
    V: Clone,
{
    fn read_arb(&self, i: RegIndex, w: RegWidth) -> Option<&V> {
        self.single.get(&(i, w))
    }

    fn store_arb(&mut self, i: RegIndex, w: RegWidth, val: V) {
        self.single.insert((i, w), val);
    }

    fn read_multi_arb(&self, i: RegIndex, w: RegWidth, count: usize) -> Option<&[V]> {
        self.vector.get(&((i, w), count)).map(|b| &b[..])
    }

    fn store_multi_arb(&mut self, i: RegIndex, w: RegIndex, vals: &[V]) {
        use std::collections::hash_map::Entry;

        match self.vector.entry(((i, w), vals.len())) {
            Entry::Occupied(o) => {
                o.into_mut().clone_from_slice(vals);
            }
            Entry::Vacant(v) => {
                let b: Box<[V]> = Box::from(vals);
                v.insert(b);
            }
        };
    }
}

impl RegState {
    pub fn new() -> Self {
        Self {
            u32s: TStore::new(),
            u64s: TStore::new(),
            uas: TStore::new(),
            i32s: TStore::new(),
            i64s: TStore::new(),
            addresses: TStore::new(),
        }
    }
}

trait DStoreFor<K, V>
where
    K: Hash + Eq,
{
    fn get_store(&self) -> &TStore<K, V>;
    fn get_store_mut(&mut self) -> &mut TStore<K, V>;
}

impl<T: Copy> StoreFor<T> for RegState
where
    RegState: DStoreFor<RegIndex, T>,
{
    fn read(&self, i: RegIndex) -> Option<T> {
        self.get_store().read(i)
    }

    fn store(&mut self, i: RegIndex, val: T) {
        self.get_store_mut().store(i, val)
    }

    fn read_multi(&self, i: RegIndex, count: usize) -> Option<&[T]> {
        self.get_store().read_multi(i, count)
    }

    fn store_multi(&mut self, i: RegIndex, vals: &[T]) {
        self.get_store_mut().store_multi(i, vals)
    }
}

impl<T: Clone> ArbStoreFor<T> for RegState
where
    RegState: DStoreFor<(RegIndex, RegWidth), T>,
{
    fn read_arb(&self, i: RegIndex, w: RegWidth) -> Option<&T> {
        self.get_store().read_arb(i, w)
    }

    fn store_arb(&mut self, i: RegIndex, w: RegWidth, val: T) {
        self.get_store_mut().store_arb(i, w, val)
    }

    fn read_multi_arb(&self, i: RegIndex, w: RegWidth, count: usize) -> Option<&[T]> {
        self.get_store().read_multi_arb(i, w, count)
    }

    fn store_multi_arb(&mut self, i: RegIndex, w: RegWidth, vals: &[T]) {
        self.get_store_mut().store_multi_arb(i, w, vals);
    }
}

// Pick the right store:
impl DStoreFor<RegIndex, u32> for RegState {
    fn get_store(&self) -> &TStore<RegIndex, u32> {
        &self.u32s
    }

    fn get_store_mut(&mut self) -> &mut TStore<RegIndex, u32> {
        &mut self.u32s
    }
}

impl DStoreFor<RegIndex, u64> for RegState {
    fn get_store(&self) -> &TStore<RegIndex, u64> {
        &self.u64s
    }

    fn get_store_mut(&mut self) -> &mut TStore<RegIndex, u64> {
        &mut self.u64s
    }
}

impl DStoreFor<RegIndex, i32> for RegState {
    fn get_store(&self) -> &TStore<RegIndex, i32> {
        &self.i32s
    }

    fn get_store_mut(&mut self) -> &mut TStore<RegIndex, i32> {
        &mut self.i32s
    }
}

impl DStoreFor<RegIndex, i64> for RegState {
    fn get_store(&self) -> &TStore<RegIndex, i64> {
        &self.i64s
    }

    fn get_store_mut(&mut self) -> &mut TStore<RegIndex, i64> {
        &mut self.i64s
    }
}

impl DStoreFor<(RegIndex, RegWidth), ArbitraryUnsignedInt> for RegState {
    fn get_store(&self) -> &TStore<(RegIndex, RegWidth), ArbitraryUnsignedInt> {
        &self.uas
    }

    fn get_store_mut(&mut self) -> &mut TStore<(RegIndex, RegWidth), ArbitraryUnsignedInt> {
        &mut self.uas
    }
}

impl DStoreFor<RegIndex, InstructionAddress> for RegState {
    fn get_store(&self) -> &TStore<RegIndex, InstructionAddress> {
        &self.addresses
    }

    fn get_store_mut(&mut self) -> &mut TStore<RegIndex, InstructionAddress> {
        &mut self.addresses
    }
}
