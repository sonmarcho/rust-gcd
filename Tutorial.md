
# Verifying a real world Rust crate

In this tutorial,
we are going to use hax and F* to verify a small real world Rust crate.
The Rust crate https://crates.io/crates/gcd by Corey Farwell that we are going to verify implements
functions to compute the greatest common divisor of two integers.

## Preparation

TODO: Install hax etc.

To get started, we clone the repo of the Rust crate:
```
git clone git@github.com:frewsxcv/rust-gcd.git
```

We add hax-lib as a dependency, which will allow us to make annotations in the Rust code:
```
cargo add --git https://github.com/hacspec/hax hax-lib
```

## Extraction

Now we can attempt to translate the Rust code into F* code, which we will later verify. Our Rust crate implements two variants to compute the greatest common divisor, the euclidean algorithm and the binary algorithm, each in various variants for different integer types. To start simple, we will focus on
the `u8` variant of the euclidean algorithm first. The following command instructs hax to extract only the function `gcd::euclid_u8` and its dependencies.
```
cargo hax into -i '-** +gcd::euclid_u8' fstar
```
This creates a new file `proofs/fstar/extraction/Gcd.fst`, which contains a translation of our Rust crate in F*. To help F* find the correct dependencies, we download [this Makefile](https://gist.github.com/W95Psp/4c304132a1f85c5af4e4959dd6b356c3)
and put it into `proofs/fstar/extraction/`.

Before we instruct F* to start proving anything, we first check that all dependencies can be found:
```
OTHERFLAGS="--lax" make -C proofs/fstar/extraction/
```
This yields some harmless warnings and eventually:
```
All verification conditions discharged successfully
```
This means that all dependencies are available and we can start proving things.

The Makefile we are using helps us to cache the results of the F* verification, but this cache has the dangerous flaw that it does not invalidate when removing the `--lax` flag we used above. So we should delete the cache now:
```
rm -rf .fstar-cache
```

## Panic freedom

By default, without us specifying anything, hax's F* backend will attempt to prove that the Rust program terminates and does not panic:
```
make -C proofs/fstar/extraction/
```
The proof attempt fails with the following error:
```
* Error 19 at Gcd.fst(26,10-26,14):
  - Subtyping check failed
  - Expected type
      o:
      (Rust_primitives.Integers.u8 & Rust_primitives.Integers.u8)
        { (let _, _ = o in
            true) /\
          (let _, _ = o in
            Rust_primitives.Hax.Int.from_machine (Rust_primitives.Integers.mk_u32
                  0)
            <:
            Hax_lib.Int.t_Int) <
          (let _, _ = temp_0_ in
            Rust_primitives.Hax.Int.from_machine (Rust_primitives.Integers.mk_u32
                  0)
            <:
            Hax_lib.Int.t_Int) }
    got type Rust_primitives.Integers.u8 & Rust_primitives.Integers.u8
```
To prove that a while-loop terminates, F* requires a measure that decreases with every loop iteration. By default, the measure is simply the number 0, which always fails and results in errors resembling the one above. We need to find a better expression that decreases with every loop iteration. The relevant while-loop is the following:
```rust
while b != 0 {
    let temp = a;
    a = b;
    b = temp;

    b %= a;
}
```
In each iteration, the variables `a` and `b` get swapped and `b` is then set to `b % a`. If we focus only on what is happening to `b` here, we observe that `b` is set to `a % b` over the course of one iteration. Since `b` is always smaller than `a % b`, `b` is decreasing with every iteration, and we can set it as our termination measure:
```rust
while b != 0 {
    hax_lib::loop_decreases!(b);
    let temp = a;
    a = b;
    b = temp;

    b %= a;
}
```
We extract the F* code again and rerun F*:
```
cargo hax into -i '-** +gcd::euclid_u8' fstar
make -C proofs/fstar/extraction/
```
We get:
```
[CHECK] Gcd.fst 
Verified module: Gcd
All verification conditions discharged successfully
```
So the `gcd::euclid_u8` function terminates on all inputs and never panics!

We would like to verify the other variants of this function for different bit lengths as well, but using
```
cargo hax into -i '-** +gcd::euclid_u8 +gcd::euclid_u16 +gcd::euclid_u32 +gcd::euclid_u64 +gcd::euclid_u128 +gcd::euclid_usize' fstar
```
is a bit inconvenient. Instead, we can also mark the functions that we want to extract in the Rust code:
```rust
#[hax_lib::include]
pub const fn $euclid(a: $T, b: $T) -> $T
{
[...]
```
and then we can extract those functions using
```
cargo hax into -i '-**' fstar
```
You can open the the file `Gcd.fst` to make sure that all desired functions have indeed been extracted.
Now we can verify all variants, which should work without any further changes:
```
make -C proofs/fstar/extraction/
```

## Binary GCD

Next, we will attempt to prove panic freedom also for the binary variants. We add an `include`-annotation to the `$binary` function:
```rust
#[hax_lib::include]
pub const fn $binary(mut u: $T, mut v: $T) -> $T
{
[...]
```

We attempt to extract this function as well:
```
cargo hax into -i '-**' fstar
```

Unfortunately, it's not that easy. Hax can only translate a fragment of Rust. If something cannot be translated, we need to work around that. In our case, we get lots of errors of this kind:
```
error: [HAX0001] something is not implemented yet.This is discussed in issue https://github.com/hacspec/hax/issues/933.
Please upvote or comment this issue if you see this error message.
Unhandled loop kind

This is discussed in issue https://github.com/hacspec/hax/issues/933.
Please upvote or comment this issue if you see this error message.
Note: the error was labeled with context `FunctionalizeLoops`.

  --> src/lib.rs:45:13
   |
45 | /             loop {
46 | |                 v >>= v.trailing_zeros();
...  |
58 | |                 if v == 0 { break; }
59 | |             }
   | |_____________^
   |
```
This is because the `loop`-construct cannot be translated. As a first (temporary) fix, we replace `loop` in `src/lib.rs` by `while true`.

We run extraction again:
```
cargo hax into -i '-**' fstar
```
Now it succeeds. We verify:
```
make -C proofs/fstar/extraction/
```
This yields a couple of harmless warnings and one error:
```
Error 72 at Gcd.fst(29,38-29,61):
  - Identifier impl_u8__trailing_zeros not found in module Core_models.Num
```
This is happening because the function `trailing_zeros` is missing in hax's F* library. We can add it locally to our project by creating a file named `Core_models.Num.fsti` in `proofs/fstar/extraction`, and inserting the following code:
```
module Core_models.Num
open Rust_primitives
 
val trailing_zeros: #t:inttype -> int_t t -> (n:u32{v n >= 0 /\ v n <= bits t})

unfold let impl_u8__trailing_zeros (n:u8) = trailing_zeros n
unfold let impl_u16__trailing_zeros (n:u16) = trailing_zeros n
unfold let impl_u32__trailing_zeros (n:u32) = trailing_zeros n
unfold let impl_u64__trailing_zeros (n:u64) = trailing_zeros n
unfold let impl_u128__trailing_zeros (n:u128) = trailing_zeros n
unfold let impl_usize__trailing_zeros (n:usize) = trailing_zeros n
```
This code tells F* about the `trailing_zeros` functions and their signature for all unsigned integer types.

Running verification again, we get the next error:
```
* Error 72 at Gcd.fst(19,28-19,41):
  - Identifier while_loop_cf not found in module Rust_primitives.Hax
```