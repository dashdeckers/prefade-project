use prefade_core::{self as core, Email as CoreEmail, NonEmptyStr as CoreNonEmptyStr};
use mlua::prelude::*;

/// Lua wrapper around the Rust `Email` newtype.
#[derive(Clone)]
pub struct Email {
    inner: CoreEmail,
}

impl LuaUserData for Email {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // Constructor (called via Email.new("test@mail.com"))
        methods.add_function("new", |_, raw: String| {
            let inner = CoreEmail::parse(&raw)
                .map_err(|e| LuaError::RuntimeError(e.to_string()))?;
            Ok(Email { inner })
        });

        // Getter for value property
        methods.add_method("value", |_, this, ()| {
            Ok(this.inner.as_str().to_owned())
        });

        // Domain method
        methods.add_method("domain", |_, this, ()| {
            Ok(this.inner.domain().to_owned())
        });

        // __tostring metamethod
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(this.inner.as_str().to_owned())
        });
    }
}

/// Lua wrapper around the Rust `NonEmptyStr` newtype.
#[derive(Clone)]
pub struct NonEmptyStr {
    inner: CoreNonEmptyStr,
}

impl LuaUserData for NonEmptyStr {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, raw: String| {
            let inner = CoreNonEmptyStr::parse(&raw)
                .map_err(|e| LuaError::RuntimeError(e.to_string()))?;
            Ok(NonEmptyStr { inner })
        });

        methods.add_method("value", |_, this, ()| {
            Ok(this.inner.as_str().to_owned())
        });

        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(this.inner.as_str().to_owned())
        });
    }
}

/// Lua-visible function delegating to core.
fn send_email(
    _lua: &Lua,
    (to, subject, body): (LuaAnyUserData, LuaAnyUserData, LuaAnyUserData),
) -> LuaResult<()> {
    let to_email = to.borrow::<Email>()?;
    let subject_str = subject.borrow::<NonEmptyStr>()?;
    let body_str = body.borrow::<NonEmptyStr>()?;
    
    core::send_email(&to_email.inner, &subject_str.inner, &body_str.inner);
    Ok(())
}

/// Lua module definition.
#[mlua::lua_module]
fn prefade_native(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("Email", lua.create_proxy::<Email>()?)?;
    exports.set("NonEmptyStr", lua.create_proxy::<NonEmptyStr>()?)?;
    exports.set("send_email", lua.create_function(send_email)?)?;
    Ok(exports)
}
