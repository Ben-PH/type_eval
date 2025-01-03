use core::marker::PhantomData;

use crate::{
    BoolExpr, OrdExpr,
    _inners::{_ExprMode, _Recurse},
};

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

pub trait OrdVal {}

pub struct Less;
impl OrdVal for Less {}
pub struct Gr8r;
impl OrdVal for Gr8r {}
pub struct Eq;
impl OrdVal for Eq {}

impl OrdExpr for Less {
    type Ret = Self;
}
impl OrdExpr for Gr8r {
    type Ret = Self;
}
impl OrdExpr for Eq {
    type Ret = Self;
}

pub struct IF<C: BoolExpr, T, F, M: _ExprMode = _Recurse> {
    _if_check: PhantomData<C>,
    _true_branch: PhantomData<T>,
    _false_branch: PhantomData<F>,
    _m: PhantomData<M>,
}
pub struct LT<L, R, M: _ExprMode = _Recurse> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _m: PhantomData<M>,
}
pub struct GT<L, R, M: _ExprMode = _Recurse> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _m: PhantomData<M>,
}
#[allow(clippy::upper_case_acronyms)]
pub struct LTE<L, R, M: _ExprMode = _Recurse> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _m: PhantomData<M>,
}
#[allow(clippy::upper_case_acronyms)]
pub struct GTE<L, R, M: _ExprMode = _Recurse> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _m: PhantomData<M>,
}
pub struct EQ<L, R, M: _ExprMode = _Recurse> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _m: PhantomData<M>,
}
#[allow(clippy::upper_case_acronyms)]
pub struct AND<L, R, M: _ExprMode = _Recurse> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _m: PhantomData<M>,
}
pub struct OR<L, R, M: _ExprMode = _Recurse> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _m: PhantomData<M>,
}
pub struct NOT<B> {
    _bool: PhantomData<B>,
}

pub struct ORD<L, R, M: _ExprMode = _Recurse> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _m: PhantomData<M>,
}
