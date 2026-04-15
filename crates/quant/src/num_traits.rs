pub trait NumConsts {
    const ZERO: Self;
    const ONE: Self;
    const TEN: Self;
    const MIN: Self;
    const MAX: Self;
    const BITS: u32;
}
macro_rules! impl_consts {
    ($typ:ty) => {
        impl NumConsts for $typ {
            const ZERO: $typ = 0;
            const ONE: $typ = 1;
            const TEN: $typ = 10;
            const MIN: $typ = <$typ>::MIN;
            const MAX: $typ = <$typ>::MAX;
            const BITS: u32 = <$typ>::BITS;
        }
    };
    ($typ:ty, $bits:expr) => {
        impl NumConsts for $typ {
            const ZERO: $typ = 0.0;
            const ONE: $typ = 1.0;
            const TEN: $typ = 10.0;
            const MIN: $typ = <$typ>::MIN;
            const MAX: $typ = <$typ>::MAX;
            const BITS: u32 = $bits;
        }
    };
}
impl_consts!(u8);
impl_consts!(u16);
impl_consts!(u32);
impl_consts!(u64);
impl_consts!(u128);
impl_consts!(usize);
impl_consts!(i8);
impl_consts!(i16);
impl_consts!(i32);
impl_consts!(i64);
impl_consts!(i128);
impl_consts!(isize);
impl_consts!(f32, 32);
impl_consts!(f64, 64);

pub trait RemEuclid {
    fn rem_euclid(self, other: Self) -> Self;
}
macro_rules! impl_rem_euclid {
    ($typ:ty) => {
        impl RemEuclid for $typ {
            fn rem_euclid(self, other: Self) -> Self {
                <$typ>::rem_euclid(self, other)
            }
        }
    };
}

impl_rem_euclid!(f32);
impl_rem_euclid!(f64);
impl_rem_euclid!(u8);
impl_rem_euclid!(u16);
impl_rem_euclid!(u32);
impl_rem_euclid!(u64);
impl_rem_euclid!(u128);
impl_rem_euclid!(usize);
impl_rem_euclid!(i8);
impl_rem_euclid!(i16);
impl_rem_euclid!(i32);
impl_rem_euclid!(i64);
impl_rem_euclid!(i128);
impl_rem_euclid!(isize);

pub trait DivEuclid {
    fn div_euclid(self, other: Self) -> Self;
}
macro_rules! impl_div_euclid {
    ($typ:ty) => {
        impl DivEuclid for $typ {
            fn div_euclid(self, other: Self) -> Self {
                <$typ>::div_euclid(self, other)
            }
        }
    };
}

impl_div_euclid!(f32);
impl_div_euclid!(f64);
impl_div_euclid!(u8);
impl_div_euclid!(u16);
impl_div_euclid!(u32);
impl_div_euclid!(u64);
impl_div_euclid!(u128);
impl_div_euclid!(usize);
impl_div_euclid!(i8);
impl_div_euclid!(i16);
impl_div_euclid!(i32);
impl_div_euclid!(i64);
impl_div_euclid!(i128);
impl_div_euclid!(isize);

pub trait ILog10 {
    fn ilog10(self) -> u32;
}
macro_rules! impl_ilog10 {
    ($typ:ty) => {
        impl ILog10 for $typ {
            fn ilog10(self) -> u32 {
                <$typ>::ilog10(self)
            }
        }
    };
}

impl_ilog10!(u8);
impl_ilog10!(u16);
impl_ilog10!(u32);
impl_ilog10!(u64);
impl_ilog10!(u128);
impl_ilog10!(usize);
impl_ilog10!(i8);
impl_ilog10!(i16);
impl_ilog10!(i32);
impl_ilog10!(i64);
impl_ilog10!(i128);
impl_ilog10!(isize);

pub trait UnsignedAbs {
    type Output;
    fn unsigned_abs(self) -> Self::Output;
}
macro_rules! impl_uabs_u {
    ($typ:ty) => {
        impl UnsignedAbs for $typ {
            type Output = $typ;
            fn unsigned_abs(self) -> Self::Output {
                self
            }
        }
    };
}
impl_uabs_u!(u8);
impl_uabs_u!(u16);
impl_uabs_u!(u32);
impl_uabs_u!(u64);
impl_uabs_u!(u128);
impl_uabs_u!(usize);

macro_rules! impl_uabs_i {
    ($typ:ty, $typ2:ty) => {
        impl UnsignedAbs for $typ {
            type Output = $typ2;
            fn unsigned_abs(self) -> Self::Output {
                self.unsigned_abs()
            }
        }
    };
}
impl_uabs_i!(i8, u8);
impl_uabs_i!(i16, u16);
impl_uabs_i!(i32, u32);
impl_uabs_i!(i64, u64);
impl_uabs_i!(i128, u128);
impl_uabs_i!(isize, usize);

pub trait Pow {
    fn pow(self, exp: u32) -> Self;
}

macro_rules! impl_pow {
    ($typ:ty) => {
        impl Pow for $typ {
            fn pow(self, exp: u32) -> Self {
                <$typ>::pow(self, exp)
            }
        }
    };
}

impl_pow!(u8);
impl_pow!(u16);
impl_pow!(u32);
impl_pow!(u64);
impl_pow!(u128);
impl_pow!(usize);
impl_pow!(i8);
impl_pow!(i16);
impl_pow!(i32);
impl_pow!(i64);
impl_pow!(i128);
impl_pow!(isize);

pub trait Sqrt {
    type Output;
    fn sqrt(self) -> Self::Output;
}

macro_rules! impl_sqrtf {
    ($typ:ty) => {
        impl Sqrt for $typ {
            type Output = $typ;
            fn sqrt(self) -> Self::Output {
                <$typ>::sqrt(self)
            }
        }
    };
}

impl_sqrtf!(f32);
impl_sqrtf!(f64);

macro_rules! impl_sqrt {
    ($typ:ty, $out:ty) => {
        impl Sqrt for $typ {
            type Output = $out;
            fn sqrt(self) -> Self::Output {
                (self as $out).sqrt()
            }
        }
    };
}

impl_sqrt!(u8, f32);
impl_sqrt!(u16, f32);
impl_sqrt!(u32, f64);
impl_sqrt!(u64, f64);
impl_sqrt!(i8, f32);
impl_sqrt!(i16, f32);
impl_sqrt!(i32, f64);
impl_sqrt!(i64, f64);

pub trait AbsDiff {
    type Output;
    fn abs_diff(self, other: Self) -> Self::Output;
}

macro_rules! impl_abs_diff {
    ($typ:ty) => {
        impl AbsDiff for $typ {
            type Output = $typ;
            fn abs_diff(self, other: Self) -> Self::Output {
                if other > self {
                    other - self
                } else {
                    self - other
                }
            }
        }

        impl AbsDiff for &$typ {
            type Output = $typ;
            fn abs_diff(self, other: Self) -> Self::Output {
                if other > self {
                    other - self
                } else {
                    self - other
                }
            }
        }
    };
}

impl_abs_diff!(f32);
impl_abs_diff!(f64);
impl_abs_diff!(u8);
impl_abs_diff!(u16);
impl_abs_diff!(u32);
impl_abs_diff!(u64);
impl_abs_diff!(u128);
impl_abs_diff!(usize);
impl_abs_diff!(i8);
impl_abs_diff!(i16);
impl_abs_diff!(i32);
impl_abs_diff!(i64);
impl_abs_diff!(i128);
impl_abs_diff!(isize);
