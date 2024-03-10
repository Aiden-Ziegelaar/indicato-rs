//! # Indicato_rs
//! This crate provides simple primitives for statistical analysis of time series stochastic data. 


/// The error module contains the error types used in the crate.
pub mod fin_error;

/// The signals module contains the signal types that can be created.
pub mod signals;

/// The traits module contains the traits that are used to define the functionality signals.
pub mod traits;

/// The math module contains calculations that are once-off, as opposed to signals which are aggregations
pub mod dequeue_math;