use core::marker::PhantomData;

use crate::{Ast, Expr, Mode};

pub trait Number {}

/// Literal representation of the 0-bit
pub struct _0;
/// Literal representation of the 1-bit
pub struct _1;
/// Literal representation of a bit-string
pub struct BitString<Bs, B, M: Mode = Ast> {
    _bits: PhantomData<Bs>,
    _last_bit: PhantomData<B>,
    _m: PhantomData<M>,
}

/// Allows a 0-bit literal to represent a concrete number
impl Number for _0 {}
/// Allows a 1-bit literal to represent a concrete number
impl Number for _1 {}
/// For a BitString to be considered as a valid concrete number, it must be prepended with
/// something that implements BitStrLit, and appended by a BitLit
impl<Bs, B> Number for BitString<Bs, B>
where
    Bs: BitStrLit,
    B: BitLit,
{
}

/// Defines the things that can be appended to a BitStrLit to make a concrete Number representation
/// form a BitString
pub trait BitLit {}
impl BitLit for _0 {}
impl BitLit for _1 {}

/// When paired with a `BitLit` terminator in a `BitString`, forms a concrete representation of an
/// integer.
pub trait BitStrLit {}

/// A conceptual "Base Case" for the leading bits to represent a Number of more than one bit.
impl BitStrLit for _1 {}

/// Any BitString is a valid BitStrLit if the leading bits form a valid BitStrLit, and the
/// terminating bit is a valid BitLit
///
/// This is functionally a recursive definition, with the base-case being `_1` and the recursive
/// case being any multi-bit bitstring.
///
/// effectively, 0b100101 => (((((1), 0), 0), 1), 0) with `1` appended for the terminating BitLit
impl<Bs, B> BitStrLit for BitString<Bs, B>
where
    Bs: BitStrLit,
    B: BitLit,
{
}

/// A BitString that is already a valid number evaluates to itself
impl<Lhs, B> Expr for BitString<Lhs, B>
where
    Lhs: BitStrLit,
    B: BitLit,
{
    type Output = Self;
}

/// subtractions can lead to `BitString`s with invalid number representations. For example:
///
/// > `2 - 1` == `0b10 - 0b1` == (`0b1 - 0b1`, 0b1)
///
/// This means `2 - 1` Evaluates to `BitString<_0, _0>`, which is not a valid expression output.
/// Leaving it just at that, `Sub<2, 1>` would not be able to have a valid `Expr` implementation,
/// as its `Output` is not a valid `Number`. It needs to be trimmed
///
/// This `Expr` implementation trimms away leading 0-bits, thus allowing for `Sub<_, 1>`
/// implementations.
impl<B> Expr for BitString<_0, B>
where
    B: Number,
{
    type Output = B;
}

impl Expr for _0 {
    type Output = _0;
}
impl Expr for _1 {
    type Output = _1;
}

#[cfg(test)]
mod test {
    use crate::{
        test_res::*,
        val_types::{BitString, _0, _1},
    };
    #[test]
    fn eval_add() {
        const _0_0: () = _b0::<BitString<_0, _0>>();
        const _0_1: () = _b1::<BitString<_0, _1>>();
    }
}
