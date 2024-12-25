use crate::prelude::{B as BitString, _0, _1};
pub trait _ExprMode {}
pub struct _Recurse;
pub struct _Base;
impl _ExprMode for _Recurse {}
impl _ExprMode for _Base {}

/// Defines the things that can be appended to a BitStrLit to make a concrete Number representation
/// form a BitString
pub trait _BitLit {}
impl _BitLit for _0 {}
impl _BitLit for _1 {}

/// When paired with a `BitLit` terminator in a `BitString`, forms a concrete representation of an
/// integer.
pub trait _BitStrLit {}

/// A conceptual "Base Case" for the leading bits to represent a Number of more than one bit.
impl _BitStrLit for _1 {}

/// Any BitString is a valid BitStrLit if the leading bits form a valid BitStrLit, and the
/// terminating bit is a valid BitLit
///
/// This is functionally a recursive definition, with the base-case being `_1` and the recursive
/// case being any multi-bit bitstring.
///
/// effectively, 0b100101 => (((((1), 0), 0), 1), 0) with `1` appended for the terminating BitLit
impl<Bs, B> _BitStrLit for BitString<Bs, B>
where
    Bs: _BitStrLit,
    B: _BitLit,
{
}
