use std::ops::{AddAssign, Div, DivAssign, Mul, MulAssign, Rem};

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

pub fn num_digits<T, U>(x: T) -> u32
where
    T: Copy + UnsignedAbs<Output = U>,
    U: Copy + Eq + ILog10 + NumConsts,
{
    let x = UnsignedAbs::unsigned_abs(x);
    if x == U::ZERO {
        1
    } else {
        ILog10::ilog10(x) + 1
    }
}

#[cfg(test)]
mod test_digits {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(num_digits(0), 1);
        assert_eq!(num_digits(1), 1);
        assert_eq!(num_digits(-1), 1);
        assert_eq!(num_digits(100), 3);
        assert_eq!(num_digits(-100), 3);
    }
}

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

pub struct DigitIter<T> {
    r: u32,
    l: u32,
    num_digits: u32,
    n: T,
}

impl<U> DigitIter<U> {
    pub fn new<T>(n: T) -> Self
    where
        T: Copy + UnsignedAbs<Output = U>,
        U: Copy + Eq + ILog10 + NumConsts,
    {
        let num_digits = num_digits(n);
        Self {
            r: 0,
            num_digits,
            l: num_digits,
            n: n.unsigned_abs(),
        }
    }

    fn digit_at(&self, i: u32) -> Option<U>
    where
        U: Copy + NumConsts + RemEuclid + Pow + Div<Output = U>,
    {
        if i < self.num_digits {
            Some((self.n / U::TEN.pow(i)).rem_euclid(U::TEN))
        } else {
            None
        }
    }
}

impl<T> Iterator for DigitIter<T>
where
    T: Copy + NumConsts + RemEuclid + Pow + Div<Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.r >= self.l {
            return None;
        }

        self.r += 1;
        self.digit_at(self.r - 1)
    }
}

impl<T> DoubleEndedIterator for DigitIter<T>
where
    T: Copy + NumConsts + RemEuclid + Pow + Div<Output = T>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.l <= self.r {
            return None;
        }

        self.l -= 1;
        self.digit_at(self.l)
    }
}

#[cfg(test)]
mod test_digit_iter {
    use super::*;

    #[test]
    fn test_digit_iter() {
        assert_eq!(0u8.iter_digits().collect::<Vec<_>>(), vec![0u8]);
        assert_eq!(102u32.iter_digits().collect::<Vec<_>>(), vec![2u32, 0, 1]);
        assert_eq!(
            102u32.iter_digits().rev().collect::<Vec<_>>(),
            vec![1u32, 0, 2]
        );
    }
}

pub fn reverse_digits<T>(n: T) -> T
where
    T: Copy + NumConsts + Rem<Output = T> + DivAssign + MulAssign + AddAssign + Eq + PartialEq,
{
    let base = T::TEN;
    let mut out = T::ZERO;

    let mut n = n;

    while n != T::ZERO {
        out *= base;
        out += n % base;
        n /= base;
    }

    out
}

#[cfg(test)]
mod test_reverse_digits {
    use super::*;
    #[test]
    fn test_reverse_digits() {
        assert_eq!(reverse_digits(0usize), 0);
        assert_eq!(reverse_digits(10usize), 1);
        assert_eq!(reverse_digits(1234560usize), 654321);
    }

    #[test]
    fn test_reverse_digits_neg() {
        assert_eq!(-1234560isize.reverse_digits(), -654321);
    }
}

pub trait Digits {
    fn iter_digits<U>(self) -> DigitIter<U>
    where
        Self: Copy + UnsignedAbs<Output = U>,
        U: Copy + Eq + ILog10 + NumConsts,
    {
        DigitIter::new(self)
    }

    fn reverse_digits(self) -> Self
    where
        Self: Copy
            + NumConsts
            + Rem<Output = Self>
            + DivAssign
            + MulAssign
            + AddAssign
            + Eq
            + PartialEq,
    {
        reverse_digits(self)
    }

    fn num_digits<U>(self) -> u32
    where
        Self: Copy + UnsignedAbs<Output = U>,
        U: Copy + Eq + ILog10 + NumConsts,
    {
        num_digits(self)
    }
}

impl<T> Digits for T {}
