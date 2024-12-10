#![no_std]

use core::marker::PhantomData;

trait StatusAssumption {}
struct Termination;
struct HasTail;
impl StatusAssumption for Termination {}
impl StatusAssumption for HasTail {}

trait EvalStep<T: StatusAssumption = Termination> {
    type Next: EvalStep;
}

trait Bool {}
struct B0;
impl Bool for B0 {}
struct B1;
impl Bool for B1 {}

struct Or<L, R> {
    _lhs: PhantomData<L>,
    _rhs: PhantomData<R>,
}
struct And<L, R> {
    _lhs: PhantomData<L>,
    _rhs: PhantomData<R>,
}
struct XOr<L, R> {
    _lhs: PhantomData<L>,
    _rhs: PhantomData<R>,
}

impl<L, R> EvalStep for Or<L, R>
where
    L: EvalStep,
    R: EvalStep,
    Or<L::Next, R::Next>: EvalStep<HasTail>,
{
    type Next = <Or<L::Next, R::Next> as EvalStep<HasTail>>::Next;
}

impl<L, R> EvalStep<HasTail> for Or<L, R>
where
    L: EvalStep,
    R: EvalStep<HasTail>,
    Or<L::Next, R::Next>: EvalStep<HasTail>,
{
    type Next = <Or<L::Next, R::Next> as EvalStep<HasTail>>::Next;
}
impl<Lhs, Rhs> EvalStep for Add<Lhs, Rhs>
where
    Lhs: EvalStep,
    Rhs: EvalStep,
    Add<Lhs::Next, Rhs::Next>: EvalStep<HasTail>,
{
    type Next = <Add<Lhs::Next, Rhs::Next> as EvalStep<HasTail>>::Next;
}
impl<L, R> EvalStep for And<L, R>
where
    L: EvalStep,
    R: EvalStep,
    And<L::Next, R::Next>: EvalStep<HasTail>,
{
    type Next = <And<L::Next, R::Next> as EvalStep<HasTail>>::Next;
}
impl<L, R> EvalStep<HasTail> for And<L, R>
where
    L: EvalStep,
    R: EvalStep<HasTail>,
    And<L::Next, R::Next>: EvalStep<HasTail>,
{
    type Next = <And<L::Next, R::Next> as EvalStep<HasTail>>::Next;
}
// impl<L, R> EvalStep<Continue> for Or<L, R>
// where
//     L: EvalStep<Continue>,
//     R: EvalStep,
//     Or<L::Next, R>: EvalStep<Continue>,
// {
//     type Next = <Or<L::Next, R> as EvalStep<Continue>>::Next;
// }
// impl<L, R> EvalStep<Continue> for Or<L, R>
// where
//     L: EvalStep,
//     R: EvalStep,
//     Or<L, R>: EvalStep,
// {
//     type Next = <Or<L, R> as EvalStep>::Next;
// }

impl EvalStep for B0 {
    type Next = Self;
}
impl EvalStep for B1 {
    type Next = Self;
}
impl EvalStep for UInt<B1, B0> {
    type Next = Self;
}
impl EvalStep for UInt<B1, B1> {
    type Next = Self;
}
impl EvalStep<HasTail> for Or<B0, B0> {
    type Next = B0;
}
impl EvalStep<HasTail> for Or<B0, B1> {
    type Next = B1;
}
impl EvalStep<HasTail> for Or<B1, B0> {
    type Next = B1;
}
impl EvalStep<HasTail> for Or<B1, B1> {
    type Next = B1;
}
impl EvalStep<HasTail> for And<B0, B0> {
    type Next = B0;
}
impl EvalStep<HasTail> for And<B0, B1> {
    type Next = B0;
}
impl EvalStep<HasTail> for And<B1, B0> {
    type Next = B0;
}
impl EvalStep<HasTail> for And<B1, B1> {
    type Next = B1;
}
impl EvalStep<HasTail> for XOr<B0, B0> {
    type Next = B0;
}
impl EvalStep<HasTail> for XOr<B0, B1> {
    type Next = B1;
}
impl EvalStep<HasTail> for XOr<B1, B0> {
    type Next = B1;
}
impl EvalStep<HasTail> for XOr<B1, B1> {
    type Next = B0;
}

struct Add<L, R> {
    _lhs: PhantomData<L>,
    _rhs: PhantomData<R>,
}
impl<Lhs> EvalStep<HasTail> for Add<UInt<Lhs, B0>, B0>
where
    Lhs: EvalStep,
    UInt<Lhs, B0>: EvalStep,
{
    type Next = UInt<Lhs, B0>;
}
impl<Lhs> EvalStep<HasTail> for Add<UInt<Lhs, B0>, B1>
where
    Lhs: EvalStep,
    UInt<Lhs, B1>: EvalStep,
{
    type Next = UInt<Lhs, B1>;
}
impl<Lhs> EvalStep<HasTail> for Add<UInt<Lhs, B1>, B0>
where
    UInt<Lhs, B1>: EvalStep,
    Lhs: EvalStep,
{
    type Next = UInt<Lhs, B1>;
}
impl EvalStep<HasTail> for Add<B1, B0> {
    type Next = B1;
}
impl EvalStep<HasTail> for Add<B0, B1> {
    type Next = B1;
}
impl EvalStep<HasTail> for Add<B0, B0> {
    type Next = B0;
}
impl EvalStep<HasTail> for Add<B1, B1> {
    type Next = UInt<B1, B0>;
}
// impl<Lhs> EvalStep for Add<UInt<Lhs, B1>, B1> 
// where
//     Lhs: EvalStep,
//     Add<Lhs::Next, B1>: EvalStep,
// {
//     type Next = ();
// }
struct UInt<U, B> {
    _lhs: PhantomData<U>,
    _rhs: PhantomData<B>,
}
// impl<Lhs> EvalStep for Add<UInt<Lhs, B1>, B1>
// where
//     Lhs: EvalStep,
//     (Lhs::Next, B1): EvalStep,
//     Add<Lhs::Next, B1, Continue>: EvalStep,
//     <Add<Lhs::Next, B1, Continue> as EvalStep>::Next: EvalStep,
//
// {
//     type Next = B1;
// }

#[cfg(test)]
mod test {
    use super::*;
    const fn _b0<E: EvalStep<Next = B0>>() {}
    const fn _b1<E: EvalStep<Next = B1>>() {}
    const fn _b2<E: EvalStep<Next = UInt<B1, B0>>>() {}
    const fn _b4<E: EvalStep<Next = UInt<UInt<B1, B0>, B0>>>() {}
    #[test]
    fn eval_ors() {
        const _F: () = _b0::<B0>();
        const _T: () = _b1::<B1>();
        const _F_OR_F: () = _b0::<Or<B0, B0>>();
        const _F_OR_T: () = _b1::<Or<B0, B1>>();
        const _T_OR_F: () = _b1::<Or<B1, B0>>();
        const _T_OR_T: () = _b1::<Or<B1, B1>>();
        const _F_OR_F__OR_F: () = _b0::<Or<Or<B0, B0>, B0>>();
        const _F_OR_F__OR_T: () = _b1::<Or<Or<B0, B0>, B1>>();
        const _F_OR_T__OR_F: () = _b1::<Or<Or<B0, B1>, B0>>();
        const _F__OR_F_OR_F: () = _b0::<Or<B0, Or<B0, B0>>>();
        const _T__OR_F_OR_F: () = _b1::<Or<B1, Or<B0, B0>>>();
        const _F__OR_T_OR_F: () = _b1::<Or<B0, Or<B1, B0>>>();
    }
    #[test]
    fn eval_add() {
        const _0_ADD_0: () = _b0::<Add<B0, B0>>();
        const _1_ADD_0: () = _b1::<Add<B1, B0>>();
        const _1_ADD_1: () = _b2::<Add<B1, B1>>();
        const _3_ADD_1: () = _b4::<Add<UInt<B1, B1>, B1>>();
        // const _B0_B1: () = evaled_b1::<UInt<B0, B1, EvaluatedAst>>();
        // const _B1_B0: () = evaled_b2::<UInt<B1, B0, EvaluatedAst>>();
        // const _B1_B1: () = evaled_b3::<UInt<B1, B1, EvaluatedAst>>();
        // const _B0_B0_B0: () = evaled_b0::<UInt<UInt<B0, B0, EvaluatedAst>, B0, EvaluatedAst>>();
    }
    #[test]
    fn and() {
        const _F_AND_F: () = _b0::<And<B0, B0>>();
        const _F_AND_T: () = _b0::<And<B0, B1>>();
        const _T_AND_F: () = _b0::<And<B1, B0>>();
        const _T_AND_T: () = _b1::<And<B1, B1>>();
        const _F_AND_F__AND_F: () = _b0::<And<And<B0, B0>, B0>>();
        const _F_AND_F__AND_T: () = _b0::<And<And<B0, B0>, B1>>();
        const _T_AND_T__Or_F: () = _b1::<Or<And<B1, B1>, B0>>();
        const _F__AND_F_AND_F: () = _b0::<And<B0, And<B0, B0>>>();
        const _T__AND_F_AND_F: () = _b0::<And<B1, And<B0, B0>>>();
        const _F__AND_T_AND_F: () = _b0::<And<B0, And<B1, B0>>>();
    }
}
