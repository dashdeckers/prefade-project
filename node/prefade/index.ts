// High-level TypeScript API for the `prefade` library.
//
// This wraps the native N-API addon built from the `crates/node` Rust crate.
// The native module is expected to export classes and functions matching
// the declarations in `native/index.d.ts`.

import type * as Native from "../native";

const native: typeof import("../native") =
  // eslint-disable-next-line @typescript-eslint/no-var-requires
  require("../prefade_node.node");

export class Email {
  private inner: Native.Email;

  private constructor(inner: Native.Email) {
    this.inner = inner;
  }

  static parse(raw: string): Email {
    const inner = new native.Email(raw);
    return new Email(inner);
  }

  get value(): string {
    return this.inner.value;
  }

  domain(): string {
    return this.inner.domain();
  }

  toString(): string {
    return this.value;
  }
}

export class NonEmptyStr {
  private inner: Native.NonEmptyStr;

  private constructor(inner: Native.NonEmptyStr) {
    this.inner = inner;
  }

  static parse(raw: string): NonEmptyStr {
    const inner = new native.NonEmptyStr(raw);
    return new NonEmptyStr(inner);
  }

  get value(): string {
    return this.inner.value;
  }

  toString(): string {
    return this.value;
  }
}

export function sendEmail(
  to: Email,
  subject: NonEmptyStr,
  body: NonEmptyStr
): void {
  (native as any).sendEmail(
    (to as any).inner,
    (subject as any).inner,
    (body as any).inner
  );
}
