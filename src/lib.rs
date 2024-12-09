#![no_std]

use core::marker::PhantomData;

struct UnfinishedAst;
struct EvaluatedAst;
trait AstStage {}
impl AstStage for UnfinishedAst {}
impl AstStage for EvaluatedAst {}

trait Value {}
trait AstEvaluation {
    type Output: Value;
}

struct B0;
struct B1;
struct UInt<LB, TB, S: AstStage> {
    _lead_bits: PhantomData<LB>,
    _term_bit: PhantomData<TB>,
    _ast_stage: PhantomData<S>,
}

trait Untrimmed {
    type Trimmed: Value;
}

impl Value for B0 {}
impl Value for B1 {}
impl Value for UInt<B1, B0, EvaluatedAst> {}
impl Value for UInt<B1, B1, EvaluatedAst> {}
impl<R> Untrimmed for UInt<B0, R, UnfinishedAst>
where
    R: Value,
{
    type Trimmed = R;
}
impl AstEvaluation for B0 {
    type Output = B0;
}
impl AstEvaluation for B1 {
    type Output = B1;
}

impl<L, R> AstEvaluation for UInt<L, R, EvaluatedAst>
where
    L: AstEvaluation,
    UInt<<L as AstEvaluation>::Output, R, UnfinishedAst>: Value,
{
    type Output = UInt<<L as AstEvaluation>::Output, R, UnfinishedAst>;
}

impl<L, R> AstEvaluation for UInt<L, R, EvaluatedAst>
where
    UInt<L, R, UnfinishedAst>: Untrimmed,
{
    type Output = <UInt<L, R, UnfinishedAst> as Untrimmed>::Trimmed;
}

#[cfg(test)]
mod test {
    use super::*;
    const fn evaled_b0<E: AstEvaluation<Output = B0>>() {}
    const fn evaled_b1<E: AstEvaluation<Output = B1>>() {}
    const fn evaled_b2<E: AstEvaluation<Output = UInt<B1, B0, EvaluatedAst>>>() {}
    const fn evaled_b3<E: AstEvaluation<Output = UInt<B1, B1, EvaluatedAst>>>() {}
    #[test]
    fn name() {
        const _B0: () = evaled_b0::<B0>();
        const _B1: () = evaled_b1::<B1>();
        const _B0_B0: () = evaled_b0::<UInt<B0, B0, EvaluatedAst>>();
        const _B0_B1: () = evaled_b1::<UInt<B0, B1, EvaluatedAst>>();
        const _B1_B0: () = evaled_b2::<UInt<B1, B0, EvaluatedAst>>();
        const _B1_B1: () = evaled_b3::<UInt<B1, B1, EvaluatedAst>>();
        const _B0_B0_B0: () = evaled_b0::<UInt<UInt<B0, B0, EvaluatedAst>, B0, EvaluatedAst>>();
    }
}
