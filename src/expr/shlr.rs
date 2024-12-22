use crate::{
    op_types::{ShLExp, ShRExp, SubExp},
    val_types::{BitStrLit, BitString, NumberVal, _0, _1},
    Base, NumExpr, NumRet, U1,
};

impl<L, R> NumExpr for ShLExp<L, R>
where
    L: NumExpr,
    R: NumExpr,
    ShLExp<L::Ret, R::Ret, Base>: NumExpr,
{
    type Ret = NumRet<ShLExp<L::Ret, R::Ret, Base>>;
}

impl<B> NumExpr for ShLExp<B, _0, Base>
where
    B: NumberVal,
{
    type Ret = B;
}
impl<B, N> NumExpr for ShLExp<B, N, Base>
where
    B: BitStrLit,
    SubExp<N, _1>: NumExpr,
    ShLExp<BitString<B, _0>, NumRet<SubExp<N, _1>>>: NumExpr,
{
    type Ret = NumRet<ShLExp<BitString<B, _0>, NumRet<SubExp<N, _1>>>>;
}
impl<L, R> NumExpr for ShRExp<L, R>
where
    L: NumExpr,
    R: NumExpr,
    ShRExp<L::Ret, R::Ret, Base>: NumExpr,
{
    type Ret = NumRet<ShRExp<L::Ret, R::Ret, Base>>;
}

impl<B> NumExpr for ShRExp<B, _0, Base>
where
    B: NumberVal,
{
    type Ret = B;
}
impl<Bs, B, N> NumExpr for ShRExp<BitString<Bs, B>, N, Base>
where
    Bs: NumExpr,
    SubExp<N, _1>: NumExpr,
    ShRExp<Bs::Ret, NumRet<SubExp<N, _1>>, Base>: NumExpr,
{
    type Ret = NumRet<ShRExp<Bs::Ret, NumRet<SubExp<N, _1>>, Base>>;
}
impl<N> NumExpr for ShRExp<U1, N, Base>
where
    SubExp<N, _1>: NumExpr,
{
    type Ret = _0;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{test_res::*, U0, U1, U2, U3, U9};
    #[allow(non_upper_case_globals)]
    #[test]
    fn eval_msb() {
        const _0_ShL_0: () = _b0::<ShLExp<_0, U0>>();
        const _1_ShL_0: () = _b1::<ShLExp<_1, U0>>();
        const _1_ShL_1: () = _b2::<ShLExp<_1, U1>>();
        const _1_ShR_1: () = _b0::<ShRExp<_1, U1>>();
        const _1_ShR_0: () = _b1::<ShRExp<_1, U0>>();

        const _10_ShR_0: () = _b2::<ShRExp<BitString<_1, _0>, U0>>();
        const _11_ShR_0: () = _b3::<ShRExp<BitString<_1, _1>, U0>>();
        const _10_ShR_1: () = _b1::<ShRExp<BitString<_1, _0>, U1>>();
        const _11_ShR_1: () = _b1::<ShRExp<BitString<_1, _1>, U1>>();

        const _1_ShL_2: () = _b1::<ShLExp<_1, U0>>();
        const _1_ShL_3: () = _b2::<ShLExp<_1, U1>>();
        const _3_ShR_2: () = _b0::<ShRExp<U3, U2>>();
        const _9_ShR_3: () = _b1::<ShRExp<U9, U3>>();
        // const _MSB_10: () = _b1::<MSB<BitString<_1, _0>>>();
        // const _MSB_11: () = _b1::<MSB<BitString<_1, _1>>>();
        // const _MSB_100: () = _b1::<MSB<BitString<BitString<_1, _0>, _1>>>();
    }
}
