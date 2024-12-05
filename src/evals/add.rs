//! basics operation outputs such as B1 + B0 and visa versa, boolean algebras, etc

use core::marker::PhantomData;

use crate::{Ast, Eval, Formula, Mode};

mod bit_bit_adds;
mod int_bit_adds;
mod int_int_adds;

pub struct Add<L, R, M: Mode = Ast> {
    _lhs: PhantomData<L>,
    _rhs: PhantomData<R>,
    _mode: PhantomData<M>,
}

impl<L: Formula, R: Formula> Formula for Add<L, R>
where
    Add<L::FOutput, R::FOutput, Eval>: Formula,
{
    type FOutput = <Add<L::FOutput, R::FOutput, Eval> as Formula>::FOutput;
}

#[cfg(test)]
mod test {
    use crate::{Formula, U0, U1, U2, U3, U4, U5};

    use super::Add;

    const fn _eval_2<F: Formula<FOutput = U2>>() {}
    const fn _eval_3<F: Formula<FOutput = U3>>() {}
    const fn _eval_4<F: Formula<FOutput = U4>>() {}
    const fn _eval_5<F: Formula<FOutput = U5>>() {}
    const fn _eval_add<F: Formula<FOutput = Add<U1, U1>>>() {}
    #[test]
    fn compile_basic_add() {
        const _ADD: () = _eval_2::<Add<U1, U1>>();
        const _ADD_3: () = _eval_3::<Add<U2, U1>>();
        const _ADD_4: () = _eval_4::<Add<U3, U1>>();
    }
    #[test]
    fn chain_add() {
        const _ADD_2: () = _eval_2::<Add<Add<U1, U0>, U1>>();
        const _ADD_3: () = _eval_3::<Add<Add<U1, U1>, U1>>();
        const _ADD_4: () = _eval_3::<Add<Add<U1, U1>, U1>>();
    }
    #[test]
    fn uint_add_uint() {
        const _ADD_2: () = _eval_2::<Add<Add<U1, U0>, U1>>();
        const _ADD_3: () = _eval_3::<Add<Add<U1, U1>, U1>>();
        const _ADD_2_2: () = _eval_4::<Add<U2, U2>>();
        const _ADD_1_2: () = _eval_3::<Add<U1, U2>>();
        const _ADD_2_1: () = _eval_3::<Add<U2, U1>>();
        const _ADD_3_2: () = _eval_5::<Add<U3, U2>>();
        const _ADD_2_3: () = _eval_5::<Add<U2, U3>>();
    }
}
