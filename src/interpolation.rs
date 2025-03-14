//! Support routines for interpolation

use rlst::RlstScalar;

pub fn evaluate_1d_with_stride<T: RlstScalar>(
    eval_points: &[T],
    interp_nodes: &[T::Real],
    interp_values: &[T],
    interp_weights: &[T::Real],
    result: &mut [T],
    stride: usize,
) {
    assert_eq!(eval_points.len(), result.len());

    todo!();
}
