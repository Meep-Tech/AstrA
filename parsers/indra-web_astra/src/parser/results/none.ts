import { Location } from "../location";
import Result, { IgnoredResultParams } from "./result";
import { Error } from "./error";

export class NoMatch extends Result {
  isError: true = true;
  isSuccess: false = false;
  message?: string;

  constructor(args?: Partial<Omit<Error, IgnoredResultParams>>) {
    super(args as { end: Location; });
    this.message = args?.message;
  }
}

export default NoMatch;