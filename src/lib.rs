mod formula_traits;
pub use formula_traits::*;

mod val_types;
pub use val_types::*;

mod evals;
pub use evals::add::*;
include!(concat!(env!("OUT_DIR"), "/consts.rs"));
pub use consts::*;

mod op_types;
pub use op_types::*;

pub struct Ast;
pub struct Eval;
impl Mode for Ast {}
impl Mode for Eval {}
