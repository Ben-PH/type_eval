use crate::{
    op_types::{ShL, ShR, Sub},
    val_types::{BitStrLit, BitString, NumberVal, _0, _1},
    Base, NumExpr, NumRet,
};

impl<L, R> NumExpr for ShL<L, R>
where
    L: NumExpr,
    R: NumExpr,
    ShL<L::Ret, R::Ret, Base>: NumExpr,
{
    type Ret = NumRet<ShL<L::Ret, R::Ret, Base>>;
}

impl<B> NumExpr for ShL<B, _0, Base>
where
    B: NumberVal,
{
    type Ret = B;
}
impl<B, N> NumExpr for ShL<B, N, Base>
where
    B: BitStrLit,
    Sub<N, _1>: NumExpr,
    ShL<BitString<B, _0>, NumRet<Sub<N, _1>>>: NumExpr,
{
    type Ret = NumRet<ShL<BitString<B, _0>, NumRet<Sub<N, _1>>>>;
}
impl<L, R> NumExpr for ShR<L, R>
where
    L: NumExpr,
    R: NumExpr,
    ShR<L::Ret, R::Ret, Base>: NumExpr,
{
    type Ret = NumRet<ShR<L::Ret, R::Ret, Base>>;
}

impl<B> NumExpr for ShR<B, _0, Base>
where
    B: NumberVal,
{
    type Ret = B;
}
impl<Bs, B, N> NumExpr for ShR<BitString<Bs, B>, N, Base>
where
    Bs: NumExpr,
    Sub<N, _1>: NumExpr,
    ShR<Bs::Ret, NumRet<Sub<N, _1>>, Base>: NumExpr,
{
    type Ret = NumRet<ShR<Bs::Ret, NumRet<Sub<N, _1>>, Base>>;
}
impl<N> NumExpr for ShR<_1, N, Base>
where
    Sub<N, _1>: NumExpr,
{
    type Ret = _0;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_res::*;
    #[allow(non_upper_case_globals)]
    #[test]
    fn eval_msb() {
        const _0_ShL_0: () = _b0::<ShL<_0, _0>>();
        const _1_ShL_0: () = _b1::<ShL<_1, _0>>();
        const _1_ShL_1: () = _b2::<ShL<_1, _1>>();
        const _1_ShR_1: () = _b0::<ShR<_1, _1>>();
        const _1_ShR_0: () = _b1::<ShR<_1, _0>>();

        const _10_ShR_0: () = _b2::<ShR<BitString<_1, _0>, _0>>();
        const _11_ShR_0: () = _b3::<ShR<BitString<_1, _1>, _0>>();
        const _10_ShR_1: () = _b1::<ShR<BitString<_1, _0>, _1>>();
        const _11_ShR_1: () = _b1::<ShR<BitString<_1, _1>, _1>>();

        const _1_ShL_2: () = _b1::<ShL<_1, _0>>();
        const _1_ShL_3: () = _b2::<ShL<_1, _1>>();
        const _3_ShR_2: () = _b0::<ShR<BitString<_1, _1>, BitString<_1, _0>>>();
        const _9_ShR_3: () =
            _b1::<ShR<BitString<BitString<BitString<_1, _0>, _0>, _1>, BitString<_1, _1>>>();
        // const _MSB_10: () = _b1::<MSB<BitString<_1, _0>>>();
        // const _MSB_11: () = _b1::<MSB<BitString<_1, _1>>>();
        // const _MSB_100: () = _b1::<MSB<BitString<BitString<_1, _0>, _1>>>();
    }
}
