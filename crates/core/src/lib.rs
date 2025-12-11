/// Compute the average of a slice of floats.
/// Returns `None` if the slice is empty.
pub fn compute_average(float_array: &[f64]) -> Option<f64> {
    if float_array.is_empty() {
        return None;
    }
    let sum: f64 = float_array.iter().sum();
    Some(sum / float_array.len() as f64)
}

