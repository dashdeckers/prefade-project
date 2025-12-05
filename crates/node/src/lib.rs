use prefade_core::{self as core, Email as CoreEmail, NonEmptyStr as CoreNonEmptyStr};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Node.js wrapper around the Rust `Email` newtype.
#[napi]
pub struct Email {
    inner: CoreEmail,
}

#[napi]
impl Email {
    #[napi(constructor)]
    pub fn new(raw: String) -> Result<Self> {
        let inner = CoreEmail::parse(&raw).map_err(|e| Error::new(Status::InvalidArg, e))?;
        Ok(Email { inner })
    }

    #[napi(getter)]
    pub fn value(&self) -> String {
        self.inner.as_str().to_owned()
    }

    #[napi]
    pub fn domain(&self) -> String {
        self.inner.domain().to_owned()
    }
}

/// Node.js wrapper around the Rust `NonEmptyStr` newtype.
#[napi]
pub struct NonEmptyStr {
    inner: CoreNonEmptyStr,
}

#[napi]
impl NonEmptyStr {
    #[napi(constructor)]
    pub fn new(raw: String) -> Result<Self> {
        let inner =
            CoreNonEmptyStr::parse(&raw).map_err(|e| Error::new(Status::InvalidArg, e))?;
        Ok(NonEmptyStr { inner })
    }

    #[napi(getter)]
    pub fn value(&self) -> String {
        self.inner.as_str().to_owned()
    }
}

/// Node-visible function delegating to core.
#[napi]
pub fn send_email(to: &Email, subject: &NonEmptyStr, body: &NonEmptyStr) -> Result<()> {
    core::send_email(&to.inner, &subject.inner, &body.inner);
    Ok(())
}
