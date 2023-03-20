#![no_std]
#![feature(trait_alias)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(const_precise_live_drops)]

//! This crate is an extension for the [tupleops](tupleops) crate,
//! which adds a trait for splitting tuples by index.
//! 
//! Tuples which may be split at index MIDDLE have the trait [TupleSplit](crate::TupleSplit)<MIDDLE>,
//! which, when split, returns ([TupleSplit::Left](TupleSplit::Left), [TupleSplit::Right](TupleSplit::Right)).
//! 
//! Type alias [Left](Left) and [Right](Right) equal [TupleSplit::Left](TupleSplit::Left) and [TupleSplit::Right](TupleSplit::Right) respectively.
//! fpr any tuple which implements [TupleSplit](crate::TupleSplit) at the given MIDDLE.
//! 
//! The trait alias [SplitLeftInto](SplitLeftInto) is implemented for any tuple which may be split where [TupleSplit::Left](TupleSplit::Left) = L.
//! 
//! The trait alias [SplitRightInto](SplitRightInto) is implemented for any tuple which may be split where [TupleSplit::Right](TupleSplit::Right) = R.
//! 
//! The trait alias [SplitInto](SplitInto) is implemented for any tuple which may be split where [TupleSplit::Left](TupleSplit::Left) = L and [TupleSplit::Right](TupleSplit::Right) = R.
//! 
//! ```rust
//! use tupleops::concat_tuples;
//! use tuple_split::*;
//! 
//! let t: (u8, f32, &str) = (1, 1.0, "test");
//! 
//! let (l, r): ((u8, f32), (&str,)) = TupleSplit::<2>::split_tuple(t);
//! 
//! assert_eq!(t, concat_tuples(l, r));
//! 
//! let (l, r): ((u8, f32), (&str,)) = split_tuple::<2, _>(t);
//! 
//! assert_eq!(t, concat_tuples(l, r));
//! ```

use blk_count_macro::count;
use tupleops::{TupleLength, Tuple};

/// Type alias [Left](Left) equals [TupleSplit::Left](TupleSplit::Left)
/// for any tuple which implements [TupleSplit](crate::TupleSplit) at the given MIDDLE.
pub type Left<T, const MIDDLE: usize> = <T as TupleSplit<MIDDLE>>::Left;
/// Type alias [Right](Right) equals [TupleSplit::Right](TupleSplit::Right)
/// for any tuple which implements [TupleSplit](crate::TupleSplit) at the given MIDDLE.
pub type Right<T, const MIDDLE: usize> = <T as TupleSplit<MIDDLE>>::Right;
/// The trait alias [SplitLeftInto](SplitLeftInto) is implemented for any tuple which may be split where [TupleSplit::Left](TupleSplit::Left) = L.
pub trait SplitLeftInto<L> = TupleSplit<{<L as TupleLength>::LENGTH}, Left = L>
where L: TupleLength, [(); <L as TupleLength>::LENGTH]:;
/// The trait alias [SplitRightInto](SplitRightInto) is implemented for any tuple which may be split where [TupleSplit::Right](TupleSplit::Right) = R.
pub trait SplitRightInto<R> = TupleSplit<{<Self as TupleLength>::LENGTH - <R as TupleLength>::LENGTH}, Right = R> + TupleLength
where R: TupleLength, [(); <Self as TupleLength>::LENGTH - <R as TupleLength>::LENGTH]:;
/// The trait alias [SplitInto](SplitInto) is implemented for any tuple which may be split where [TupleSplit::Left](TupleSplit::Left) = L and [TupleSplit::Right](TupleSplit::Right) = R.
pub trait SplitInto<L, R> = TupleSplit<{<L as TupleLength>::LENGTH}, Left = L, Right = R>
where L: TupleLength, [(); <L as TupleLength>::LENGTH]:;

/// Tuples which may be split at index MIDDLE have the trait [TupleSplit](crate::TupleSplit)<MIDDLE>,
/// which, when split, returns ([TupleSplit::Left](TupleSplit::Left), [TupleSplit::Right](TupleSplit::Right)).
/// 
/// ```rust
/// use tupleops::concat_tuples;
/// use tuple_split::*;
/// 
/// let t: (u8, f32, &str) = (1, 1.0, "test");
/// let (l, r): ((u8, f32), (&str,)) = TupleSplit::<2>::split_tuple(t);
/// 
/// assert_eq!(t, concat_tuples(l, r));
/// ```
#[const_trait]
pub trait TupleSplit<const MIDDLE: usize>: Tuple
{
    type Left;
    type Right;

    fn split_tuple(self) -> (Self::Left, Self::Right);
}

/// Splits tuple at a given index.
/// 
/// Index is specified as const generic MIDDLE.
/// 
/// Tuple must be of trait [TupleSplit](crate::TupleSplit)<MIDDLE>.
/// 
/// Returns ([TupleSplit::Left](TupleSplit::Left), [TupleSplit::Right](TupleSplit::Right)) for the given Tuple and MIDDLE.
/// 
/// ```rust
/// use tupleops::concat_tuples;
/// use tuple_split::*;
/// 
/// let t: (u8, f32, &str) = (1, 1.0, "test");
/// let (l, r): ((u8, f32), (&str,)) = split_tuple::<2, _>(t);
/// 
/// assert_eq!(t, concat_tuples(l, r));
/// ```
pub const fn split_tuple<const MIDDLE: usize, T>(tuple: T) -> (T::Left, T::Right)
where
    T: ~const TupleSplit<MIDDLE>
{
    tuple.split_tuple()
}

macro_rules! impl_split_single {
    (( $($types1:ident),* ), ( $($types2:ident),* )) => {
        impl<$($types1,)* $($types2,)*> const TupleSplit<{count!($($types1),*)}> for ($($types1,)* $($types2,)*)
        {
            type Left = ($($types1,)*);
            type Right = ($($types2,)*);

            fn split_tuple(self) -> (Self::Left, Self::Right)
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

impl_split_all!{
    (_1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16)
}