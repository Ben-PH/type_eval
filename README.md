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
    AddOut<X, U4>: Mul<Y>,
    MulOut<AddOut<X, U4>, Mul<Y>>: Div<Z>,
{
    type ResHelper = DivOut<MulOut<AddOut<X, U4>, Mul<Y>>, Div<Z>>;
    // Use `ResHelper` as the final type-system evaluation
}

```

This process of saying "the type-values passed in must be able to perform this arithmetic" is now done internally through the `*Expr` traits (Num/Bool/<TODO> expression traits):

```rust
fn thing<E: NumExpr>() {
    // Use `E::Ret` as the final type-system evaluation
}

fn main() {
    // todo: actually impl the op! macro:
    // do_thing::<op!(((X + 4) * Y) / Z)>();
    // ...which would expand to:
    do_thing::<Div<Mul<Add<X, U4>, Y>, Z>>();
}
```


### Limitations

Currently division and negative numbers are not yet implemented.

The following operations are implemented: `+`, `-`, `*`, `<<`, `>>`, `<`, `>`, `<=`, `>=`, `==`

The error output is in a messy binary form. The plan is to make this more readable.


### Future work

- division
- negative numbers
- Move from binary to decimal representation
- More readable errors
- Explore use of `struct U238...` instead of `type U238 = ...`
- Ergonomic control flow
- Usable in place of a `usize` type in declaring array lengths.
- Upstreaming into a nursary/nightly feature.
