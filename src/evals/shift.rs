use crate::{
    op_types::{ShL, ShR},
    Bit, Eval, Formula, UInt, Unsigned, B0,
};

impl<U> Formula for ShL<U, Eval>
where
    U: Unsigned,
    UInt<U, B0>: Unsigned + Formula,
{
    type FOutput = UInt<U, B0>;
}

impl<U, B> Formula for ShR<UInt<U, B>, Eval>
where
    B: Bit,
    U: Unsigned + Formula,
    UInt<U, B0>: Unsigned + Formula,
{
    type FOutput = U;
}
