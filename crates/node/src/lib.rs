use prefade_core as core;
use napi_derive::napi;

/// Compute the average of an array of floats.
/// Returns `null` if the array is empty.
#[napi]
pub fn compute_average(float_array: Vec<f64>) -> Option<f64> {
    core::compute_average(&float_array)
}

