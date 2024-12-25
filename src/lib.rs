//! ```rust
//! use type_eval::{prelude::*, BoolExpr};
//! // A keyboard matrix trait.
//! trait KBMatrix
//! // TODO: impl non-zero
//! // where
//! //     GTE<Self::Width, U1>: BoolExpr<Ret = True>,
//! //     GTE<Self::Height, U1>: BoolExpr<Ret = True>,
//! {
//!     type Width: NumberVal;
//!     type Height: NumberVal;
//! }

//! // a convenience definition to get a parent-matrix width and height
//! type ParentWidth<S: SubMatrix> = <S::Parent as KBMatrix>::Width;
//! type ParentHeight<S: SubMatrix> = <S::Parent as KBMatrix>::Height;

//! // You can define a sub matrix such that the compiler will cause an error
//! // if it's not contained within the parent matrix
//! trait SubMatrix
//! where
//!     GTE<Self::Width, U1>: BoolExpr<Ret = True>,
//!     GTE<Self::Height, U1>: BoolExpr<Ret = True>,
//!     // this is effectively `assert!(x + width <= parent.width)`, but in the type system
//!     LTE<AddExp<Self::Width, Self::XLoc>, ParentWidth<Self>>: BoolExpr<Ret = True>,
//!     // this is effectively `assert!(y + height <= parent.height)`, but in the type system
//!     LTE<AddExp<Self::Height, Self::YLoc>, ParentHeight<Self>>: BoolExpr<Ret = True>,
//! {
//!     type Parent: KBMatrix;
//!     type Width: NumberVal;
//!     type XLoc: NumberVal;
//!     type Height: NumberVal;
//!     type YLoc: NumberVal;
//! }
//! ```
#![no_std]
pub mod prelude {
    pub use crate::{ctrl_types::*, num_vals::*, op_types::*, val_types::*};
}
/// Implementors of [`BoolExpr`]
pub mod ctrl_types;
/// Implementors of [`NumExpr`]
pub mod op_types;
/// Constructors for numbers expressed at the type-level
pub mod val_types;

/// Integers expressed as types, e.g.: [`prelude::U0`], [`prelude::U1`]
pub mod num_vals {
    include!(concat!(env!("OUT_DIR"), "/consts.rs"));
}
mod expr;

/// Inner implementation types. Generally not intended for end-use
pub mod _inners;

/// An expression returning a [`prelude::NumberVal`]
pub trait NumExpr {
    type Ret: val_types::NumberVal;
}
/// <T as [NumExpr]>::Ret helper
pub type NumRet<T> = <T as NumExpr>::Ret;

/// An expression returning a [`prelude::BoolVal`]
pub trait BoolExpr {
    type Ret: prelude::BoolVal;
}
/// <T as [BoolExpr]>::Ret helper
pub type BoolRet<T> = <T as BoolExpr>::Ret;

#[cfg(test)]
mod test_res {
    use ctrl_types::{False, True};
    use val_types::B as BitString;
    use val_types::_0;

    use crate::num_vals::*;
    use crate::val_types::_1;

    use super::*;
    pub(crate) const fn _b0<E: NumExpr<Ret = U0>>() {}
    pub(crate) const fn _b1<E: NumExpr<Ret = U1>>() {}
    pub(crate) const fn _b2<E: NumExpr<Ret = U2>>() {}
    pub(crate) const fn _b3<E: NumExpr<Ret = U3>>() {}
    pub(crate) const fn _b4<E: NumExpr<Ret = U4>>() {}
    pub(crate) const fn _b5<E: NumExpr<Ret = U5>>() {}
    pub(crate) const fn _b6<E: NumExpr<Ret = U6>>() {}
    pub(crate) const fn _b7<E: NumExpr<Ret = U7>>() {}
    pub(crate) const fn _b8<E: NumExpr<Ret = U8>>() {}
    pub(crate) const fn _b9<E: NumExpr<Ret = U9>>() {}

    pub(crate) const fn _t<E: BoolExpr<Ret = True>>() {}
    pub(crate) const fn _f<E: BoolExpr<Ret = False>>() {}
    #[test]
    fn eval_add() {
        const _0_0: () = _b0::<BitString<_0, _0>>();
        const _0_1: () = _b1::<BitString<_0, _1>>();
    }
}
