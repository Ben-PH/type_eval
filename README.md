An evolution of the typenum crate, the goal is to bake evaluation into the types:

instead of 

```rust
fn add<L: Unsigned, R: Unsigned>() -> u64
where
    <L as Add<R>>::Output: Unsigned
{
    <<L as Add<R>>::Output>::U64
}
```
...it would be something more along the lines of:
```rust
fn add<L: Unsigned, R: Unsigned>() -> u64
{
    Add<L, R, Eval>::U64
}
```

