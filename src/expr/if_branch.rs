use crate::{
    ctrl_types::{BoolVal, False, True, IF},
    BoolExpr, BoolRet, NumExpr, NumRet, OrdExpr, OrdRet,
    _inners::_Base,
    prelude::NumberVal,
};

impl<C, T, F> NumExpr for IF<C, T, F>
where
    C: BoolExpr,
    BoolRet<C>: BoolVal,
    IF<C::Ret, T, F, _Base>: NumExpr,
    NumRet<IF<C::Ret, T, F, _Base>>: NumberVal,
{
    type Ret = NumRet<IF<C::Ret, T, F, _Base>>;
}

impl<T, F> NumExpr for IF<True, T, F, _Base>
where
    T: NumExpr,
    NumRet<T>: NumberVal,
{
    type Ret = NumRet<T>;
}
impl<T, F> NumExpr for IF<False, T, F, _Base>
where
    F: NumExpr,
    NumRet<F>: NumberVal,
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
impl<C, T, F> OrdExpr for IF<C, T, F>
where
    C: BoolExpr,
    BoolRet<C>: BoolExpr,
    IF<C::Ret, T, F, _Base>: OrdExpr,
{
    type Ret = OrdRet<IF<C::Ret, T, F, _Base>>;
}

impl<T, F> OrdExpr for IF<True, T, F, _Base>
where
    T: OrdExpr,
{
    type Ret = OrdRet<T>;
}
impl<T, F> OrdExpr for IF<False, T, F, _Base>
where
    F: OrdExpr,
{
    type Ret = OrdRet<F>;
}
#[cfg(test)]
#[allow(clippy::used_underscore_items)]
mod test {
    use super::*;
    use crate::{
        num_vals::{U0, U1, U2, U3},
        op_types::{AddExp, DivExp},
        prelude::{MulExp, GT, LT, U5},
        test_res::*,
    };
    #[test]
    fn eval_if() {
        const _IF_T_1_2: () = _b1::<IF<LT<U0, U1>, U1, U2>>();
        const _IF_F_1_2: () = _b2::<IF<GT<U0, U1>, U1, U2>>();
    }
    #[test]
    fn eval_if_nested() {
        const _ARITH: () = _b10::<IF<LT<AddExp<U2, U3>, MulExp<U2, U3>>, MulExp<U2, U5>, U0>>();
        const _IF_T_1ADD2_2: () = _b3::<NumRet<IF<LT<U0, U1>, AddExp<U1, U2>, U2>>>();
    }
    #[test]
    fn eval_if_div_0() {
        const _IF_T_1_DIV0: () = _b1::<IF<LT<U0, U1>, U1, DivExp<U1, U0>>>();
    }
}
