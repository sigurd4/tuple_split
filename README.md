[![Build Status (nightly)](https://github.com/sigurd4/tuple_split/workflows/Build-nightly/badge.svg)](https://github.com/sigurd4/tuple_split/actions/workflows/build-nightly.yml)
[![Build Status (nightly, all features)](https://github.com/sigurd4/tuple_split/workflows/Build-nightly-all-features/badge.svg)](https://github.com/sigurd4/tuple_split/actions/workflows/build-nightly-all-features.yml)

[![Build Status (stable)](https://github.com/sigurd4/tuple_split/workflows/Build-stable/badge.svg)](https://github.com/sigurd4/tuple_split/actions/workflows/build-stable.yml)
[![Build Status (stable, all features)](https://github.com/sigurd4/tuple_split/workflows/Build-stable-all-features/badge.svg)](https://github.com/sigurd4/tuple_split/actions/workflows/build-stable-all-features.yml)

[![Test Status](https://github.com/sigurd4/tuple_split/workflows/Test/badge.svg)](https://github.com/sigurd4/tuple_split/actions/workflows/test.yml)
[![Lint Status](https://github.com/sigurd4/tuple_split/workflows/Lint/badge.svg)](https://github.com/sigurd4/tuple_split/actions/workflows/lint.yml)

[![Latest Version](https://img.shields.io/crates/v/tuple_split.svg)](https://crates.io/crates/tuple_split)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://img.shields.io/docsrs/tuple_split)](https://docs.rs/tuple_split)
[![Coverage Status](https://img.shields.io/codecov/c/github/sigurd4/tuple_split)](https://app.codecov.io/github/sigurd4/tuple_split)

# tuple_split

This crate an extension for the [tupleops](https://crates.io/crates/tupleops) crate.

[tupleops](https://crates.io/crates/tupleops) contains many useful features for manipulating tuples and using tuples in generic code. However, it does not support any kind of splitting of tuples. This crate adds that feature.

## Examples

```rust
let t = (32, 0.707, "test");

// Splitting tuples by index
let (l, r) = tuple_split::split_tuple_at::<0, _>(t);
assert_eq!(t, tupleops::concat_tuples(l, r));

let (l, r) = tuple_split::split_tuple_at::<1, _>(t);
assert_eq!(t, tupleops::concat_tuples(l, r));

let (l, r) = tuple_split::split_tuple_at::<2, _>(t);
assert_eq!(t, tupleops::concat_tuples(l, r));

let (l, r) = tuple_split::split_tuple_at::<3, _>(t);
assert_eq!(t, tupleops::concat_tuples(l, r));

// Splitting tuples given a left side
let (l, r) = tuple_split::split_tuple_into_left::<(u8, f32), _>(t);
assert_eq!(t, tupleops::concat_tuples(l, r));

// Splitting tuples given a right side
let (l, r) = tuple_split::split_tuple_into_right::<(&str,), _>(t);
assert_eq!(t, tupleops::concat_tuples(l, r));

// Splitting tuples given both sides
let (l, r) = tuple_split::split_tuple_into::<(u8, f32), (&str)>(t);
assert_eq!(t, tupleops::concat_tuples(l, r));
```

## Split by index

Tuples can be split by a const-generic index. To use this feature, put `#![feature(generic_const_exprs)]` on the top of your `lib.rs` or `main.rs`.

### Example

```rust
#![feature(generic_const_exprs)]

let t = (1, 1.0, "test");

let (l, r) = tuple_split::split_tuple_at::<2, _>(t);

assert_eq!(t, tupleops::concat_tuples(l, r));
```

## Split by type

The type of tuple you want from the split operation can be used instead of an index. This does not require `#![feature(generic_const_exprs)]`. Either the left, right or both can be provided as a generic type.

### Examples

#### Left

```rust
let t = (1, 1.0, "test");

let (l, r) = tuple_split::split_tuple_into_left::<(u8, f32), _>(t);

assert_eq!(t, tupleops::concat_tuples(l, r));
```

#### Right

```rust
let t = (1, 1.0, "test");

let (l, r) = tuple_split::split_tuple_into_right::<(&str,), _>(t);

assert_eq!(t, tupleops::concat_tuples(l, r));
```

#### Both

```rust
let t = (1, 1.0, "test");

let (l, r) = tuple_split::split_tuple_into::<(u8, f32), (&str,)>(t);

assert_eq!(t, tupleops::concat_tuples(l, r));
```

## Tuple sizes

By default, this crate operates with tuples of up to 4 elements, just like the [tupleops](https://crates.io/crates/tupleops) crate. If you need to use bigger tuples, use the features `8`, `16`, `32`, `64`, `96`, `128`, `160`, `192`, `224` or `256` to set the maximum supported tuple size.

If you are using this crate in another crate, it's a nice gesture to provide this tuple size interface to the end user upstream as well. It can be done like this in the `Cargo.toml`:

```toml
[features]
default = []
dont_hurt_yourself_by_using_all_features = ["tuple_split/dont_hurt_yourself_by_using_all_features"]
8 = ["tuple_split/8"]
16 = ["8", "tuple_split/16"]
32 = ["16", "tuple_split/32"]
64 = ["32", "tuple_split/64"]
96 = ["64", "tuple_split/96"]
128 = ["96", "tuple_split/128"]
160 = ["128", "tuple_split/160"]
192 = ["160", "tuple_split/192"]
224 = ["192", "tuple_split/224"]
256 = ["224", "tuple_split/256"]
```

The `dont_hurt_yourself_by_using_all_features` is there to prevent usage of tuples bigger than 8 if `cargo` is ran with the flag `--all-features`. Using a tuple size above 16 is highly discouraged as it will make compilation time unbearably long. Compilation time will increase exponentially. You have been warned.