///! Very simple vector implementation for AOC
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

/// 2D Vector
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

/// 3D Vector
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// Macro implementing vector operators (no RHS)
macro_rules! impl_vec_op {
    ($vec:ident { $($field:ident),* }, $op:ident, $fn:ident) => (
        impl<T: $op> $op for $vec<T> {
            type Output = $vec<<T as $op>::Output>;
            fn $fn(self) -> Self::Output {
                $vec { $($field: $op::$fn(self.$field),)* }
            }
        }
    )
}

/// Macro implementing vector, vector operators
macro_rules! impl_vec_vec_op {
    ($vec:ident { $($field:ident),* }, $op:ident, $fn:ident) => (
        impl<T: $op> $op for $vec<T> {
            type Output = $vec<<T as $op>::Output>;
            fn $fn(self, rhs: Self) -> Self::Output {
                $vec { $($field: $op::$fn(self.$field, rhs.$field),)* }
            }
        }
    )
}

/// Macro implementing vector, vector assignment operators
macro_rules! impl_vec_vec_assign {
    ($vec:ident { $($field:ident),* }, $op:ident, $fn:ident) => (
        impl<T: $op> $op for $vec<T> {
            fn $fn(&mut self, rhs: Self) {
                $($op::$fn(&mut self.$field, rhs.$field);)*
            }
        }
    )
}

/// Macro implementing vector, scalar operators
macro_rules! impl_vec_scalar_op {
    ($vec:ident { $($field:ident),* }, $op:ident, $fn:ident) => (
        impl<RHS: Copy, T: $op<RHS>> $op<RHS> for $vec<T> {
            type Output = $vec<<T as $op<RHS>>::Output>;
            fn $fn(self, rhs: RHS) -> Self::Output {
                $vec { $($field: $op::$fn(self.$field, rhs),)* }
            }
        }
    )
}

/// Macro implementing vector, scalar assignment operators
macro_rules! impl_vec_scalar_assign {
    ($vec:ident { $($field:ident),* }, $op:ident, $fn:ident) => (
        impl<RHS: Copy, T: $op<RHS>> $op<RHS> for $vec<T> {
            fn $fn(&mut self, rhs: RHS) {
                $($op::$fn(&mut self.$field, rhs);)*
            }
        }
    )
}

/// Macro implementing special functions only on primitive integers
macro_rules! impl_vec_num {
    ($vec:ident { $($field:ident),* }, $num:ty) => (
        #[allow(dead_code)]
        impl $vec<$num> {
            /// Calculates the taxicab (L1) norm of this vector
            pub fn taxicab_norm(self) -> $num {
                let mut result: $num = 0;
                $(result += self.$field.abs();)*
                result
            }
        }
    )
}

/// Main macro implementing all vectors
macro_rules! impl_vec {
    ($vec:ident { $($field:ident),* }) => (
        /// Optionally impl Ord if T also implements it
        impl<T: Ord> Ord for $vec<T> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.partial_cmp(other).unwrap()
            }
        }

        #[allow(dead_code)]
        impl<T: Copy + Into<f64>> $vec<T> {
            /// Calculates the euclidean (L2) norm of this vector
            pub fn euclidean_norm(self) -> f64 {
                self.euclidean_norm_squared().sqrt()
            }

            /// Calculates the squared euclidean (L2) norm of this vector
            pub fn euclidean_norm_squared(self) -> f64 {
                let mut result = 0.0;
                $(result += self.$field.into() * self.$field.into();)*
                result
            }
        }

        impl_vec_op!($vec { $($field),* }, Neg, neg);

        impl_vec_vec_op!($vec { $($field),* }, Add, add);
        impl_vec_vec_op!($vec { $($field),* }, Sub, sub);
        impl_vec_vec_assign!($vec { $($field),* }, AddAssign, add_assign);
        impl_vec_vec_assign!($vec { $($field),* }, SubAssign, sub_assign);

        impl_vec_scalar_op!($vec { $($field),* }, Mul, mul);
        impl_vec_scalar_op!($vec { $($field),* }, Div, div);
        impl_vec_scalar_op!($vec { $($field),* }, Rem, rem);
        impl_vec_scalar_assign!($vec { $($field),* }, MulAssign, mul_assign);
        impl_vec_scalar_assign!($vec { $($field),* }, DivAssign, div_assign);
        impl_vec_scalar_assign!($vec { $($field),* }, RemAssign, rem_assign);

        impl_vec_num!($vec { $($field),* }, i8);
        impl_vec_num!($vec { $($field),* }, i16);
        impl_vec_num!($vec { $($field),* }, i32);
        impl_vec_num!($vec { $($field),* }, i64);
        impl_vec_num!($vec { $($field),* }, isize);
    )
}

impl_vec!(Vector2 { x, y });
impl_vec!(Vector3 { x, y, z });
