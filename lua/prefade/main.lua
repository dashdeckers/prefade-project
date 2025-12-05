local mydomain = require("prefade.init")
local Email = mydomain.Email
local NonEmptyStr = mydomain.NonEmptyStr
local send_email = mydomain.send_email

local function send_welcome(address)
    local email = Email.new(address)           -- validated in Rust
    local subject = NonEmptyStr.new("Welcome!")
    local body = NonEmptyStr.new("Thanks for joining.")
    send_email(email, subject, body)
end

send_welcome("test@mail.com")
