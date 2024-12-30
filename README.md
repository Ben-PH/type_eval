An R&D project that explores adding evaluation as a first-class-citizen to the typenum crate.

Typenum treats constant values and evaluation the way it ought to be treated:

- Unconcerned with machine representation (u64, usize, i32, etc)
- Specifications in the type-system (`fn foo<F: IsGreater<U8>()`)
- Compile-time and Execution-time code is explicitly separated
 
The shortfall of `typenum`, is that evaluation is not provided. The operation `((x + 4) * Y) / Z` must be expressed like so:

```rust
fn thing<X, Y>()
where
    X: Add<U4>,
    op!(X + U4): Mul<Y>,
    op!((X + U4) * Y): Div<Z>,
    op!(((X + U4) * Y) / Z): Unsigned,
{
    type ResHelper = op!(((X + U4) * Y) / Z);
    println!("the value evaluated to {}", ResHelper::USIZE)
    // Use `ResHelper` as the final type-system evaluation
}

fn main() {
    thing::<X, U4>();
}

```

This process of saying "the type-values passed in must be able to perform this arithmetic" is now done internally through the `*Expr` traits (Num/Bool/Ord/other expression traits):

```rust
fn thing<E: NumExpr>() {
    // TODO: extract an execution-time value from the type
    println!("the value evaluated to {}", E::Ret)
}

fn main() {
    // todo: actually impl the op! macro:
    // do_thing::<op!(((X + 4) * Y) / Z)>();
    // ...which would expand to:
    do_thing::<Div<Mul<Add<X, U4>, Y>, Z>>();
}
```


### Limitations

Currently negative numbers are not yet implemented.

The strategy for implementing division requires type-resolution capabilities made available in nightly-2024-10-17. This should be improved in the near future.

The following operations are implemented: `if`, `+`, `-`, `*`, `/`, `<<`, `>>`, `<`, `>`, `<=`, `>=`, `==`, `cmp`

The error output is in a messy binary form. The plan is to make this more readable.


### Future work

In _very_ approximate order of priority:

- Runtime number expression
- Robust property/fuzz testing
- negative numbers
- Move from binary to decimal representation
- More readable errors
- implementation of an `op` macro (replace `IF<LT<X, Y>, Add<Div<X, U2>, Y>, Z>` with `op!(if X < Y {(X + 2)/ Y} else {Z})`)
- Explore use of `struct U238...` instead of `type U238 = ...`
- Usable in place of a `usize` type in declaring array lengths.
- `match` expression impl
- closure expression impl
- any-type expression (i.e. Bool/Num/Ord expressions all impl the generic expression)
- Upstreaming into a nursary/nightly feature.
