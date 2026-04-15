use std::ops::{Div, Mul, Rem};

use crate::num_traits::NumConsts;

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
