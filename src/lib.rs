//! Efficient evaluation of Chebychev approximations

mod cheb1;
mod interpolation;

pub use cheb1::*;

/// Specify Chebychev points of first or second kind
pub enum Kind {
    First,
    Second,
}
