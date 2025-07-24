use std::ops::{Add, AddAssign, Sub, SubAssign};

pub trait AdditiveArithmetic:
    Add<Output = Self> + AddAssign + Sub<Output = Self> + SubAssign + Sized
{
    const ZERO: Self;
}

macro_rules! additive_arithmetic_int_impl {
    ($($t:ty)*) => ($(
        impl AdditiveArithmetic for $t {
            const ZERO: Self = 0;
        }
    )*)
}

macro_rules! additive_arithmetic_float_impl {
    ($($t:ty)*) => ($(
        impl AdditiveArithmetic for $t {
            const ZERO: Self = 0.0;
        }
    )*)
}

additive_arithmetic_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
additive_arithmetic_float_impl! { f32 f64 }
