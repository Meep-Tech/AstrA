import { test } from "../parser/test";
import NamedEntry from "../tokens/named-entry";

export function runTests(options?: {
  limit?: number,
  throwUnexpectedErrors?: boolean
}) {
  test(NamedEntry, options);
}

export default runTests;