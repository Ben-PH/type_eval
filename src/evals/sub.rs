use core::marker::PhantomData;

use crate::{Ast, Eval, Formula, Mode, UInt, B0, B1};

pub struct Sub<L, R, M: Mode = Ast> {
    _lhs: PhantomData<L>,
    _rhs: PhantomData<R>,
    _mode: PhantomData<M>,
}

impl<L: Formula, R: Formula> Formula for Sub<L, R>
where
    Sub<L::Output, R::Output, Eval>: Formula,
{
    type Output =<Sub<L::Output, R::Output, Eval> as Formula>::Output;
}
impl Formula for Sub<B0, B0, Eval> {
    type Output = B0;
}
impl Formula for Sub<B1, B0, Eval> {
    type Output = B1;
}
impl Formula for Sub<B1, B1, Eval> {
    type Output = B0;
}

impl Formula for Sub<UInt<B1, B0>, B1, Eval>
{
    type Output = B1;
}
impl<L> Formula for Sub<UInt<L, B1>, B1, Eval>
{
    type Output = UInt<L, B0>;
}
impl<L, R> Formula for Sub<UInt<L, R>, B0, Eval>
{
    type Output = UInt<L, R>;
}
impl<LB, RB> Formula for Sub<UInt<LB, B0>, UInt<RB, B0>, Eval>
where
    Sub<LB, RB>: Formula,
{
    type Output = UInt<<Sub<LB, RB> as Formula>::Output, B0>;
}
impl<LB, RB> Formula for Sub<UInt<LB, B1>, UInt<RB, B0>, Eval>
where
    Sub<LB, RB>: Formula,
{
    type Output = UInt<<Sub<LB, RB> as Formula>::Output, B1>;
}
impl<LB, RB> Formula for Sub<UInt<LB, B1>, UInt<RB, B1>, Eval>
where
    Sub<LB, RB>: Formula,
{
    type Output = UInt<<Sub<LB, RB> as Formula>::Output, B0>;
}
struct Trim<F, M: Mode = Ast> {
    _form: PhantomData<F>,
    _mode: PhantomData<M>,
}
impl<L: Formula> Formula for Trim<UInt<B0, L>, Eval> {
    type Output = L::Output;
}


#[cfg(test)]
mod test {
    use crate::{Formula, U0, U1, U2, U3, U4, U5};

    use super::{Sub, Trim};

    const fn _eval_0<F: Formula<Output = U0>>() {}
    const fn _eval_1<F: Formula<Output = U1>>() {}
    const fn _eval_2<F: Formula<Output = U2>>() {}
    const fn _eval_3<F: Formula<Output = U3>>() {}
    const fn _eval_Sub<F: Formula<Output = Sub<U1, U1>>>(){}
    #[test]
    fn compile_basic_sub() {
        const _SUB: () = _eval_0::<Sub<U1, U1>>();
        const _SUB_3: () = _eval_1::<Sub<U2, U1>>();
        const _SUB_4: () = _eval_2::<Sub<U3, U1>>();
    }
    #[test]
    fn chain_sub() {
        const _1_SUB_0_SUB_1: () = _eval_0::<Sub<Sub<U1, U0>, U1>>();
        const _2_SUB_1_SUB_1: () = _eval_0::<Sub<Sub<U2, U1>, U1>>();
        const _3_SUB_1_SUB_0: () = _eval_2::<Sub<Sub<U3, U1>, U0>>();
    }
    // #[test]
    // fn uint_add_uint() {
        const _3_SUB_3: () = _eval_0::<Trim<Sub<U2, U2>>>();
    //     const _3_SUB_2: () = _eval_1::<Sub<U3, U2>>();
    //     const _3_SUB_3: () = _eval_0::<Sub<U3, U3>>();
    //     const _4_SUB_2: () = _eval_2::<Sub<U4, U2>>();
    //     const _4_SUB_3: () = _eval_1::<Sub<U4, U3>>();
    // }
}
