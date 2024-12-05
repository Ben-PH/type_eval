use core::marker::PhantomData;

use crate::{Ast, Eval, Formula, Mode};

mod bit_sub;
mod int_bit_sub;
mod int_int_sub;
pub struct Sub<L, R, M: Mode = Ast> {
    _lhs: PhantomData<L>,
    _rhs: PhantomData<R>,
    _mode: PhantomData<M>,
}

impl<L: Formula, R: Formula> Formula for Sub<L, R>
where
    Sub<L::FOutput, R::FOutput, Eval>: Formula,
{
    type FOutput = <Sub<L::FOutput, R::FOutput, Eval> as Formula>::FOutput;
}

#[cfg(test)]
mod test {
    use crate::{Formula, U0, U1, U2, U3, U4};

    use super::Sub;

    const fn _eval_0<F: Formula<FOutput = U0>>() {}
    const fn _eval_1<F: Formula<FOutput = U1>>() {}
    const fn _eval_2<F: Formula<FOutput = U2>>() {}
    const fn _eval_3<F: Formula<FOutput = U3>>() {}
    #[test]
    fn compile_basic_sub() {
        const _1_SUB_1: () = _eval_0::<Sub<U1, U1>>();
        const _2_SUB_1: () = _eval_1::<Sub<U2, U1>>();
        const _3_SUB_1: () = _eval_2::<Sub<U3, U1>>();
    }
    #[test]
    fn chain_sub() {
        const _1_SUB_0_SUB_1: () = _eval_0::<Sub<Sub<U1, U0>, U1>>();
        const _2_SUB_1_SUB_1: () = _eval_0::<Sub<Sub<U2, U1>, U1>>();
        const _3_SUB_1_SUB_0: () = _eval_2::<Sub<Sub<U3, U1>, U0>>();
    }
    #[test]
    fn uint_add_uint() {
        const _2_SUB_2: () = _eval_0::<Sub<U2, U2>>();
        const _3_SUB_2: () = _eval_1::<Sub<U3, U2>>();
        const _3_SUB_3: () = _eval_0::<Sub<U3, U3>>();
        const _4_SUB_2: () = _eval_2::<Sub<U4, U2>>();
        const _4_SUB_3: () = _eval_1::<Sub<U4, U3>>();
    }
}
