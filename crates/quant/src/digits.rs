use std::ops::{AddAssign, Div, DivAssign, MulAssign, Rem};

use crate::num_traits::{ILog10, NumConsts, Pow, RemEuclid, UnsignedAbs};

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

    fn remaining(&self) -> u32 {
        self.l - self.r
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining() as usize;
        (remaining, Some(remaining))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.remaining() as usize
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
