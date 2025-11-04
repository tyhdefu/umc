use std::collections::HashMap;

use crate::model::RegIndex;

pub trait StoreFor<T> {
    fn read(&self, i: RegIndex) -> Option<&T>;

    fn store(&mut self, i: RegIndex, val: T);

    fn read_multi(&self, i: RegIndex, count: usize) -> Option<&[T]>;

    fn store_multi(&mut self, i: RegIndex, vals: &[T]);
}

pub struct RegState {
    u32s: TStore<u32>,
    u64s: TStore<u64>,
    i32s: TStore<i32>,
    i64s: TStore<i64>,
}

struct TStore<T> {
    single: HashMap<RegIndex, T>,
    vector: HashMap<(RegIndex, usize), Box<[T]>>,
}

impl<T> TStore<T> {
    pub fn new() -> Self {
        Self {
            single: HashMap::new(),
            vector: HashMap::new(),
        }
    }
}

impl<T> Default for TStore<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy> StoreFor<T> for TStore<T> {
    fn read(&self, i: RegIndex) -> Option<&T> {
        self.single.get(&i)
    }

    fn store(&mut self, i: RegIndex, val: T) {
        self.single.insert(i, val);
    }

    fn read_multi(&self, i: RegIndex, count: usize) -> Option<&[T]> {
        self.vector.get(&(i, count)).map(|b| &b[..])
    }

    fn store_multi(&mut self, i: RegIndex, vals: &[T]) {
        use std::collections::hash_map::Entry;

        match self.vector.entry((i, vals.len())) {
            Entry::Occupied(o) => {
                o.into_mut().copy_from_slice(vals);
            }
            Entry::Vacant(v) => {
                let b: Box<[T]> = Box::from(vals);
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
            i32s: TStore::new(),
            i64s: TStore::new(),
        }
    }
}

trait DStoreFor<T> {
    fn get_store(&self) -> &TStore<T>;
    fn get_store_mut(&mut self) -> &mut TStore<T>;
}

impl<T: Copy> StoreFor<T> for RegState
where
    RegState: DStoreFor<T>,
{
    fn read(&self, i: RegIndex) -> Option<&T> {
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

impl DStoreFor<u32> for RegState {
    fn get_store(&self) -> &TStore<u32> {
        &self.u32s
    }

    fn get_store_mut(&mut self) -> &mut TStore<u32> {
        &mut self.u32s
    }
}

impl DStoreFor<u64> for RegState {
    fn get_store(&self) -> &TStore<u64> {
        &self.u64s
    }

    fn get_store_mut(&mut self) -> &mut TStore<u64> {
        &mut self.u64s
    }
}

impl DStoreFor<i32> for RegState {
    fn get_store(&self) -> &TStore<i32> {
        &self.i32s
    }

    fn get_store_mut(&mut self) -> &mut TStore<i32> {
        &mut self.i32s
    }
}

impl DStoreFor<i64> for RegState {
    fn get_store(&self) -> &TStore<i64> {
        &self.i64s
    }

    fn get_store_mut(&mut self) -> &mut TStore<i64> {
        &mut self.i64s
    }
}
