import { Result } from "./result";
import { Unexpexted } from "./unexpected";
import { Error } from './error';
import { Location } from "../location";
import { Ignore } from "./ignore";
import { IRule } from "../rule";
import NoMatch from "./none";
import Match from "./match";

export type IResult
  = Result
  | IRule;

export default IResult;
export {
  IResult as IParserResult
}

function MATCH(args: ConstructorParameters<typeof Match>[0] & { trim?: boolean }): Match;
function MATCH(end: Location): Match;
function MATCH(args: Location | (ConstructorParameters<typeof Match>[0] & { trim?: boolean })): Match;
function MATCH(args: Location | (ConstructorParameters<typeof Match>[0] & { trim?: boolean })): Match {
  if (args?.hasOwnProperty('end')) {
    return new Match(args as ConstructorParameters<typeof Match>[0]);
  } else {
    return new Match({
      end: args as Location,
    });
  }
}

export { MATCH };

function FAIL(at: Location): Error;
function FAIL(childFailure: Result): Error;
function FAIL(args: ConstructorParameters<typeof Error>[0]): Error;
function FAIL(args?: Result | Location | ConstructorParameters<typeof Error>[0]): Error;
function FAIL(args?: Result | Location | ConstructorParameters<typeof Error>[0]): Error {
  if (!args) {
    return NONE();
  } else if (args[0] instanceof Result) {
    return new Error({
      end: args[0].end,
      children: [args[0]],
    });
  } else {
    return new Error(args[0]);
  }
};

export { FAIL };

function NONE(): NoMatch;
function NONE(...args: ConstructorParameters<typeof NoMatch>): NoMatch;
function NONE(...args: ConstructorParameters<typeof NoMatch> | []): NoMatch;
function NONE(...args: ConstructorParameters<typeof NoMatch> | []): NoMatch {
  return new NoMatch(...args);
};

export { NONE };

export function UNEXPECTED(...args: ConstructorParameters<typeof Unexpexted>) {
  return new Unexpexted(...args);
};

export function NOT_IMPLEMENTED(): Ignore {
  return new Ignore({
    message: 'Not implemented'
  });
}

export function IGNORE(args: ConstructorParameters<typeof Ignore>[0]): Ignore;
export function IGNORE(until?: Location): Ignore;
export function IGNORE(args?: unknown) {
  if (args?.hasOwnProperty('end')) {
    return new Ignore(args as ConstructorParameters<typeof Ignore>[0]);
  } else {
    return new Ignore({
      end: args as Location,
    });
  }
}

