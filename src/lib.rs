use val_types::Number;

// mod formula_traits;
// pub use formula_traits::*;
//
mod val_types;
//
// mod evals;
// pub use evals::add::*;
// include!(concat!(env!("OUT_DIR"), "/consts.rs"));
// pub use consts::*;
//
mod op_types;
//

mod expr;
pub trait Mode {}
pub struct Ast;
pub struct Eval;
impl Mode for Ast {}
impl Mode for Eval {}
pub trait Expr {
    type Output: Number;
}

mod remys_gift;
