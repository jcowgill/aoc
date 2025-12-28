use nalgebra::{Dim, Matrix, Scalar, Storage, U1};
use num::{Signed, Zero};
use std::cmp::Ordering;

pub trait VectorExt<T> {
    /// Calculates the taxicab (L1) norm of this vector
    fn taxicab_norm(&self) -> T;

    /// Calculates the squared euclidean (L2 ^2) norm of this vector
    fn l2_squared_norm(&self) -> T;
}

impl<T: Scalar + Signed + Zero, R: Dim, S: Storage<T, R, U1>> VectorExt<T> for Matrix<T, R, U1, S> {
    fn taxicab_norm(&self) -> T {
        self.fold(Zero::zero(), |acc, comp| acc + comp.abs())
    }

    fn l2_squared_norm(&self) -> T {
        self.fold(Zero::zero(), |acc, comp| acc + comp.clone() * comp)
    }
}

/// Compares two matrixes elementwise to create a total ordering between them
pub fn total_matrix_cmp<T, R, C, S>(a: &Matrix<T, R, C, S>, b: &Matrix<T, R, C, S>) -> Ordering
where
    T: Scalar + Ord,
    R: Dim,
    C: Dim,
    S: Storage<T, R, C>,
{
    for (left, right) in a.iter().zip(b.iter()) {
        let order = left.cmp(right);
        if order.is_ne() {
            return order;
        }
    }

    Ordering::Equal
}
