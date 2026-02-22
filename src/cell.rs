use std::{
    hash::Hash,
    ops::{BitOrAssign, ShlAssign},
};

use num_traits::{PrimInt, Signed, WrappingAdd, WrappingSub};
pub trait Cell:
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
    + From<u8>
    + From<u32>
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
        + BitOrAssign
        + From<u8>
        + From<u32>,
> Cell for T
{
}
