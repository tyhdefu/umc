use std::marker::PhantomData;

use umc_model::reg_model::{Reg, RegOrConstant, RegTypeT};

use crate::vm::{memory::MemoryAccessError, state::StoreFor, widths::WidthOptions};

/*
trait SingleWidthRT: RegTypeT {
    type Value;

    fn is_value_zero(v: &Self::Value) -> bool;
}

pub struct SingleWidth<RT: SingleWidthRT>(PhantomData<RT>);

impl<RT: SingleWidthRT, STATE> WidthOptions<STATE> for SingleWidth<RT>
where
    STATE: StoreFor<RT::Value, RT>,
{
    type RT = RT;

    fn compare(
        p1: &RegOrConstant<Self::RT>,
        p2: &RegOrConstant<Self::RT>,
        state: &STATE,
    ) -> Option<std::cmp::Ordering> {
        todo!()
    }

    fn is_zero(reg: &RegOrConstant<Self::RT>, state: &STATE) -> bool {
        match reg {
            RegOrConstant::Reg(reg) => todo!(),
            RegOrConstant::Const(c) => ,
        }
        state
            .read(reg)
            .map(|v| RT::is_value_zero(v))
            .unwrap_or(true) // Assume true by default
    }

    fn store_into_memory<M: crate::vm::memory::MemoryManager>(
        reg: Reg<Self::RT>,
        state: &STATE,
        memory: &mut M,
        address: &M::Address,
    ) -> Result<(), MemoryAccessError<M::Address>> {
        todo!()
    }
}
*/
