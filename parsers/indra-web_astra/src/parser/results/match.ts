import { Location } from "../location";
import { IgnoredResultParams, Result } from "./result";

export class Match extends Result {
  isError: false = false;
  isSuccess: true = true;

  skipped?: { before: number, after: number };

  constructor(init: { end: Location } & Partial<Omit<Match, IgnoredResultParams>>) {
    super(init);

    this.name = init?.name ?? '';
    this.types = init?.types ? [this.name, ...init.types] : [this.name];
    this.start = (init?.start)!;
    this.end = (init?.end)!;
  }
}

export default Match;

export {
  Match as ParserToken
}