#![no_std]
#![feature(trait_alias)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(const_precise_live_drops)]
#![feature(tuple_trait)]
#![recursion_limit = "512"]

//!
//! This crate an extension for the [tupleops](tupleops) crate.
//!
//! [tupleops](tupleops) contains many useful features for manipulating tuples andusing tuples in generic code.
//! However, it does not support any kind of splitting of tuples. This crate adds that feature.
//!
//! # Examples
//!
//! ```rust
//! #![feature(generic_const_exprs)]
//!
//! let t = (32, 0.707, "test");
//!
//! // Splitting tuples by index
//! let (l, r) = tuple_split::split_tuple_at::<0, _>(t);
//! assert_eq!(t, tupleops::concat_tuples(l, r));
//!
//! let (l, r) = tuple_split::split_tuple_at::<1, _>(t);
//! assert_eq!(t, tupleops::concat_tuples(l, r));
//!
//! let (l, r) = tuple_split::split_tuple_at::<2, _>(t);
//! assert_eq!(t, tupleops::concat_tuples(l, r));
//!
//! let (l, r) = tuple_split::split_tuple_at::<3, _>(t);
//! assert_eq!(t, tupleops::concat_tuples(l, r));
//!
//! // Splitting tuples given a left side
//! let (l, r) = tuple_split::split_tuple_into_left::<(u8, f32), _>(t);
//! assert_eq!(t, tupleops::concat_tuples(l, r));
//!
//! // Splitting tuples given a right side
//! let (l, r) = tuple_split::split_tuple_into_right::<(&str,), _>(t);
//! assert_eq!(t, tupleops::concat_tuples(l, r));
//!
//! // Splitting tuples given both sides
//! let (l, r) = tuple_split::split_tuple_into::<(u8, f32), (&str,)>(t);
//! assert_eq!(t, tupleops::concat_tuples(l, r));
//! ```
//!
//! # Split by index
//!
//! Tuples can be split by a const-generic index. To use this feature, put `#![feature(generic_const_exprs)]` on the top of your `lib.rs` or `main.rs`.
//!
//! ## Example
//!
//! ```rust
//! #![feature(generic_const_exprs)]
//!
//! let t = (1, 1.0, "test");
//!
//! let (l, r) = tuple_split::split_tuple_at::<2, _>(t);
//!
//! assert_eq!(t, tupleops::concat_tuples(l, r));
//! ```
//!
//! # Split by type
//!
//! The type of tuple you want from the split operation can be used instead of an index. This does not require `#![feature(generic_const_exprs)]`. Either the left, right
//! or both can be provided as a generic type.
//!
//! ## Examples
//!
//! ### Left
//!
//! ```rust
//! let t = (1, 1.0, "test");
//!
//! let (l, r) = tuple_split::split_tuple_into_left::<(u8, f32), _>(t);
//!
//! assert_eq!(t, tupleops::concat_tuples(l, r));
//! ```
//!
//! ### Right
//!
//! ```rust
//! let t = (1, 1.0, "test");
//!
//! let (l, r) = tuple_split::split_tuple_into_right::<(&str,), _>(t);
//!
//! assert_eq!(t, tupleops::concat_tuples(l, r));
//! ```
//!
//! ### Both
//!
//! ```rust
//! let t = (1, 1.0, "test");
//!
//! let (l, r) = tuple_split::split_tuple_into::<(u8, f32), (&str,)>(t);
//!
//! assert_eq!(t, tupleops::concat_tuples(l, r));
//! ```
//!
//! ## Tuple sizes
//!
//! By default, this crate operates with tuples of up to 16 elements, just like the [tupleops](https://crates.io/crates/tupleops) crate.
//! If you want to use differently sized tuples, use the features `8`, `16`, `32`, `64`, `96`, `128`, `160`, `192`, `224` or `256` to set the maximum supported tuple
//! size.
//!
//! The `dont_hurt_yourself_by_using_all_features` is there to prevent usage of tuples bigger than 8 if `cargo` is ran with the flag `--all-features`.
//! Using a tuple size above 16 is highly discouraged as it will make compilation time unbearably long. Compilation time will increase exponentially.
//! You have been warned.

use core::marker::Tuple;

use blk_count_macro::count;
use tupleops::{ConcatTuples, TupleConcat};

/// Type alias [Left](Left) equals [TupleSplit::Left](TupleSplit::Left)
/// for any tuple which implements [TupleSplit](crate::TupleSplit) at the given MIDDLE.
pub type Left<T, const MIDDLE: usize> = <T as TupleSplitAt<MIDDLE>>::Left;
/// Type alias [Right](Right) equals [TupleSplit::Right](TupleSplit::Right)
/// for any tuple which implements [TupleSplit](crate::TupleSplit) at the given MIDDLE.
pub type Right<T, const MIDDLE: usize> = <T as TupleSplitAt<MIDDLE>>::Right;

/// Tuples which may be split at index `MIDDLE` have the trait [TupleSplitAt](crate::TupleSplitAt),
/// which, when split, returns [TupleSplitAt::Left](TupleSplitAt::Left), [TupleSplitAt::Right](TupleSplitAt::Right).
///
/// # Example
///
/// ```rust
/// #![feature(generic_const_exprs)]
///
/// let t = (1, 1.0, "test");
///
/// let (l, r) = tuple_split::split_tuple_at::<2, _>(t);
///
/// assert_eq!(t, tupleops::concat_tuples(l, r));
/// ```
#[diagnostic::on_unimplemented(message = "`{Self}` cannot be split at index `{MIDDLE}`")]
#[const_trait]
pub trait TupleSplitAt<const MIDDLE: usize>: Tuple
{
    type Left: Tuple;
    type Right: Tuple;

    fn split_tuple_at(self) -> (Self::Left, Self::Right);
}

/// A trait for splitting a tuple up into two parts given a specified left part `L` and right part `R`. `L` and `R` must be the left and right part of `Self`.
///
/// Tuples will be split into parts `L` and `R`.
///
/// # Example
///
/// ```rust
/// let t = (1, 1.0, "test");
///
/// let (l, r) = tuple_split::split_tuple_into::<(u8, f32), (&str,)>(t);
///
/// assert_eq!(t, tupleops::concat_tuples(l, r));
/// ```
#[diagnostic::on_unimplemented(message = "`{Self}` cannot be split up into `{L}` and `{R}`")]
#[const_trait]
pub trait TupleSplitInto<L, R>: ~const TupleSplitIntoLeft<L, Right = R> + ~const TupleSplitIntoRight<R, Left = L>
where
    L: Tuple,
    R: Tuple
{
    fn split_tuple_into(self) -> (L, R);
}

impl<T, L, R> const TupleSplitInto<L, R> for T
where
    Self: ~const TupleSplitIntoLeft<L, Right = R> + ~const TupleSplitIntoRight<R, Left = L>,
    L: Tuple,
    R: Tuple
{
    fn split_tuple_into(self) -> (L, R)
    {
        self.split_tuple_into_left()
    }
}

/// A trait for splitting a tuple up into two parts given a specified left part `L`. `L` must be a leftmost segment of `Self`.
///
/// Tuples will be split into parts `L` and [TupleSplitIntoLeft::Right](TupleSplitIntoLeft::Right).
///
/// # Example
///
/// ```rust
/// let t = (1, 1.0, "test");
///
/// let (l, r) = tuple_split::split_tuple_into_left::<(u8, f32), _>(t);
///
/// assert_eq!(t, tupleops::concat_tuples(l, r));
/// ```
#[diagnostic::on_unimplemented(message = "`{L}` is not the left part of `{Self}`")]
#[const_trait]
pub trait TupleSplitIntoLeft<L>: Tuple
where
    L: Tuple
{
    type Right: Tuple;

    fn split_tuple_into_left(self) -> (L, Self::Right);
}

/// A trait for splitting a tuple up into two parts given a specified right part `R`. `R` must be a rightmost segment of `Self`.
///
/// Tuples will be split into parts [TupleSplitIntoRight::Left](TupleSplitIntoRight::Left) and `R`.
///
/// # Example
///
/// ```rust
/// let t = (1, 1.0, "test");
///
/// let (l, r) = tuple_split::split_tuple_into_right::<(&str,), _>(t);
///
/// assert_eq!(t, tupleops::concat_tuples(l, r));
/// ```
#[diagnostic::on_unimplemented(message = "`{R}` is not the right part of `{Self}`")]
#[const_trait]
pub trait TupleSplitIntoRight<R>: Tuple
where
    R: Tuple
{
    type Left: Tuple;

    fn split_tuple_into_right(self) -> (Self::Left, R);
}

/// Splits tuple at a given index.
///
/// Index is specified as const generic `MIDDLE.
///
/// Tuple must be of trait `[TupleSplitAt](crate::TupleSplitAt)<MIDDLE>`.
///
/// Returns `([TupleSplitAt::Left](TupleSplitAt::Left), [TupleSplitAt::Right](TupleSplitAt::Right))` for the given Tuple and `MIDDLE`.
///
/// ```rust
/// #![feature(generic_const_exprs)]
///
/// let t = (1, 1.0, "test");
///
/// let (l, r) = tuple_split::split_tuple_at::<2, _>(t);
///
/// assert_eq!(t, tupleops::concat_tuples(l, r));
/// ```
pub const fn split_tuple_at<const MIDDLE: usize, T>(tuple: T) -> (T::Left, T::Right)
where
    T: ~const TupleSplitAt<MIDDLE>
{
    tuple.split_tuple_at()
}

/// A trait for splitting a tuple up into two parts given a specified left part `L` and right part `R`. `L` and `R` must be the left and right part of `Self`.
///
/// Tuples will be split into parts `L` and `R`.
///
/// # Example
///
/// ```rust
/// let t = (1, 1.0, "test");
///
/// let (l, r) = tuple_split::split_tuple_into::<(u8, f32), (&str,)>(t);
///
/// assert_eq!(t, tupleops::concat_tuples(l, r));
/// ```
pub const fn split_tuple_into<L, R>(tuple: ConcatTuples<L, R>) -> (L, R)
where
    L: Tuple,
    R: Tuple,
    (L, R): TupleConcat<L, R>,
    ConcatTuples<L, R>: ~const TupleSplitInto<L, R>
{
    tuple.split_tuple_into()
}

/// Splits a tuple up into two parts given a specified left part `L`. `L` must be a leftmost segment of `Self`.
///
/// Tuples will be split into parts `L` and [TupleSplitIntoLeft::Right](TupleSplitIntoLeft::Right).
///
/// # Example
///
/// ```rust
/// let t = (1, 1.0, "test");
///
/// let (l, r) = tuple_split::split_tuple_into_left::<(u8, f32), _>(t);
///
/// assert_eq!(t, tupleops::concat_tuples(l, r));
/// ```
pub const fn split_tuple_into_left<L, T>(tuple: T) -> (L, T::Right)
where
    L: Tuple,
    T: ~const TupleSplitIntoLeft<L>
{
    tuple.split_tuple_into_left()
}

/// Splits a tuple up into two parts given a specified right part `R`. `R` must be a rightmost segment of `Self`.
///
/// Tuples will be split into parts [TupleSplitIntoRight::Left](TupleSplitIntoRight::Left) and `R`.
///
/// # Example
///
/// ```rust
/// let t = (1, 1.0, "test");
///
/// let (l, r) = tuple_split::split_tuple_into_right::<(&str,), _>(t);
///
/// assert_eq!(t, tupleops::concat_tuples(l, r));
/// ```
pub const fn split_tuple_into_right<R, T>(tuple: T) -> (T::Left, R)
where
    R: Tuple,
    T: ~const TupleSplitIntoRight<R>
{
    tuple.split_tuple_into_right()
}

macro_rules! impl_split_single {
    (( $($types1:ident),* ), ( $($types2:ident),* )) => {
        impl<$($types1,)* $($types2,)*> const TupleSplitAt<{count!($($types1),*)}> for ($($types1,)* $($types2,)*)
        {
            type Left = ($($types1,)*);
            type Right = ($($types2,)*);

            fn split_tuple_at(self) -> (Self::Left, Self::Right)
            {
                let ($($types1,)* $($types2,)*) = self;
                (($($types1,)*), ($($types2,)*))
            }
        }

        impl<$($types1,)* $($types2,)*> const TupleSplitIntoLeft<($($types1,)*)> for ($($types1,)* $($types2,)*)
        {
            type Right = ($($types2,)*);

            fn split_tuple_into_left(self) -> (($($types1,)*), ($($types2,)*))
            {
                let ($($types1,)* $($types2,)*) = self;
                (($($types1,)*), ($($types2,)*))
            }
        }
        impl<$($types1,)* $($types2,)*> const TupleSplitIntoRight<($($types2,)*)> for ($($types1,)* $($types2,)*)
        {
            type Left = ($($types1,)*);

            fn split_tuple_into_right(self) -> (($($types1,)*), ($($types2,)*))
            {
                let ($($types1,)* $($types2,)*) = self;
                (($($types1,)*), ($($types2,)*))
            }
        }
    };
}
macro_rules! impl_split_combinations {
    ( (), ( $($types2:ident),* ) ) => {
        impl_split_single!{(), ($($types2),*)}
    };
    (($t0:ident $(,$types1:ident)* ), ( $($types2:ident),* )) => {
        impl_split_single!{($t0 $(,$types1)*), ($($types2),*)}

        impl_split_combinations!{($($types1),*), ($t0 $(,$types2)*)}
    };
    (($($types:ident),*)) => {
        impl_split_combinations!{($($types),*), ()}
    }
}
macro_rules! impl_split_all {
    (()) => {
        impl_split_combinations!{()}
    };
    (($t0:ident $(,$types:ident)*)) => {
        impl_split_combinations!{($t0 $(,$types)*)}

        impl_split_all!{($($types),*)}
    }
}

#[cfg(not(feature = "8"))]
impl_split_all! {
    (
        _1, _2, _3, _4
    )
}

#[cfg(feature = "8")]
#[cfg(any(feature = "dont_hurt_yourself_by_using_all_features", not(feature = "16")))]
impl_split_all! {
    (
        _1, _2, _3, _4, _5, _6, _7, _8
    )
}

#[cfg(feature = "16")]
#[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
#[cfg(not(feature = "32"))]
impl_split_all! {
    (
        _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16
    )
}

#[cfg(feature = "32")]
#[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
#[cfg(not(feature = "64"))]
impl_split_all! {
    (
        _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16,
        _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32
    )
}

#[cfg(feature = "64")]
#[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
#[cfg(not(feature = "96"))]
impl_split_all! {
    (
        _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16,
        _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32,
        _33, _34, _35, _36, _37, _38, _39, _40, _41, _42, _43, _44, _45, _46, _47, _48,
        _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59, _60, _61, _62, _63, _64
    )
}

#[cfg(feature = "96")]
#[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
#[cfg(not(feature = "128"))]
impl_split_all! {
    (
        _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16,
        _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32,
        _33, _34, _35, _36, _37, _38, _39, _40, _41, _42, _43, _44, _45, _46, _47, _48,
        _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59, _60, _61, _62, _63, _64,
        _65, _66, _67, _68, _69, _70, _71, _72, _73, _74, _75, _76, _77, _78, _79, _80,
        _81, _82, _83, _84, _85, _86, _87, _88, _89, _90, _91, _92, _93, _94, _95, _96
    )
}

#[cfg(feature = "128")]
#[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
#[cfg(not(feature = "160"))]
impl_split_all! {
    (
        _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16,
        _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32,
        _33, _34, _35, _36, _37, _38, _39, _40, _41, _42, _43, _44, _45, _46, _47, _48,
        _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59, _60, _61, _62, _63, _64,
        _65, _66, _67, _68, _69, _70, _71, _72, _73, _74, _75, _76, _77, _78, _79, _80,
        _81, _82, _83, _84, _85, _86, _87, _88, _89, _90, _91, _92, _93, _94, _95, _96,
        _97, _98, _99, _100, _101, _102, _103, _104, _105, _106, _107, _108, _109, _110, _111, _112,
        _113, _114, _115, _116, _117, _118, _119, _120, _121, _122, _123, _124, _125, _126, _127, _128
    )
}

#[cfg(feature = "160")]
#[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
#[cfg(not(feature = "192"))]
impl_split_all! {
    (
        _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16,
        _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32,
        _33, _34, _35, _36, _37, _38, _39, _40, _41, _42, _43, _44, _45, _46, _47, _48,
        _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59, _60, _61, _62, _63, _64,
        _65, _66, _67, _68, _69, _70, _71, _72, _73, _74, _75, _76, _77, _78, _79, _80,
        _81, _82, _83, _84, _85, _86, _87, _88, _89, _90, _91, _92, _93, _94, _95, _96,
        _97, _98, _99, _100, _101, _102, _103, _104, _105, _106, _107, _108, _109, _110, _111, _112,
        _113, _114, _115, _116, _117, _118, _119, _120, _121, _122, _123, _124, _125, _126, _127, _128,
        _129, _130, _131, _132, _133, _134, _135, _136, _137, _138, _139, _140, _141, _142, _143, _144,
        _145, _146, _147, _148, _149, _150, _151, _152, _153, _154, _155, _156, _157, _158, _159, _160
    )
}

#[cfg(feature = "192")]
#[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
#[cfg(not(feature = "224"))]
impl_split_all! {
    (
        _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16,
        _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32,
        _33, _34, _35, _36, _37, _38, _39, _40, _41, _42, _43, _44, _45, _46, _47, _48,
        _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59, _60, _61, _62, _63, _64,
        _65, _66, _67, _68, _69, _70, _71, _72, _73, _74, _75, _76, _77, _78, _79, _80,
        _81, _82, _83, _84, _85, _86, _87, _88, _89, _90, _91, _92, _93, _94, _95, _96,
        _97, _98, _99, _100, _101, _102, _103, _104, _105, _106, _107, _108, _109, _110, _111, _112,
        _113, _114, _115, _116, _117, _118, _119, _120, _121, _122, _123, _124, _125, _126, _127, _128,
        _129, _130, _131, _132, _133, _134, _135, _136, _137, _138, _139, _140, _141, _142, _143, _144,
        _145, _146, _147, _148, _149, _150, _151, _152, _153, _154, _155, _156, _157, _158, _159, _160,
        _161, _162, _163, _164, _165, _166, _167, _168, _169, _170, _171, _172, _173, _174, _175, _176,
        _177, _178, _179, _180, _181, _182, _183, _184, _185, _186, _187, _188, _189, _190, _191, _192
    )
}

#[cfg(feature = "224")]
#[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
#[cfg(not(feature = "256"))]
impl_split_all! {
    (
        _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16,
        _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32,
        _33, _34, _35, _36, _37, _38, _39, _40, _41, _42, _43, _44, _45, _46, _47, _48,
        _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59, _60, _61, _62, _63, _64,
        _65, _66, _67, _68, _69, _70, _71, _72, _73, _74, _75, _76, _77, _78, _79, _80,
        _81, _82, _83, _84, _85, _86, _87, _88, _89, _90, _91, _92, _93, _94, _95, _96,
        _97, _98, _99, _100, _101, _102, _103, _104, _105, _106, _107, _108, _109, _110, _111, _112,
        _113, _114, _115, _116, _117, _118, _119, _120, _121, _122, _123, _124, _125, _126, _127, _128,
        _129, _130, _131, _132, _133, _134, _135, _136, _137, _138, _139, _140, _141, _142, _143, _144,
        _145, _146, _147, _148, _149, _150, _151, _152, _153, _154, _155, _156, _157, _158, _159, _160,
        _161, _162, _163, _164, _165, _166, _167, _168, _169, _170, _171, _172, _173, _174, _175, _176,
        _177, _178, _179, _180, _181, _182, _183, _184, _185, _186, _187, _188, _189, _190, _191, _192,
        _193, _194, _195, _196, _197, _198, _199, _200, _201, _202, _203, _204, _205, _206, _207, _208,
        _209, _210, _211, _212, _213, _214, _215, _216, _217, _218, _219, _220, _221, _222, _223, _224
    )
}

#[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
#[cfg(feature = "256")]
impl_split_all! {
    (
        _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16,
        _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32,
        _33, _34, _35, _36, _37, _38, _39, _40, _41, _42, _43, _44, _45, _46, _47, _48,
        _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59, _60, _61, _62, _63, _64,
        _65, _66, _67, _68, _69, _70, _71, _72, _73, _74, _75, _76, _77, _78, _79, _80,
        _81, _82, _83, _84, _85, _86, _87, _88, _89, _90, _91, _92, _93, _94, _95, _96,
        _97, _98, _99, _100, _101, _102, _103, _104, _105, _106, _107, _108, _109, _110, _111, _112,
        _113, _114, _115, _116, _117, _118, _119, _120, _121, _122, _123, _124, _125, _126, _127, _128,
        _129, _130, _131, _132, _133, _134, _135, _136, _137, _138, _139, _140, _141, _142, _143, _144,
        _145, _146, _147, _148, _149, _150, _151, _152, _153, _154, _155, _156, _157, _158, _159, _160,
        _161, _162, _163, _164, _165, _166, _167, _168, _169, _170, _171, _172, _173, _174, _175, _176,
        _177, _178, _179, _180, _181, _182, _183, _184, _185, _186, _187, _188, _189, _190, _191, _192,
        _193, _194, _195, _196, _197, _198, _199, _200, _201, _202, _203, _204, _205, _206, _207, _208,
        _209, _210, _211, _212, _213, _214, _215, _216, _217, _218, _219, _220, _221, _222, _223, _224,
        _225, _226, _227, _228, _229, _230, _231, _232, _233, _234, _235, _236, _237, _238, _239, _240,
        _241, _242, _243, _244, _245, _246, _247, _248, _249, _250, _251, _252, _253, _254, _255, _256
    )
}

#[cfg(test)]
mod tests
{
    use crate as tuple_split;

    #[test]
    fn test_split_concat()
    {
        let t: (u8, f32, &str) = (1, 1.0, "test");

        let (l, r) = tuple_split::split_tuple_at::<0, _>(t);
        assert_eq!(t, tupleops::concat_tuples(l, r));

        let (l, r) = tuple_split::split_tuple_at::<1, _>(t);
        assert_eq!(t, tupleops::concat_tuples(l, r));

        let (l, r) = tuple_split::split_tuple_at::<2, _>(t);
        assert_eq!(t, tupleops::concat_tuples(l, r));

        let (l, r) = tuple_split::split_tuple_at::<3, _>(t);
        assert_eq!(t, tupleops::concat_tuples(l, r));
    }

    #[cfg(feature = "8")]
    #[test]
    fn test()
    {
        let t = (1u8, 2u16, 3u32, 4u64, 5u128);

        let (l1, r1) = tuple_split::split_tuple_into_left::<(u8, u16), _>(t);
        let (l2, r2) = tuple_split::split_tuple_into_right::<(u32, u64, u128), _>(t);
        let (l3, r3) = tuple_split::split_tuple_at::<2, _>(t);

        assert_eq!(l1, l2);
        assert_eq!(l2, l3);

        assert_eq!(r1, r2);
        assert_eq!(r2, r3);

        assert_eq!(t, tupleops::concat_tuples(l1, r1));
        assert_eq!(t, tupleops::concat_tuples(l2, r2));
        assert_eq!(t, tupleops::concat_tuples(l3, r3));
    }
}

/*mod private
{
    use core::{marker::Tuple, mem::ManuallyDrop};

    use tupleops::{ConcatTuples, TupleConcat};

    union TupleConcatTransmutation<L, R>
    where
        L: Tuple,
        R: Tuple,
        (L, R): TupleConcat<L, R, Type: Tuple>
    {
        split: ManuallyDrop<(ManuallyDrop<L>, ManuallyDrop<R>)>,
        concat: ManuallyDrop<ConcatTuples<L, R>>
    }

    /// TODO: find a safe way to do this.
    /// According to language specifications, the compiler can re-order elements in a tuple in any way in memory.
    /// This function assumes they are in-order.
    /// It works, i guess? (so far)
    /// It's only because the compiler happens to do it like i assumed (most of the time, i think...)
    #[deprecated(note = "This is undefined behaviour")]
    pub const fn tuple_split_const_hold<L, R>(tuple: ConcatTuples<L, R>) -> (ManuallyDrop<L>, ManuallyDrop<R>)
    where
        L: Tuple,
        R: Tuple,
        (L, R): TupleConcat<L, R, Type: Tuple>
    {
        assert!(
            core::mem::size_of::<(ManuallyDrop<L>, ManuallyDrop<R>)>() == core::mem::size_of::<ConcatTuples<L, R>>(),
            "You just ran into undefined behaviour. Please report it on https://github.com/sigurd4/fn_zip/issues."
        );
        unsafe {
            ManuallyDrop::into_inner(
                TupleConcatTransmutation {
                    concat: ManuallyDrop::new(tuple)
                }
                .split
            )
        }
    }

    /*#[deprecated(note = "This is undefined behaviour")]
    pub const fn tuple_split_const<L, R>(tuple: ConcatTuples<L, R>) -> (L, R)
    where
        L: Tuple,
        R: Tuple,
        (L, R): TupleConcat<L, R, Type: Tuple>
    {
        let (left, right) = tuple_split_const_hold(tuple);
        (ManuallyDrop::into_inner(left), ManuallyDrop::into_inner(right))
    }*/
}*/
