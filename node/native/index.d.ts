export class Email {
  constructor(raw: string);
  readonly value: string;
  domain(): string;
}

export class NonEmptyStr {
  constructor(raw: string);
  readonly value: string;
}

export function send_email(
  to: Email,
  subject: NonEmptyStr,
  body: NonEmptyStr
): void;
