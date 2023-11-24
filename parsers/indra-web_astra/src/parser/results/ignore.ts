import { Result } from "./result";
import { Location } from "../location";

export class Ignore extends Result {
  isError: false = false;
  isSuccess: boolean = false;
  isIgnored: true;
  constructor(init?: { message?: string, end?: Location, start?: Location }) {
    super(init as { end: Location; });
  }
}
