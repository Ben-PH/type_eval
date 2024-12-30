//! `type_eval` provides data and evaluation types to the compiler, unburdened
//! by the limitations of execution code semantics.
//!
//!
//! ### Erase requirement of machine representation:
//!
//! ```rust
//! # use type_eval::{prelude::*, BoolExpr, NumExpr};
//! #[deprecated(note="use `NumRet::<DivExpr>::$TYPE` instead")]
//! use core::ops::Div;
//! use core::cmp::Eq;
//! pub const fn safe_div_u32(numerator: u32, denominator: u32) -> Option<u32> {
//!     if denominator == 0 {
//!         None
//!     } else {
//!         Some(numerator / denominator)
//!     }
//! }
//!
//! use type_eval::{NumRet, MemRep};
//! fn main() {
//!     // let safe_dived = safe_div(4u16, 2usize);
//!     let safe_dived = NumRet::<DivExp<U4, U2>>::MU32;
//! }
//! ```
//!
//! ### Calculated array length
//!
//!```
//! use type_eval::{num_vals::*, prelude::*, NumRet, MemRep};
//! type Area<Width, Height> = NumRet::<MulExp<Width, Height>>;
//! fn area_array<const S: usize>(array: [u8; S])
//! { }
//!
//! fn main() {
//!     area_array([0u8; Area::<U3, U4>::MUSIZE]);
//! }
//!
//!```
//! ### Enforce predicates/preconditions into the type-system:
//!
//!```
//! # use type_eval::{prelude::*, BoolExpr, NumExpr};
//!
//! // define functions that carry a proof of evaluation result
//! const fn _b0<E: NumExpr<Ret = U0>>() {}
//! const fn _b1<E: NumExpr<Ret = U1>>() {}
//! const fn _b2<E: NumExpr<Ret = U2>>() {}
//! const fn _b3<E: NumExpr<Ret = U3>>() {}
//! #[allow(non_upper_case_globals)]
//! fn add_sub() {
//!     const _2_ADD_1__SUB_3: () = _b0::<SubExp<AddExp<U2, U1>, U3>>();
//!     const _6_SUB__1_ADD_3: () = _b2::<SubExp<U6, AddExp<U1, U3>>>();
//!     // const COMPILE_FAIL: () = _b2::<SubExp<U7, AddExp<U1, U3>>>();
//!
//! }
//!
//! fn shift_msb() {
//!     const _MSB__2_SHL_1: () = _b2::<MSB<ShLExp<U2, U1>>>();
//!     const _MSB__2_SHL_0: () = _b1::<MSB<ShLExp<U2, U0>>>();
//!
//!     const _MSB_4__SUB__MSB_3: () = _b1::<SubExp<MSB<U4>, MSB<U3>>>();
//!     const _MSB_4__ADD__MSB_3: () = _b3::<AddExp<MSB<U4>, MSB<U3>>>();
//! }
//!
//! fn ifs() {
//!     // if (1/2) < 1 {0} else {1 / 0}
//!     const _IF_T_U0_DIV0: () = _b0::<IF<LT<DivExp<U1, U2>, U1>, U0, DivExp<U1, U0>>>();
//!     // div-by-zero is a compile fail
//!     // const _COMPILE_FAIL: () = _b0::<DivExp<U1, U0>>();
//! }
//!
//! fn lt4() {
//!     // define a function that carries a proof of evaluaton result < 6
//!     const fn _lt6<E: NumExpr>()
//!     where
//!         LT<E::Ret, U6>: BoolExpr<Ret = True>,
//!     {
//!     }
//!
//!     // 2 + 3 < 6
//!     const _5_LT_6: () = _lt6::<AddExp<U2, U3>>();
//!     // 2 * 3 !< 6
//!     // const FAIL: () = _lt6::<MulExp<U2, U3>>();
//! }
//!```
//! let's observe the compilation failure:
//! ```compile_fail
//! # use type_eval::{prelude::*, BoolExpr, NumExpr};
//! # #[allow(non_upper_case_globals)]
//! fn lt4() {
//!     // define a function that carries a proof of evaluaton result < 6
//!     const fn _lt6<E: NumExpr>()
//!     where
//!         LT<E::Ret, U6>: BoolExpr<Ret = True>,
//!     {}
//!
//!     // 2 * 3 !< 6
//!     const FAIL: () = _lt6::<MulExp<U2, U3>>();
//! }
//!```
//!
//! ## Propogate predicates/preconditions downstream via the type-system:
//!
//! Enforce co-veraiance of predicates when extending traits
//! In this example, a submatrix of XS/XY dimensions wants to be contained
//! within a "parent" matrix of X/Y dimension
//!
//! The top-left corner (let's call it XC/YCof the sub-matrix is constrained
//! so it's wholely contained within the parent matrix. i.e.
//!
//! `XC + XS < X && YC + YS < Y`
//!
//!```rust
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
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
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

// TODO: break down the traits so that memory-representations are only implemented
// where the type fits in said representation.
// e.g. u8 is only valid when the bit-tring is 0..=255
/// In-memory representation, e.g. [u32]
/// ```rust
/// use type_eval::num_vals::U4;
/// use type_eval::MemRep;
/// assert_eq!(4, U4::MU32)
/// ```
#[allow(clippy::cast_possible_truncation)]
pub trait MemRep {
    const MU128: u128;
    const MUSIZE: usize = Self::MU128 as usize;
    const MU64: u64 = Self::MU128 as u64;
    const MU32: u32 = Self::MU128 as u32;
    const MU16: u16 = Self::MU128 as u16;
    const MU8: u8 = Self::MU128 as u8;
}
/// An expression returning a [`prelude::NumberVal`]
pub trait NumExpr {
    type Ret: val_types::NumberVal;
}
#[allow(clippy::doc_markdown)]
/// <T as [NumExpr]>::Ret helper
pub type NumRet<T> = <T as NumExpr>::Ret;

/// An expression returning a [`prelude::BoolVal`]
pub trait BoolExpr {
    type Ret: prelude::BoolVal;
}
#[allow(clippy::doc_markdown)]
/// <T as [BoolExpr]>::Ret helper
pub type BoolRet<T> = <T as BoolExpr>::Ret;
/// An expression returning a [`prelude::BoolVal`]
pub trait OrdExpr {
    type Ret: prelude::OrdVal;
}
#[allow(clippy::doc_markdown)]
/// <T as [BoolExpr]>::Ret helper
pub type OrdRet<T> = <T as OrdExpr>::Ret;

#[allow(clippy::used_underscore_items)]
#[cfg(test)]
mod test_res {
    use super::{
        ctrl_types::{Eq, False, Gr8r, Less, True},
        num_vals::*,
        BoolExpr, NumExpr, OrdExpr,
    };

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
    pub(crate) const fn _b10<E: NumExpr<Ret = U10>>() {}
    pub(crate) const fn _b21<E: NumExpr<Ret = U21>>() {}
    pub(crate) const fn _b35<E: NumExpr<Ret = U35>>() {}

    pub(crate) const fn _t<E: BoolExpr<Ret = True>>() {}
    pub(crate) const fn _f<E: BoolExpr<Ret = False>>() {}
    pub(crate) const fn _lt<E: OrdExpr<Ret = Less>>() {}
    pub(crate) const fn _eq<E: OrdExpr<Ret = Eq>>() {}
    pub(crate) const fn _gt<E: OrdExpr<Ret = Gr8r>>() {}
}
