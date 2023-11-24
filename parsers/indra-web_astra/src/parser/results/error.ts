import { Location } from "../location";
import Result, { IgnoredResultParams } from "./result";

export class Error extends Result {
  isError: true = true;
  isSuccess: false = false;
  message?: string;

  constructor(args: { end: Location } & Partial<Omit<Error, IgnoredResultParams>>) {
    super(args);
    this.message = args?.message;
  }
}

export default Error;
export {
  Error as ParserError
};
