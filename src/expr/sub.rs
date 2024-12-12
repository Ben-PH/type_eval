#![cfg_attr(rustfmt, rustfmt::skip)]

use std::marker::PhantomData;

use crate::{
    op_types::Sub,
    val_types::{BitLit, BitStrLit, BitString, Number, _0, _1},
    Eval, Expr,
};

type BS<Bs,B> = BitString<Bs,B>;

impl<L: Expr, R: Expr> Expr for Sub<L, R>
where
    Sub<L::Output, R::Output, Eval>: Expr,
{
    type Output = <Sub<L::Output, R::Output, Eval> as Expr>::Output;
}

// ----
//  Most basic of addition evaluations
// ----
impl       Expr for Sub<                              _0, _0, Eval>                                                    { type Output = _0;                }                                            //            0 - 0 =                   0
impl       Expr for Sub<                              _1, _0, Eval>                                                    { type Output = _1;                }                                            //            1 - 0 =                   1
impl       Expr for Sub<                              _1, _1, Eval>                                                    { type Output = _0;                }                                            //            1 - 1 =                   0
impl<Bs, B>Expr for Sub<          BitString<    Bs ,  B>, _0, Eval>   where Bs : BitStrLit, B  : BitLit                { type Output = BitString<Bs, B>;  }                                            //    [..Bs, B] - 0 =           [..Bs, B]
                                                                                                                
impl<Bs>   Expr for Sub<          BitString<    Bs , _1>, _1, Eval>   where Bs : BitStrLit                             { type Output = BitString<Bs, _0>; }                                            //    [..Bs, 1] - 1 =           [..Bs, 0]
                                                                                                                
impl       Expr for Sub<          BitString<    _1 , _0>, _1, Eval>                                                    { type Output = _1;                }                                            //        0b_10 - 1 =                   1
impl<Bs>   Expr for Sub<BitString<BitString<Bs, _1>, _0>, _1, Eval>   where Bs : BitStrLit                             { type Output = BitString<BitString<Bs, _0>, _1>; }                             // [..Bs, 1, 0] - 1 =        [..Bs, 0, 1]
                                                                                                                
impl<Bs>   Expr for Sub<BitString<BitString<Bs, _0>, _0>, _1, Eval>   where Bs : BitStrLit, Sub<BitString<Bs, _0>, _1> : Expr, 
                                                                            <Sub<BitString<Bs, _0>, _1> 
                                                                              as Expr>::Output : BitStrLit                                                     
                                                                                                                       { type Output = BitString<<Sub<BitString<Bs, _0>, _1> as Expr>::Output, _1>; }  // [..Bs, 0, 0] - 1 = [..([..Bs,0]-1), 1]

impl<LBs : BitStrLit, LB : BitLit, RBs : BitStrLit, RB : BitLit>
           Expr for Sub<BitString<LBs, LB>, BitString<RBs, RB>, Eval> where SubAcc<Push<BitString<LBs, LB>, BitString<RBs, RB>>, Empty> : ApplySub 
                                                                                                                      { type Output = <SubAcc<Push<BitString<LBs, LB>, BitString<RBs, RB>>, Empty> as ApplySub>::Output;}


pub trait Stack{}
pub struct Top<T : BitLit, Rest : Stack>{ _top : PhantomData<T>, _rest : PhantomData<Rest> } 
pub struct Empty;
impl Stack for Empty{}
impl<T : BitLit, Rest : Stack> Stack for Top<T, Rest>{}

trait SubState{}
pub struct Push<LBs : BitStrLit, RBs : BitStrLit>{  
  _l_bits : PhantomData<LBs>,
  _r_bits : PhantomData<RBs>,
}
pub struct Pop<Bs : BitStrLit>{_bits : PhantomData<Bs>}
impl<LBs : BitStrLit, RBs : BitStrLit> SubState for Push<LBs,RBs>{}
impl<Bs : BitStrLit> SubState for Pop<Bs>{}
pub struct SubAcc<A : SubState,S : Stack>{
  _stack  : PhantomData<S>,
  _action : PhantomData<A>,  
}
pub trait ApplySub {
  type Output : Expr + Number;
}

impl<                                  S : Stack> ApplySub for SubAcc<Push<               _1,                _1>,            S>                                                       { type Output = _0; }
impl<LBs : BitStrLit, RBs : BitStrLit, S : Stack> ApplySub for SubAcc<Push<BitString<LBs,_0>, BitString<RBs,_0>>,            S> where SubAcc<Push<LBs, RBs>, Top<_0,S>>    : ApplySub { type Output = <SubAcc<Push<LBs, RBs>,   Top<_0,S>> as ApplySub>::Output; }
impl<LBs : BitStrLit, RBs : BitStrLit, S : Stack> ApplySub for SubAcc<Push<BitString<LBs,_1>, BitString<RBs,_1>>,            S> where SubAcc<Push<LBs, RBs>, Top<_0,S>>    : ApplySub { type Output = <SubAcc<Push<LBs, RBs>,   Top<_0,S>> as ApplySub>::Output; }
                                                                                                                             
impl<LBs : BitStrLit, RBs : BitStrLit, S : Stack> ApplySub for SubAcc<Push<BitString<LBs,_0>, BitString<RBs,_1>>,            S> where Sub<LBs, _1> : Expr, 
                                                                                                                                      <Sub<LBs, _1> 
                                                                                                                                        as Expr>::Output : BitStrLit,
                                                                                                                                      SubAcc<Push<<Sub<LBs, _1> 
                                                                                                                                        as Expr>::Output, RBs>, Top<_1,S>> : ApplySub { type Output = <SubAcc<Push<<Sub<LBs, _1> as Expr>::Output, RBs>, Top<_1,S>> as ApplySub>::Output; }
impl<LBs : BitStrLit, RBs : BitStrLit, S : Stack> ApplySub for SubAcc<Push<BitString<LBs,_1>, BitString<RBs,_0>>,            S> where SubAcc<Push<LBs, RBs>, Top<_1,S>> : ApplySub, 
                                                                                                                                      <SubAcc<Push<LBs, RBs>, Top<_1,S>> 
                                                                                                                                        as ApplySub>::Output : Expr + Number          { type Output = <SubAcc<Push<LBs, RBs>, Top<_1,S>> as ApplySub>::Output; }  
                                                                                                                             
impl<Bs : BitStrLit,                   S : Stack> ApplySub for SubAcc<Push<BitString<Bs,_1>,                 _1>,            S> where SubAcc<Pop<BitString<Bs, _0>>,    S> : ApplySub { type Output = <SubAcc<Pop<BitString<Bs, _0>>,    S> as ApplySub>::Output; }
impl<Bs : BitStrLit, B : BitLit,    Rest : Stack> ApplySub for SubAcc<Pop<Bs>,                                    Top<B, Rest>> where SubAcc<Pop<BitString<Bs, _0>>, Rest> : ApplySub, 
                                                                                                                                      <SubAcc<Pop<BitString<Bs, _0>>, Rest> 
                                                                                                                                        as ApplySub>::Output : Expr + Number          { type Output = <SubAcc<Pop<BitString<Bs, B>> , Rest> as ApplySub>::Output; }
impl<Bs : BitStrLit,                            > ApplySub for SubAcc<Pop<Bs>,                                          Empty>  where Bs : Number + Expr                              { type Output = Bs; }



#[cfg(test)]
mod test {
    use super::*;
    const fn _b0<E: Expr<Output = _0>>() {}
    const fn _b1<E: Expr<Output = _1>>() {}
    const fn _b2<E: Expr<Output = BitString<_1, _0>>>() {}
    const fn _b3<E: Expr<Output = BitString<_1, _1>>>() {}
    const fn _b4<E: Expr<Output = BitString<BitString<_1, _0>, _0>>>() {}
    const fn _b5<E: Expr<Output = BitString<BitString<_1, _0>, _1>>>() {}
    const fn _b6<E: Expr<Output = BitString<BitString<_1, _1>, _0>>>() {}
    #[test]
    fn eval_add() {
        const _0_SUB_0: () = _b0::<Sub<_0, _0>>();
        const _1_SUB_0: () = _b1::<Sub<_1, _0>>();
        const _1_SUB_1: () = _b0::<Sub<_1, _1>>();
        const _2_SUB_1: () = _b1::<Sub<BitString<_1, _0>, _1>>();
        // const _1_SUB_2: () = _b3::<Sub<_1, BitString<_1, _0>>>();
        const _3_SUB_1: () = _b2::<Sub<BitString<_1, _1>, _1>>();
        // const _1_SUB_3: () = _b4::<Sub<_1, BitString<_1, _1>>>();
        const _2_SUB_2: () = _b0::<Sub<BitString<_1, _0>, BitString<_1, _0>>>();
        const _3_SUB_2: () = _b1::<Sub<BitString<_1, _1>, BitString<_1, _0>>>();
        const _3_SUB_3: () = _b0::<Sub<BitString<_1, _1>, BitString<_1, _1>>>();
    }
}
