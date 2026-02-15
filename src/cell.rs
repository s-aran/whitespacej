use std::{
    hash::Hash,
    ops::{BitOrAssign, ShlAssign},
};

use num_traits::{PrimInt, Signed, WrappingAdd, WrappingSub};
pub(crate) trait Cell:
    PrimInt
    + Signed
    + Copy
    + PartialEq
    + PartialOrd
    + Ord
    + Eq
    + WrappingAdd
    + WrappingSub
    + Hash
    + ShlAssign
    + BitOrAssign
{
}

impl<
    T: PrimInt
        + Signed
        + Copy
        + PartialEq
        + PartialOrd
        + Ord
        + Eq
        + WrappingAdd
        + WrappingSub
        + Hash
        + ShlAssign
        + BitOrAssign,
> Cell for T
{
}
