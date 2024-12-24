use crate::{
    ctrl_types::{False, True, AND, OR},
    Base, BoolExpr, BoolRet,
};

impl<L, R> BoolExpr for AND<L, R>
where
    L: BoolExpr,
    R: BoolExpr,
    AND<L::Ret, R::Ret, Base>: BoolExpr,
{
    type Ret = BoolRet<AND<L::Ret, R::Ret, Base>>;
}

impl BoolExpr for AND<True, True, Base> {
    type Ret = True;
}
impl BoolExpr for AND<True, False, Base> {
    type Ret = False;
}
impl BoolExpr for AND<False, True, Base> {
    type Ret = False;
}
impl BoolExpr for AND<False, False, Base> {
    type Ret = False;
}
impl<L, R> BoolExpr for OR<L, R>
where
    L: BoolExpr,
    R: BoolExpr,
    OR<L::Ret, R::Ret, Base>: BoolExpr,
{
    type Ret = BoolRet<OR<L::Ret, R::Ret, Base>>;
}

impl BoolExpr for OR<True, True, Base> {
    type Ret = True;
}
impl BoolExpr for OR<True, False, Base> {
    type Ret = True;
}
impl BoolExpr for OR<False, True, Base> {
    type Ret = True;
}
impl BoolExpr for OR<False, False, Base> {
    type Ret = False;
}
