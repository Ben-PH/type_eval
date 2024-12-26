use crate::{
    prelude::{False, True, IF},
    BoolExpr, BoolRet, NumExpr, NumRet,
    _inners::_Base,
};

impl<C, T, F> NumExpr for IF<C, T, F>
where
    C: BoolExpr,
    BoolRet<C>: BoolExpr,
    IF<C::Ret, T, F, _Base>: NumExpr,
{
    type Ret = NumRet<IF<C::Ret, T, F, _Base>>;
}

impl<T, F> NumExpr for IF<True, T, F, _Base>
where
    T: NumExpr,
{
    type Ret = NumRet<T>;
}
impl<T, F> NumExpr for IF<False, T, F, _Base>
where
    F: NumExpr,
{
    type Ret = NumRet<F>;
}
impl<C, T, F> BoolExpr for IF<C, T, F>
where
    C: BoolExpr,
    BoolRet<C>: BoolExpr,
    IF<C::Ret, T, F, _Base>: BoolExpr,
{
    type Ret = BoolRet<IF<C::Ret, T, F, _Base>>;
}

impl<T, F> BoolExpr for IF<True, T, F, _Base>
where
    T: BoolExpr,
{
    type Ret = BoolRet<T>;
}
impl<T, F> BoolExpr for IF<False, T, F, _Base>
where
    F: BoolExpr,
{
    type Ret = BoolRet<F>;
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        num_vals::{U0, U1, U2},
        prelude::{MulExp, GT, LT, U5},
        test_res::*,
    };
    #[test]
    fn eval_add() {
        const _IF_T_1_2: () = _b1::<IF<LT<U0, U1>, U1, U2>>();
        const _IF_F_1_2: () = _b2::<IF<GT<U0, U1>, U1, U2>>();
        const _ARITH: () = _b10::<IF<LT<U1, U2>, MulExp<U2, U5>, U0>>();
    }
}
