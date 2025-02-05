#![feature(generic_const_exprs)]

use matrix_kit::matrix::*;
use matrix_kit::algebra::*;



#[cfg(test)]
mod matrix_tests {

    use super::*;

	#[test]
	fn test_mat_mul() {
		// test the super simple identity!
		let identity = Matrix::<2, 2, ZM<11>>::identity();
		let simple_vector = Matrix::<2, 1, ZM<11>>::from_flatmap([3.into(), 7.into()]);

		let out_vector = identity * simple_vector;

		assert_eq!(out_vector, simple_vector);

		let mat = Matrix::<2, 3, ZM<11>>::from_flatmap([3.into(), 2.into(), 5.into(), 1.into(), 7.into(), 0.into()]);
		let vec = Matrix::<3, 1, ZM<11>>::from_flatmap([1.into(), 4.into(), 9.into()]);
		let out = mat * vec;

		assert_eq!(out, Matrix::<2, 1, ZM<11>>::from_flatmap([9.into(), 6.into()]));

	}

	#[test]
	fn test_full_mat_mul() {
		let a = Matrix::<3, 4, ZM<11>>::from_flatmap([4.into(), 8.into(), 5.into(), 5.into(), 6.into(), 5.into(), 9.into(), 1.into(), 10.into(), 2.into(), 1.into(), 0.into()]);
		let b = Matrix::<4, 2, ZM<11>>::from_flatmap([1.into(), 2.into(), 8.into(), 3.into(), 1.into(), 4.into(), 0.into(), 7.into()]);

		let prod = a * b;
		

		assert_eq!(prod, Matrix::<3, 2, ZM<11>>::from_flatmap([4.into(), 9.into(), 7.into(), 5.into(), 6.into(), 3.into()]));
	}
}