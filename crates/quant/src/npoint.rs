use std::{
    fmt::Write,
    iter::Sum,
    ops::{Add, Mul},
};

use crate::num_traits::{AbsDiff, NumConsts, Sqrt};

#[repr(transparent)]
pub struct NPoint<T, const N: usize>([T; N]);

impl<T, const N: usize> NPoint<T, N> {
    pub fn new(arr: [T; N]) -> Self {
        Self(arr)
    }

    pub fn zero() -> Self
    where
        T: NumConsts,
    {
        [T::ZERO; N].into()
    }

    pub fn one() -> Self
    where
        T: NumConsts,
    {
        [T::ONE; N].into()
    }

    pub fn distance<'a, U>(&'a self, other: &'a Self) -> U
    where
        &'a T: AbsDiff<Output = T>,
        T: Sum + Mul<Output = T> + Sqrt<Output = U>,
    {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| a.abs_diff(b) * a.abs_diff(b))
            .sum::<T>()
            .sqrt()
    }

    pub fn manhattan_distance<'a>(&'a self, other: &'a Self) -> T
    where
        &'a T: AbsDiff<Output = T>,
        T: Sum,
    {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| a.abs_diff(b))
            .sum()
    }
}

impl<'a, T, const N: usize> std::ops::Add<&'a NPoint<T, N>> for &'a NPoint<T, N>
where
    &'a T: Add<&'a T, Output = T>,
{
    type Output = NPoint<T, N>;

    fn add(self, rhs: &'a NPoint<T, N>) -> Self::Output {
        let v = self
            .0
            .iter()
            .zip(rhs.0.iter())
            .map(|(a, b)| a + b)
            .collect::<Vec<T>>();

        let out: [T; N] = v
            .try_into()
            .unwrap_or_else(|_| panic!("Could not construct fixed length array"));

        out.into()
    }
}

impl<T, U, const N: usize> From<U> for NPoint<T, N>
where
    U: Into<[T; N]>,
{
    fn from(value: U) -> Self {
        Self(value.into())
    }
}

impl<T, const N: usize> std::fmt::Display for NPoint<T, N>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('(')?;

        for (i, val) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, "{val}")?;
            } else {
                write!(f, ", {val}")?;
            }
        }

        f.write_char(')')
    }
}

impl<T, const N: usize> std::fmt::Debug for NPoint<T, N>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl<T, const N: usize> Clone for NPoint<T, N>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T, const N: usize> Copy for NPoint<T, N> where T: Copy {}

impl<T, const N: usize> PartialEq for NPoint<T, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T, const N: usize> Eq for NPoint<T, N> where T: Eq {}

#[cfg(test)]
mod test {
    use crate::npoint::NPoint;

    #[test]
    fn test_into() {
        let _: NPoint<usize, 4> = (1, 1, 1, 1).into();
    }

    #[test]
    fn test_distance() {
        assert_eq!(
            5.0,
            NPoint::<f64, 2>::zero().distance(&(3.0f64, 4.0f64).into())
        );
        assert_eq!(
            5.0,
            NPoint::<f64, 2>::zero().distance(&(-3.0f64, 4.0f64).into())
        );
    }

    #[test]
    fn test_add() {
        let a = NPoint::new([0, 1, 2, 3]);
        let b = NPoint::new([10, 20, 30, 40]);
        let target = NPoint::new([10i32, 21, 32, 43]);
        let out = &a + &b;

        assert_eq!(out, target);
    }
}
