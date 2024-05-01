use std::{
  fmt::Debug,
  ops::{Add, Sub},
};

pub trait Value:
  Add<Output = Self> + Sub<Output = Self> + Copy + Debug + Default + Eq + Ord
{
}

macro_rules! add_impl {
  ($($t:ty)*) => {$(
      impl Value for $t {}
    )*
  };
}

add_impl!(usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128);
