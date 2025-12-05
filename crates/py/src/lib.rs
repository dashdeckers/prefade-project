#![allow(unsafe_op_in_unsafe_fn)]
#![allow(clippy::useless_conversion)]
use prefade_core::{self as core, Email as CoreEmail, NonEmptyStr as CoreNonEmptyStr};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Python wrapper around the Rust `Email` newtype.
#[pyclass]
pub struct Email {
    inner: CoreEmail,
}

#[pymethods]
impl Email {
    #[new]
    fn new(raw: &str) -> PyResult<Self> {
        let inner = CoreEmail::parse(raw).map_err(PyValueError::new_err)?;
        Ok(Email { inner })
    }

    /// The underlying string value (read-only).
    #[getter]
    fn value(&self) -> &str {
        self.inner.as_str()
    }

    /// The domain portion of the email.
    fn domain(&self) -> &str {
        self.inner.domain()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Email({:?})", self.inner.as_str()))
    }
}

/// Python wrapper around the Rust `NonEmptyStr` newtype.
#[pyclass]
pub struct NonEmptyStr {
    inner: CoreNonEmptyStr,
}

#[pymethods]
impl NonEmptyStr {
    #[new]
    fn new(raw: &str) -> PyResult<Self> {
        let inner = CoreNonEmptyStr::parse(raw).map_err(PyValueError::new_err)?;
        Ok(NonEmptyStr { inner })
    }

    #[getter]
    fn value(&self) -> &str {
        self.inner.as_str()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("NonEmptyStr({:?})", self.inner.as_str()))
    }
}

/// Python-visible function that delegates to Rust domain logic.
#[pyfunction]
fn send_email(to: &Email, subject: &NonEmptyStr, body: &NonEmptyStr) -> PyResult<()> {
    core::send_email(&to.inner, &subject.inner, &body.inner);
    Ok(())
}

/// Python module definition.
#[pymodule]
fn _prefade_native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Email>()?;
    m.add_class::<NonEmptyStr>()?;
    m.add_function(wrap_pyfunction!(send_email, m)?)?;
    Ok(())
}
