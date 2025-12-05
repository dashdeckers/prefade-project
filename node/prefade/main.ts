import { Email, NonEmptyStr, sendEmail } from "./index";

function sendWelcome(address: string): void {
  const email = Email.parse(address);         // validated in Rust
  const subject = NonEmptyStr.parse("Welcome!");
  const body = NonEmptyStr.parse("Thanks for joining.");
  sendEmail(email, subject, body);
}

sendWelcome("test@mail.com");
