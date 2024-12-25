use crate::{
    _inners::_Base,
    ctrl_types::{False, True, AND, OR},
    BoolExpr, BoolRet,
};

impl<L, R> BoolExpr for AND<L, R>
where
    L: BoolExpr,
    R: BoolExpr,
    AND<L::Ret, R::Ret, _Base>: BoolExpr,
{
    type Ret = BoolRet<AND<L::Ret, R::Ret, _Base>>;
}

impl BoolExpr for AND<True, True, _Base> {
    type Ret = True;
}
impl BoolExpr for AND<True, False, _Base> {
    type Ret = False;
}
impl BoolExpr for AND<False, True, _Base> {
    type Ret = False;
}
impl BoolExpr for AND<False, False, _Base> {
    type Ret = False;
}
impl<L, R> BoolExpr for OR<L, R>
where
    L: BoolExpr,
    R: BoolExpr,
    OR<L::Ret, R::Ret, _Base>: BoolExpr,
{
    type Ret = BoolRet<OR<L::Ret, R::Ret, _Base>>;
}

impl BoolExpr for OR<True, True, _Base> {
    type Ret = True;
}
impl BoolExpr for OR<True, False, _Base> {
    type Ret = True;
}
impl BoolExpr for OR<False, True, _Base> {
    type Ret = True;
}
impl BoolExpr for OR<False, False, _Base> {
    type Ret = False;
}
