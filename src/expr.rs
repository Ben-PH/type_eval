use crate::{
    val_types::{BitLit, BitStrLit, BitString, Number, _0},
    Expr,
};

mod add;
mod sub;
// Evaluation of a straight-up bit-string literal
impl<Lhs, B> Expr for BitString<Lhs, B>
where
    Lhs: BitStrLit,
    B: BitLit,
{
    type Output = Self;
}

// Evaluation of a straight-up bit-string literal
impl<B> Expr for BitString<_0, B>
where
    B: Number,
{
    type Output = B;
}

#[cfg(test)]
mod test {
    use crate::val_types::_1;

    use super::*;
    const fn _b0<E: Expr<Output = _0>>() {}
    const fn _b1<E: Expr<Output = _1>>() {}
    const fn _b2<E: Expr<Output = BitString<_1, _0>>>() {}
    const fn _b3<E: Expr<Output = BitString<_1, _1>>>() {}
    const fn _b4<E: Expr<Output = BitString<BitString<_1, _0>, _0>>>() {}
    const fn _b5<E: Expr<Output = BitString<BitString<_1, _0>, _1>>>() {}
    const fn _b6<E: Expr<Output = BitString<BitString<_1, _1>, _0>>>() {}
    #[test]
    fn eval_add() {
        const _0_0: () = _b0::<BitString<_0, _0>>();
        const _0_1: () = _b1::<BitString<_0, _1>>();
    }
}
