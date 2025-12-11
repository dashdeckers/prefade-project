// High-level TypeScript API for the `prefade` library.
//
// This wraps the native N-API addon built from the `crates/node` Rust crate.
// The native module is expected to export classes and functions matching
// the declarations in `native/index.d.ts`.

const native: typeof import("../native") =
  // eslint-disable-next-line @typescript-eslint/no-var-requires
  require("../prefade_node.node");

export function computeAverage(floatArray: number[]): number | null {
  return native.computeAverage(floatArray);
}

