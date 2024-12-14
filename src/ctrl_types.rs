use core::marker::PhantomData;

use crate::{BoolExpr, ExprMode, Recurse};

pub trait BoolVal {}

pub struct True;
impl BoolVal for True {}
pub struct False;
impl BoolVal for False {}

impl BoolExpr for True {
    type Ret = Self;
}
impl BoolExpr for False {
    type Ret = Self;
}

pub struct LT<L, R, M: ExprMode = Recurse> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _m: PhantomData<M>,
}
pub struct GT<L, R, M: ExprMode = Recurse> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _m: PhantomData<M>,
}
#[allow(clippy::upper_case_acronyms)]
pub struct LTE<L, R, M: ExprMode = Recurse> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _m: PhantomData<M>,
}
#[allow(clippy::upper_case_acronyms)]
pub struct GTE<L, R, M: ExprMode = Recurse> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _m: PhantomData<M>,
}
pub struct EQ<L, R, M: ExprMode = Recurse> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _m: PhantomData<M>,
}
#[allow(clippy::upper_case_acronyms)]
pub struct AND<L, R, M: ExprMode = Recurse> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _m: PhantomData<M>,
}
pub struct OR<L, R, M: ExprMode = Recurse> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _m: PhantomData<M>,
}
