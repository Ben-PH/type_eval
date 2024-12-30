use crate::prelude::{B, _0, _1};
pub trait _ExprMode {}
pub struct _Recurse;
pub struct _Base;
impl _ExprMode for _Recurse {}
impl _ExprMode for _Base {}

/// Defines the things that can be appended to a [`_BitStrLit`] to make a concrete Number representation
/// form a [`B`]itString
pub trait _BitLit {}
impl _BitLit for _0 {}
impl _BitLit for _1 {}

/// When paired with a `BitLit` terminator in a [`B`]itString, forms a concrete representation of an
/// integer.
pub trait _BitStrLit {}

/// A conceptual "Base Case" for the leading bits to represent a Number of more than one bit.
impl _BitStrLit for _1 {}

/// Any [`B`]itString is a valid [`_BitStrLit`] if the leading bits form a valid [`_BitStrLit`], and the
/// terminating bit is a valid [`BitLit`]
///
/// This is functionally a recursive definition, with the base-case being `_1` and the recursive
/// case being any multi-bit bitstring.
///
/// effectively, 0b100101 => (10010) ++ 1 => ((1001) ++ 0) ++ 1 => ... => (((((1, 0), 0), 1), 0), 1)
impl<Bs, Bt> _BitStrLit for B<Bs, Bt>
where
    Bs: _BitStrLit,
    Bt: _BitLit,
{
}
