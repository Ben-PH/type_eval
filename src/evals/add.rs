//! basics operation outputs such as B1 + B0 and visa versa, boolean algebras, etc

use core::marker::PhantomData;

use crate::{Ast, Eval, Formula, Mode, UInt, B0, B1};

pub struct Add<L, R, M: Mode = Ast> {
    _lhs: PhantomData<L>,
    _rhs: PhantomData<R>,
    _mode: PhantomData<M>,
}

impl<L: Formula, R: Formula> Formula for Add<L, R>
where
    Add<L::Output, R::Output, Eval>: Formula,
{
    type Output =<Add<L::Output, R::Output, Eval> as Formula>::Output;
}
impl Formula for Add<B0, B0, Eval> {
    type Output = B0;
}
impl Formula for Add<B0, B1, Eval> {
    type Output = B1;
}
impl Formula for Add<B1, B0, Eval> {
    type Output = B1;
}
impl Formula for Add<B1, B1, Eval> {
    type Output = UInt<B1, B0>;
}

impl Formula for Add<UInt<B1, B0>, B1, Eval>
{
    type Output = UInt<B1, B1>;
}
impl Formula for Add<UInt<B1, B1>, B1, Eval>
{
    type Output = UInt<UInt<B1, B0>, B0>;
}
impl<L, R> Formula for Add<UInt<UInt<L, R>, B0>, B1, Eval>
{
    type Output = UInt<UInt<L, R>, B1>;
}
impl<L, R> Formula for Add<UInt<UInt<L, R>, B1>, B1, Eval>
{
    type Output = UInt<Add<UInt<L, R>, B1>, B0>;
}
impl<L, R> Formula for Add<B1, UInt<L, R>, Eval>
where
    Add<UInt<L, R>, B1>: Formula,
{
    type Output = <Add<UInt<L, R>, B1> as Formula>::Output;
}

impl<L, R> Formula for Add<B0, UInt<L, R>, Eval>
where
    Add<UInt<L, R>, B0>: Formula,
{
    type Output = <Add<UInt<L, R>, B0> as Formula>::Output;
}

// adding nums to nums
impl<LB, RB> Formula for Add<UInt<LB, B0>, UInt<RB, B0>, Eval>
where
    Add<LB, RB>: Formula,
{
    type Output = UInt<<Add<LB, RB> as Formula>::Output, B0>;
}
impl<LB, RB> Formula for Add<UInt<LB, B0>, UInt<RB, B1>, Eval>
where
    Add<LB, RB, Eval>: Formula,
{
    type Output = UInt<<Add<LB, RB, Eval> as Formula>::Output, B1>;
}
impl<LB, RB> Formula for Add<UInt<LB, B1>, UInt<RB, B0>, Eval>
where
    Add<LB, RB, Eval>: Formula,
{
    type Output = UInt<<Add<LB, RB, Eval> as Formula>::Output, B1>;
}
impl<LB, RB> Formula for Add<UInt<LB, B1>, UInt<RB, B1>, Eval>
where
    Add<LB, RB, Eval>: Formula,
{
    type Output = UInt<<Add<LB, RB, Eval> as Formula>::Output, B0>;
}


#[cfg(test)]
mod test {
    use crate::{Formula, U0, U1, U2, U3, U4, U5};

    use super::Add;

    const fn _eval_2<F: Formula<Output = U2>>() {}
    const fn _eval_3<F: Formula<Output = U3>>() {}
    const fn _eval_4<F: Formula<Output = U4>>() {}
    const fn _eval_5<F: Formula<Output = U5>>() {}
    const fn _eval_add<F: Formula<Output = Add<U1, U1>>>(){}
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
