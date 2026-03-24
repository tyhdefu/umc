use std::collections::HashMap;
use std::hash::Hash;

use crate::vm::memory::MemoryAddress;
use crate::vm::memory::safe::SafeAddress;
use crate::vm::types::address::InstructionAddress;
use crate::vm::types::int::ArbitraryInt;
use crate::vm::types::uint::ArbitraryUnsignedInt;
use crate::vm::types::vector::VecValue;
use rustc_hash::FxBuildHasher;
use umc_model::reg_model::{
    FloatRegT, InstrRegT, MemRegT, Reg, RegTypeT, SignedRegT, UnsignedRegT,
};
use umc_model::{RegIndex, RegWidth};

pub trait StoreFor<V, RT: RegTypeT> {
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

pub struct RegState<M: MemoryAddress> {
    u1s: FastPrimStore<bool>,
    u32s: FastPrimStore<u32>,
    u64s: HashMapStore<RegIndex, u64>,
    uas: HashMapStore<Reg<UnsignedRegT>, ArbitraryUnsignedInt>,
    i32s: HashMapStore<RegIndex, i32>,
    i64s: HashMapStore<RegIndex, i64>,
    ias: HashMapStore<Reg<SignedRegT>, ArbitraryInt>,
    f32s: HashMapStore<RegIndex, f32>,
    f64s: HashMapStore<RegIndex, f64>,
    addresses: HashMapStore<Reg<InstrRegT>, InstructionAddress>,
    mem_addresses: FastOptStore<M>,
}

impl<M: MemoryAddress> RegState<M> {
    pub fn new() -> Self {
        Self {
            u1s: FastPrimStore::new_default(),
            u32s: FastPrimStore::new_default(),
            u64s: HashMapStore::new(),
            uas: HashMapStore::new(),
            i32s: HashMapStore::new(),
            i64s: HashMapStore::new(),
            ias: HashMapStore::new(),
            f32s: HashMapStore::new(),
            f64s: HashMapStore::new(),
            addresses: HashMapStore::new(),
            mem_addresses: FastOptStore::new_none(),
        }
    }
}

trait DStoreFor<RT: RegTypeT, V>
where
    Reg<RT>: Hash + Eq,
{
    fn get_store(&self) -> &HashMapStore<Reg<RT>, V>;
    fn get_store_mut(&mut self) -> &mut HashMapStore<Reg<RT>, V>;
}

trait PrimNumStoreFor<RT: RegTypeT, P: Copy> {
    const BITS: u32;
    fn get_store(&self) -> &HashMapStore<RegIndex, P>;
    fn get_store_mut(&mut self) -> &mut HashMapStore<RegIndex, P>;
}

impl<RT: RegTypeT, V, M: MemoryAddress> StoreFor<V, RT> for RegState<M>
where
    Self: DStoreFor<RT, V>,
    Reg<RT>: Hash + Eq + Copy + 'static,
{
    fn read(&self, k: Reg<RT>) -> Option<&V> {
        DStoreFor::get_store(self).read(k)
    }

    fn store(&mut self, k: Reg<RT>, val: V) {
        DStoreFor::get_store_mut(self).store(k, val);
    }

    fn read_multi(&self, k: Reg<RT>, count: usize) -> Option<&VecValue<V>> {
        DStoreFor::get_store(self).read_multi(k, count)
    }

    fn store_multi_clone(&mut self, k: Reg<RT>, vals: &[V])
    where
        V: Clone,
    {
        DStoreFor::get_store_mut(self).store_multi_clone(k, vals);
    }

    fn store_multi_copy(&mut self, k: Reg<RT>, vals: &[V])
    where
        V: Copy,
    {
        DStoreFor::get_store_mut(self).store_multi_copy(k, vals);
    }
}

impl<M: MemoryAddress, P: Copy, RT: RegTypeT<WIDTH = RegWidth>> StorePrim<P, RT> for RegState<M>
where
    Self: PrimNumStoreFor<RT, P>,
    P: Copy,
{
    fn read_prim(&self, k: Reg<RT>) -> Option<P> {
        debug_assert!(k.width <= Self::BITS);
        PrimNumStoreFor::get_store(self).read(k.index).copied()
    }

    fn store_prim(&mut self, k: Reg<RT>, val: P) {
        debug_assert!(k.width <= Self::BITS);
        PrimNumStoreFor::get_store_mut(self).store(k.index, val);
    }

    fn read_multi_prim(&self, k: Reg<RT>, count: usize) -> Option<&VecValue<P>> {
        debug_assert!(k.width <= Self::BITS);
        PrimNumStoreFor::get_store(self).read_multi(k.index, count)
    }

    fn store_multi_copy_prim(&mut self, k: Reg<RT>, vals: &[P]) {
        debug_assert!(k.width <= Self::BITS);
        PrimNumStoreFor::get_store_mut(self).store_multi_copy(k.index, vals);
    }
}

/// Store that primarily uses a vector, but fallbacks to hashmap for high indicies
struct FastPrimStore<WV, V = WV> {
    single_fast: Vec<WV>,
    fallback: HashMapStore<RegIndex, V>,
}

const FAST_STORE_COUNT: usize = 128;

impl<V> FastPrimStore<V>
where
    V: Default + Copy,
{
    pub fn new_default() -> Self {
        Self {
            single_fast: vec![V::default(); FAST_STORE_COUNT],
            fallback: HashMapStore::new(),
        }
    }

    fn read_prim(&self, index: RegIndex) -> Option<V> {
        if (index as usize) < FAST_STORE_COUNT {
            return Some(self.single_fast[index as usize].clone());
        }
        self.read_fallback(index)
    }

    #[inline(never)]
    #[cold]
    fn read_fallback(&self, index: RegIndex) -> Option<V> {
        return self.fallback.read(index).cloned();
    }

    fn store_prim(&mut self, index: RegIndex, val: V) {
        if (index as usize) < FAST_STORE_COUNT {
            self.single_fast[index as usize] = val;
            return;
        }
        self.store_fallback(index, val);
    }

    #[inline(never)]
    #[cold]
    fn store_fallback(&mut self, index: RegIndex, val: V) {
        self.fallback.store(index, val);
    }
}

pub struct FastOptStore<V> {
    single_fast: Vec<Option<V>>,
    fallback: HashMapStore<RegIndex, V>,
}

impl<V> FastOptStore<V>
where
    V: Clone,
{
    fn new_none() -> Self {
        Self {
            single_fast: vec![None; FAST_STORE_COUNT],
            fallback: HashMapStore::new(),
        }
    }

    fn read_fast(&self, index: RegIndex) -> Option<&V> {
        if (index as usize) < FAST_STORE_COUNT {
            return self.single_fast[index as usize].as_ref();
        }
        return self.read_fallback(index);
    }

    // Make read fast inlined
    #[inline(never)]
    #[cold]
    fn read_fallback(&self, index: RegIndex) -> Option<&V> {
        return self.fallback.read(index);
    }

    fn store_fast(&mut self, index: RegIndex, val: V) {
        if (index as usize) < FAST_STORE_COUNT {
            self.single_fast[index as usize] = Some(val);
            return;
        }
        self.store_fallback(index, val);
    }

    // Make store fast inlined
    #[inline(never)]
    #[cold]
    fn store_fallback(&mut self, index: RegIndex, val: V) {
        self.fallback.store(index, val);
    }
}

/// Store based on HashMaps
struct HashMapStore<K: Hash + Eq, V> {
    single: HashMap<K, V, FxBuildHasher>,
    vector: HashMap<(K, usize), VecValue<V>, FxBuildHasher>,
}

impl<K, V> HashMapStore<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            single: HashMap::with_hasher(FxBuildHasher::default()),
            vector: HashMap::with_hasher(FxBuildHasher::default()),
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

impl StoreFor<SafeAddress, MemRegT> for RegState<SafeAddress> {
    fn read(&self, k: Reg<MemRegT>) -> Option<&SafeAddress> {
        self.mem_addresses.read_fast(k.index)
    }

    fn store(&mut self, k: Reg<MemRegT>, val: SafeAddress) {
        self.mem_addresses.store_fast(k.index, val)
    }

    fn read_multi(&self, k: Reg<MemRegT>, count: usize) -> Option<&VecValue<SafeAddress>> {
        self.mem_addresses.fallback.read_multi(k.index, count)
    }

    fn store_multi_copy(&mut self, k: Reg<MemRegT>, vals: &[SafeAddress]) {
        panic!("Uncallable method");
        //self.mem_addresses.fallback.store_multi_copy(k.index, vals);
    }

    fn store_multi_clone(&mut self, k: Reg<MemRegT>, vals: &[SafeAddress]) {
        self.mem_addresses.fallback.store_multi_clone(k.index, vals);
    }
}

impl<M: MemoryAddress> StorePrim<bool, UnsignedRegT> for RegState<M> {
    fn read_prim(&self, k: Reg<UnsignedRegT>) -> Option<bool> {
        debug_assert_eq!(k.width, 1);
        self.u1s.read_prim(k.index)
    }

    fn store_prim(&mut self, k: Reg<UnsignedRegT>, val: bool) {
        debug_assert_eq!(k.width, 1);
        self.u1s.store_prim(k.index, val);
    }

    fn read_multi_prim(&self, k: Reg<UnsignedRegT>, count: usize) -> Option<&VecValue<bool>> {
        debug_assert_eq!(k.width, 1);
        self.u1s.fallback.read_multi(k.index, count)
    }

    fn store_multi_copy_prim(&mut self, k: Reg<UnsignedRegT>, vals: &[bool]) {
        debug_assert_eq!(k.width, 1);
        self.u1s.fallback.store_multi_copy(k.index, vals);
    }
}

impl<M: MemoryAddress> StorePrim<u32, UnsignedRegT> for RegState<M> {
    fn read_prim(&self, k: Reg<UnsignedRegT>) -> Option<u32> {
        debug_assert_eq!(k.width, u32::BITS);
        self.u32s.read_prim(k.index)
    }

    fn store_prim(&mut self, k: Reg<UnsignedRegT>, val: u32) {
        debug_assert_eq!(k.width, u32::BITS);
        self.u32s.store_prim(k.index, val);
    }

    fn read_multi_prim(&self, k: Reg<UnsignedRegT>, count: usize) -> Option<&VecValue<u32>> {
        debug_assert_eq!(k.width, u32::BITS);
        self.u32s.fallback.read_multi(k.index, count)
    }

    fn store_multi_copy_prim(&mut self, k: Reg<UnsignedRegT>, vals: &[u32]) {
        debug_assert_eq!(k.width, u32::BITS);
        self.u32s.fallback.store_multi_copy(k.index, vals);
    }
}

impl<M: MemoryAddress> PrimNumStoreFor<UnsignedRegT, u64> for RegState<M> {
    const BITS: u32 = u64::BITS;

    fn get_store(&self) -> &HashMapStore<RegIndex, u64> {
        &self.u64s
    }

    fn get_store_mut(&mut self) -> &mut HashMapStore<RegIndex, u64> {
        &mut self.u64s
    }
}

impl<M: MemoryAddress> PrimNumStoreFor<SignedRegT, i32> for RegState<M> {
    const BITS: u32 = i32::BITS;

    fn get_store(&self) -> &HashMapStore<RegIndex, i32> {
        &self.i32s
    }

    fn get_store_mut(&mut self) -> &mut HashMapStore<RegIndex, i32> {
        &mut self.i32s
    }
}

impl<M: MemoryAddress> PrimNumStoreFor<SignedRegT, i64> for RegState<M> {
    const BITS: u32 = i64::BITS;

    fn get_store(&self) -> &HashMapStore<RegIndex, i64> {
        &self.i64s
    }

    fn get_store_mut(&mut self) -> &mut HashMapStore<RegIndex, i64> {
        &mut self.i64s
    }
}

impl<M: MemoryAddress> PrimNumStoreFor<FloatRegT, f32> for RegState<M> {
    const BITS: u32 = 32;

    fn get_store(&self) -> &HashMapStore<RegIndex, f32> {
        &self.f32s
    }

    fn get_store_mut(&mut self) -> &mut HashMapStore<RegIndex, f32> {
        &mut self.f32s
    }
}

impl<M: MemoryAddress> PrimNumStoreFor<FloatRegT, f64> for RegState<M> {
    const BITS: u32 = 64;

    fn get_store(&self) -> &HashMapStore<RegIndex, f64> {
        &self.f64s
    }

    fn get_store_mut(&mut self) -> &mut HashMapStore<RegIndex, f64> {
        &mut self.f64s
    }
}

impl<M: MemoryAddress> DStoreFor<UnsignedRegT, ArbitraryUnsignedInt> for RegState<M> {
    fn get_store(&self) -> &HashMapStore<Reg<UnsignedRegT>, ArbitraryUnsignedInt> {
        &self.uas
    }

    fn get_store_mut(&mut self) -> &mut HashMapStore<Reg<UnsignedRegT>, ArbitraryUnsignedInt> {
        &mut self.uas
    }
}

impl<M: MemoryAddress> DStoreFor<SignedRegT, ArbitraryInt> for RegState<M> {
    fn get_store(&self) -> &HashMapStore<Reg<SignedRegT>, ArbitraryInt> {
        &self.ias
    }

    fn get_store_mut(&mut self) -> &mut HashMapStore<Reg<SignedRegT>, ArbitraryInt> {
        &mut self.ias
    }
}

impl<M: MemoryAddress> DStoreFor<InstrRegT, InstructionAddress> for RegState<M> {
    fn get_store(&self) -> &HashMapStore<Reg<InstrRegT>, InstructionAddress> {
        &self.addresses
    }

    fn get_store_mut(&mut self) -> &mut HashMapStore<Reg<InstrRegT>, InstructionAddress> {
        &mut self.addresses
    }
}
