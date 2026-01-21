use std::collections::HashMap;
use std::hash::Hash;

use crate::vm::types::address::InstructionAddress;
use crate::vm::types::uint::ArbitraryUnsignedInt;
use crate::vm::types::vector::VecValue;
use umc_model::RegIndex;
use umc_model::reg_model::{FloatRegT, InstrRegT, NumReg, Reg, RegTypeT, SignedRegT, UnsignedRegT};

pub trait StoreFor<V, RT: RegTypeT>
where
    RT::R: Copy,
{
    fn read(&self, k: Reg<RT>) -> Option<&V>;

    fn store(&mut self, k: Reg<RT>, val: V);

    fn read_multi(&self, k: Reg<RT>, count: usize) -> Option<&VecValue<V>>;

    fn store_multi_copy(&mut self, k: Reg<RT>, vals: &[V])
    where
        V: Copy;

    fn store_multi_clone(&mut self, k: Reg<RT>, vals: &[V])
    where
        V: Clone;
}

pub trait StorePrim<V, RT: RegTypeT>
where
    V: Copy,
{
    fn read_prim(&self, k: Reg<RT>) -> Option<V>;

    fn store_prim(&mut self, k: Reg<RT>, val: V);

    fn read_multi_prim(&self, k: Reg<RT>, count: usize) -> Option<&VecValue<V>>;

    fn store_multi_copy_prim(&mut self, k: Reg<RT>, vals: &[V]);
}

pub struct RegState {
    u32s: HashMapStore<RegIndex, u32>,
    u64s: HashMapStore<RegIndex, u64>,
    uas: HashMapStore<NumReg, ArbitraryUnsignedInt>,
    i32s: HashMapStore<RegIndex, i32>,
    i64s: HashMapStore<RegIndex, i64>,
    f32s: HashMapStore<RegIndex, f32>,
    f64s: HashMapStore<RegIndex, f64>,
    addresses: HashMapStore<RegIndex, InstructionAddress>,
}

impl RegState {
    pub fn new() -> Self {
        Self {
            u32s: HashMapStore::new(),
            u64s: HashMapStore::new(),
            uas: HashMapStore::new(),
            i32s: HashMapStore::new(),
            i64s: HashMapStore::new(),
            f32s: HashMapStore::new(),
            f64s: HashMapStore::new(),
            addresses: HashMapStore::new(),
        }
    }
}

trait DStoreFor<RT: RegTypeT, V>
where
    RT::R: Hash + Eq,
{
    fn get_store(&self) -> &HashMapStore<RT::R, V>;
    fn get_store_mut(&mut self) -> &mut HashMapStore<RT::R, V>;
}

trait PrimNumStoreFor<RT: RegTypeT, P: Copy> {
    const BITS: u32;
    fn get_store(&self) -> &HashMapStore<RegIndex, P>;
    fn get_store_mut(&mut self) -> &mut HashMapStore<RegIndex, P>;
}

impl<RT: RegTypeT, V> StoreFor<V, RT> for RegState
where
    Self: DStoreFor<RT, V>,
    RT::R: Hash + Eq + Copy + 'static,
{
    fn read(&self, k: Reg<RT>) -> Option<&V> {
        DStoreFor::get_store(self).read(k.0)
    }

    fn store(&mut self, k: Reg<RT>, val: V) {
        DStoreFor::get_store_mut(self).store(k.0, val);
    }

    fn read_multi(&self, k: Reg<RT>, count: usize) -> Option<&VecValue<V>> {
        DStoreFor::get_store(self).read_multi(k.0, count)
    }

    fn store_multi_clone(&mut self, k: Reg<RT>, vals: &[V])
    where
        V: Clone,
    {
        DStoreFor::get_store_mut(self).store_multi_clone(k.0, vals);
    }

    fn store_multi_copy(&mut self, k: Reg<RT>, vals: &[V])
    where
        V: Copy,
    {
        DStoreFor::get_store_mut(self).store_multi_copy(k.0, vals);
    }
}

impl<P: Copy, RT: RegTypeT<R = NumReg>> StorePrim<P, RT> for RegState
where
    Self: PrimNumStoreFor<RT, P>,
    RT::R: Copy,
    P: Copy,
{
    fn read_prim(&self, k: Reg<RT>) -> Option<P> {
        debug_assert!(k.0.width <= Self::BITS);
        PrimNumStoreFor::get_store(self).read(k.0.index).copied()
    }

    fn store_prim(&mut self, k: Reg<RT>, val: P) {
        debug_assert!(k.0.width <= Self::BITS);
        PrimNumStoreFor::get_store_mut(self).store(k.0.index, val);
    }

    fn read_multi_prim(&self, k: Reg<RT>, count: usize) -> Option<&VecValue<P>> {
        debug_assert!(k.0.width <= Self::BITS);
        PrimNumStoreFor::get_store(self).read_multi(k.0.index, count)
    }

    fn store_multi_copy_prim(&mut self, k: Reg<RT>, vals: &[P]) {
        debug_assert!(k.0.width <= Self::BITS);
        PrimNumStoreFor::get_store_mut(self).store_multi_copy(k.0.index, vals);
    }
}

/// Store based on HashMaps
struct HashMapStore<K: Hash + Eq, V> {
    single: HashMap<K, V>,
    vector: HashMap<(K, usize), VecValue<V>>,
}

impl<K, V> HashMapStore<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            single: HashMap::new(),
            vector: HashMap::new(),
        }
    }

    fn read(&self, k: K) -> Option<&V> {
        self.single.get(&k)
    }

    fn store(&mut self, k: K, val: V) {
        self.single.insert(k, val);
    }

    fn read_multi(&self, k: K, count: usize) -> Option<&VecValue<V>> {
        self.vector.get(&(k, count))
    }

    fn store_multi_copy(&mut self, k: K, vals: &[V])
    where
        V: Copy,
    {
        use std::collections::hash_map::Entry;

        match self.vector.entry((k, vals.len())) {
            Entry::Occupied(o) => {
                o.into_mut().copy_from_slice(vals);
            }
            Entry::Vacant(v) => {
                v.insert(VecValue::from_vec(vals.to_vec()));
            }
        };
    }

    fn store_multi_clone(&mut self, k: K, vals: &[V])
    where
        V: Clone,
    {
        use std::collections::hash_map::Entry;

        match self.vector.entry((k, vals.len())) {
            Entry::Occupied(o) => {
                o.into_mut().clone_from_slice(vals);
            }
            Entry::Vacant(v) => {
                v.insert(VecValue::from_vec(vals.to_vec()));
            }
        };
    }
}

impl PrimNumStoreFor<UnsignedRegT, u32> for RegState {
    const BITS: u32 = u32::BITS;

    fn get_store(&self) -> &HashMapStore<RegIndex, u32> {
        &self.u32s
    }

    fn get_store_mut(&mut self) -> &mut HashMapStore<RegIndex, u32> {
        &mut self.u32s
    }
}

impl PrimNumStoreFor<UnsignedRegT, u64> for RegState {
    const BITS: u32 = u64::BITS;

    fn get_store(&self) -> &HashMapStore<RegIndex, u64> {
        &self.u64s
    }

    fn get_store_mut(&mut self) -> &mut HashMapStore<RegIndex, u64> {
        &mut self.u64s
    }
}

impl PrimNumStoreFor<SignedRegT, i32> for RegState {
    const BITS: u32 = i32::BITS;

    fn get_store(&self) -> &HashMapStore<RegIndex, i32> {
        &self.i32s
    }

    fn get_store_mut(&mut self) -> &mut HashMapStore<RegIndex, i32> {
        &mut self.i32s
    }
}

impl PrimNumStoreFor<SignedRegT, i64> for RegState {
    const BITS: u32 = i64::BITS;

    fn get_store(&self) -> &HashMapStore<RegIndex, i64> {
        &self.i64s
    }

    fn get_store_mut(&mut self) -> &mut HashMapStore<RegIndex, i64> {
        &mut self.i64s
    }
}

impl PrimNumStoreFor<FloatRegT, f32> for RegState {
    const BITS: u32 = 32;

    fn get_store(&self) -> &HashMapStore<RegIndex, f32> {
        &self.f32s
    }

    fn get_store_mut(&mut self) -> &mut HashMapStore<RegIndex, f32> {
        &mut self.f32s
    }
}

impl PrimNumStoreFor<FloatRegT, f64> for RegState {
    const BITS: u32 = 64;

    fn get_store(&self) -> &HashMapStore<RegIndex, f64> {
        &self.f64s
    }

    fn get_store_mut(&mut self) -> &mut HashMapStore<RegIndex, f64> {
        &mut self.f64s
    }
}

impl DStoreFor<UnsignedRegT, ArbitraryUnsignedInt> for RegState {
    fn get_store(&self) -> &HashMapStore<NumReg, ArbitraryUnsignedInt> {
        &self.uas
    }

    fn get_store_mut(&mut self) -> &mut HashMapStore<NumReg, ArbitraryUnsignedInt> {
        &mut self.uas
    }
}

impl DStoreFor<InstrRegT, InstructionAddress> for RegState {
    fn get_store(&self) -> &HashMapStore<RegIndex, InstructionAddress> {
        &self.addresses
    }

    fn get_store_mut(&mut self) -> &mut HashMapStore<RegIndex, InstructionAddress> {
        &mut self.addresses
    }
}
