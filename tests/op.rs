use type_eval::op;
use type_eval::{num_vals::*, MemRep, NumExpr, NumRet};

type My4 = op!(U2 + U2);

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

#[test]
fn name() {
    assert_eq!(NumRet::<My4>::MU32, 4);
}
#[allow(non_upper_case_globals)]
#[allow(clippy::used_underscore_items)]
#[test]
fn eval_add_op() {
    const _0_ADD_0: () = _b0::<op!(U0 + U0)>();
    const _1_ADD_0: () = _b1::<op!(U1 + U0)>();
    const _1_ADD_1: () = _b2::<op!(U1 + U1)>();
    const _2_ADD_1: () = _b3::<op!(U2 + U1)>();
    const _1_ADD_2: () = _b3::<op!(U1 + U2)>();
    const _3_ADD_1: () = _b4::<op!(U3 + U1)>();
    const _4_ADD_1: () = _b5::<op!(U4 + U1)>();
    const _1_ADD_3: () = _b4::<op!(U1 + U3)>();
    const _1_ADD_4: () = _b5::<op!(U1 + U4)>();
    const _2_ADD_2: () = _b4::<op!(U2 + U2)>();
    const _3_ADD_3: () = _b6::<op!(U3 + U3)>();
    const _6_ADD_1: () = _b7::<op!(U6 + U1)>();
    const _7_ADD_1: () = _b8::<op!(U7 + U1)>();

    const _1_ADD_1__ADD_1: () = _b3::<op!(U1 + (U1 + U1))>();
    const _1_ADD__1_ADD_1: () = _b3::<op!((U1 + U1) + U1)>();

    const _3_ADD_3__ADD_3: () = _b9::<op!(U3 + (U3 + U3))>();
    const _3_ADD__3_ADD_3: () = _b9::<op!((U3 + U3) + U3)>();
}
