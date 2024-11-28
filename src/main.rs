use tn_direct::{consts::*, B1, Eval, Formula};

// use tn_direct::F;
fn main() {
}
const _2: () = proof::<tn_direct::Add<B1, B1, Eval>>();
const fn proof<F : Formula<Output = U2>>() {}

// const _0 : () = proof::<Not<Not<Or<False, And<True, And<True, True>>>>>>();
