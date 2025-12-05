from prefade import Email, NonEmptyStr, send_email

def send_welcome(address: str) -> None:
    email = Email(address)          # validated in Rust
    subject = NonEmptyStr("Welcome!")
    body = NonEmptyStr("Thanks for joining.")
    send_email(email, subject, body)

if __name__ == "__main__":
    send_welcome("test@mail.com")
