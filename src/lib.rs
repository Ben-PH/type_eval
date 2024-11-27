
mod formula_traits;
use formula_traits::*;

mod val_types;
use val_types::*;

mod evals;
include!(concat!(env!("OUT_DIR"), "/consts.rs"));


mod op_types;
mod complex_evals;

pub struct Ast;
pub struct Eval;
impl Mode for Ast {}
impl Mode for Eval {}

