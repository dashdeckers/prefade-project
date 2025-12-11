#![allow(unsafe_op_in_unsafe_fn)]
use prefade_core as core;
use pyo3::prelude::*;

/// Compute the average of a list of floats.
/// Returns `None` if the list is empty.
#[pyfunction]
fn compute_average(float_array: Vec<f64>) -> Option<f64> {
    core::compute_average(&float_array)
}

/// Python module definition.
#[pymodule]
fn _prefade_native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_average, m)?)?;
    Ok(())
}

