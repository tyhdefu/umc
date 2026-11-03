use umc_model::RegWidth;
use umc_model::instructions::BinaryCondition;
use umc_model::reg_model::{FloatRegT, Reg, RegOrConstant, SignedRegT, UnsignedRegT};

use super::int::{cmp_cond, store_flag};
use super::operand::JitReg;
use super::{Ctx, Flow};

use crate::vm::compiler::FrameSlot;
use crate::vm::compiler::bridge::{jit_frem_f32, jit_frem_f64};
use crate::vm::compiler::emit::*;
use crate::vm::compiler::{BlockExit, CompileError};
