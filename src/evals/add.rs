//! basics operation outputs such as B1 + B0 and visa versa, boolean algebras, etc

use core::marker::PhantomData;

use crate::{Ast, Eval, Formula, Mode, UInt, B0, B1, U2};

pub struct Add<L, R, M: Mode = Ast> {
    _lhs: PhantomData<L>,
    _rhs: PhantomData<R>,
    _mode: PhantomData<M>,
}

impl Formula for Add<B0, B1> {
    type Output = B1;
}
impl Formula for Add<B1, B1> {
    type Output = U2;
}

impl<L> Formula for Add<UInt<L, B1>, B1, Eval>
where
    L: Formula,
    Add<L::Output, B1>: Formula,
{
    type Output = UInt<<Add<L::Output, B1> as Formula>::Output, B0>;
}

impl<L> Formula for Add<UInt<L, B0>, B1>
where
    L: Formula,
    // Add<L, B1>: Formula,
{
    type Output = UInt<L, B1>;
}
#[cfg(test)]
mod test {
    use crate::{Eval, Formula, U1, U2, U3, U4};

    use super::Add;

    #[test]
    fn compile_basic_add() {
        const fn _eval_2<F: Formula<Output = U2>>() {}
        const _ADD: () = _eval_2::<Add<U1, U1>>();

        const fn _eval_3<F: Formula<Output = U3>>() {}
        const _ADD_3: () = _eval_3::<Add<U2, U1>>();

        const fn _eval_4<F: Formula<Output = U4>>() {}
        const _ADD_4: () = _eval_4::<Add<U3, U1, Eval>>();
    }
}
