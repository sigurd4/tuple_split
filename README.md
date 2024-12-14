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

Tuples which may be split at index MIDDLE have the trait `TupleSplit`,
which, when split, returns `TupleSplit::Left`, `TupleSplit::Right`.

They can lso be split by specifying either of the sides or both.

```rust
let t: (u8, f32, &str) = (32, 0.707, "test");

// Splitting tuples by index
let (l, r): ((), (u8, f32, &str)) = tuple_split::split_tuple_at::<0, _>(t);
assert_eq!(t, tupleops::concat_tuples(l, r));

let (l, r): ((u8,), (f32, &str)) = tuple_split::split_tuple_at::<1, _>(t);
assert_eq!(t, tupleops::concat_tuples(l, r));

let (l, r): ((u8, f32), (&str,)) = tuple_split::split_tuple_at::<2, _>(t);
assert_eq!(t, tupleops::concat_tuples(l, r));

let (l, r): ((u8, f32, &str), ()) = tuple_split::split_tuple_at::<3, _>(t);
assert_eq!(t, tupleops::concat_tuples(l, r));

// Splitting tuples given a left side
let (l, r): ((u8, f32), (&str,)) = tuple_split::split_tuple_into_left::<(u8, f32)>(t);
assert_eq!(t, tupleops::concat_tuples(l, r));

// Splitting tuples given a right side
let (l, r): ((u8, f32), (&str,)) = tuple_split::split_tuple_into_right::<(&str,)>(t);
assert_eq!(t, tupleops::concat_tuples(l, r));

// Splitting tuples given both sides
let (l, r): ((u8, f32), (&str,)) = tuple_split::split_tuple_into::<(u8, f32), (&str)>(t);
assert_eq!(t, tupleops::concat_tuples(l, r));
```