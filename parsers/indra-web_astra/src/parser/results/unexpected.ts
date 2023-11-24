import { Location } from "../location";
import { IRule } from "../rule";
import { getRuleName } from "../rules";
import { IgnoredResultParams } from "./result";
import { Error } from "./error";

export class Unexpexted extends Error {
  found: string;
  expected?: string;

  constructor(args: {
    found: string;
    end: Location;
    expected?: IRule | IRule[];
  } & Partial<Omit<Unexpexted, 'expected' | IgnoredResultParams>>) {
    super(args);
    this.found = args.found;
    this.expected = args.expected instanceof Array
      ? args.expected
        .map(getRuleName)
        .join(' | ')
      : getRuleName(args.expected);
    this.message
      = `Unexpected token: ❝${this.found}❞`
      + (this.expected
        ? `; expected: ❝${this.expected}❞.`
        : '.')
      + (this.message
        ? ` ${this.message}`
        : '');
  }
}

export default Unexpexted;
