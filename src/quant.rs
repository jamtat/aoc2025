use std::ops::{Div, Mul, Rem};

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Mul<Output = T> + Div<Output = T> + Eq + NumConsts + Rem<Output = T> + Copy,
{
    a * b / gcd(a, b)
}

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Rem<Output = T> + Eq + NumConsts + Copy,
{
    if b == T::ZERO { a } else { gcd(b, a % b) }
}

pub trait QuantIter: Iterator {
    fn lcm(mut self) -> Option<Self::Item>
    where
        Self::Item: Mul<Output = Self::Item>
            + Div<Output = Self::Item>
            + Eq
            + NumConsts
            + Rem<Output = Self::Item>
            + Copy,
        Self: Sized,
    {
        self.next().map(|first| self.fold(first, lcm))
    }

    fn gcd(mut self) -> Option<Self::Item>
    where
        Self::Item: Rem<Output = Self::Item> + Eq + NumConsts + Copy,
        Self: Sized,
    {
        self.next().map(|first| self.fold(first, gcd))
    }
}

impl<T: ?Sized> QuantIter for T where T: Iterator {}

pub trait NumConsts {
    const ZERO: Self;
    const ONE: Self;
}
macro_rules! impl_consts {
    ($typ:ty) => {
        impl NumConsts for $typ {
            const ZERO: $typ = 0;
            const ONE: $typ = 1;
        }
    };
    ($typ:ty, f) => {
        impl NumConsts for $typ {
            const ZERO: $typ = 0.0;
            const ONE: $typ = 1.0;
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
impl_consts!(f32, f);
impl_consts!(f64, f);

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

pub trait ILog10 {
    fn ilog10(self) -> u32;
}
macro_rules! impl_ilog10 {
    ($typ:ty) => {
        impl ILog10 for $typ {
            fn ilog10(self) -> u32 {
                self.ilog10()
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

pub fn digits<T, U>(x: T) -> u32
where
    T: Copy + UnsignedAbs<Output = U>,
    U: Copy + Eq + ILog10 + NumConsts,
{
    let x = x.unsigned_abs();
    if x == U::ZERO { 1 } else { x.ilog10() + 1 }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(digits(0), 1);
        assert_eq!(digits(1), 1);
        assert_eq!(digits(-1), 1);
        assert_eq!(digits(100), 3);
        assert_eq!(digits(-100), 3);
    }
}
