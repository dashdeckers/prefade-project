-- High-level Lua API for the `prefade` library.
--
-- This module re-exports classes and functions from the native Rust extension
-- module `prefade_native`. The native module is expected to be a shared
-- library (.dll/.so/.dylib) in the same directory.

local native = require("prefade_native")

return {
    Email = native.Email,
    NonEmptyStr = native.NonEmptyStr,
    send_email = native.send_email,
}
