import {
  FAIL,
  MATCH,
  NONE,
  IGNORE,
  NOT_IMPLEMENTED,
  UNEXPECTED,
  Result,
  Ignore
} from "../results";
import parse from "../parse";
import optional, { Optional } from "./dynamic/optional";
import { Field } from "./dynamic/field";
import { Cursor } from "../cursor";
import Tokenizer, { TokenData } from "../tokenizer";
import { Logger } from "../../utils/logs";
import { IRule } from "../rule";
import Sequence from "./dynamic/sequence";
import { Choice } from ".";

export const RESULTS = {
  FAIL: FAIL,
  MATCH: MATCH,
  NONE: NONE,
  IGNORE: IGNORE,
  UNEXPECTED: UNEXPECTED,
  NOT_IMPLEMENTED: NOT_IMPLEMENTED
}

export type Builders = {
  readonly PARSE: typeof parse;
  readonly FIELD: (name: string, rule: IRule) => Field.Parser;
  readonly OPTIONAL: (rule: IRule) => Optional.Parser;
  readonly SEQUENCE: (...args: IRule[]) => Sequence.Parser;
  readonly CHOICE: (...args: IRule[]) => Choice.Parser;
};

export const DEBUG = {
  LOG: new Logger(),
};

export class Scanner<TTokenData extends TokenData = TokenData> {
  private _logger: Logger;
  readonly start: Cursor;
  readonly cursor: Cursor;
  readonly token: Tokenizer<TTokenData>;

  get index() { return this.cursor.index; }
  get line() { return this.cursor.line; }
  get column() { return this.cursor.column; }

  get $() { return this as Builders; }

  constructor(
    cursor: Cursor,
    token: Tokenizer<TTokenData>,
  ) {
    this.start = cursor;
    this.cursor = cursor;
    this.token = token;
    this._logger = new Logger("PARSER", token.name);
  }

  get PARSE() {
    return parse;
  }

  FIELD(name: string, rule: IRule): Field.Parser {
    return new Field.Parser(name, rule);
  }

  OPTIONAL(rule: IRule): Optional.Parser {
    return new Optional.Parser(rule);
  }

  SEQUENCE(...args: IRule[]): Sequence.Parser {
    return new Sequence.Parser(args);
  }

  CHOICE(...args: IRule[]): Choice.Parser {
    return new Choice.Parser(args);
  }

  get FAIL(): typeof FAIL {
    return function fail(args: Parameters<typeof FAIL>[0]): ReturnType<typeof FAIL> {
      if (args instanceof Result) {
        return FAIL(Object.assign(args, { ...this.token, ...args, start: this.start }));
      } else if (args.hasOwnProperty("index") && !args.hasOwnProperty("end")) {
        return FAIL({ ...this.token, end: args, start: this.start });
      } else {
        return FAIL({ ...this.token, ...args, start: this.start });
      }
    }.bind(this);
  }

  get MATCH(): typeof MATCH {
    return function match(args: Parameters<typeof MATCH>[0]): ReturnType<typeof MATCH> {
      if (args.hasOwnProperty("index") && !args.hasOwnProperty("end")) {
        return MATCH({ ...this.token, end: args, start: this.start });
      } else {
        return MATCH({ ...this.token, ...args, start: this.start });
      }
    }.bind(this);
  }

  get NONE(): typeof NONE {
    return function none(args?: Parameters<typeof NONE>[0]): ReturnType<typeof NONE> {
      if (args.hasOwnProperty("index") && !args.hasOwnProperty("end")) {
        return NONE({ ...this.token, end: args, start: this.start });
      } else if (args) {
        return NONE({ ...this.token, ...args, end: this.token.end ?? this.start, start: this.start });
      } else {
        return NONE({ ...this.token, end: this.token.end ?? this.start, start: this.start });
      }
    }.bind(this);
  }

  get UNEXPECTED(): typeof UNEXPECTED {
    return function unexpected(args: Parameters<typeof UNEXPECTED>[0]): ReturnType<typeof UNEXPECTED> {
      return UNEXPECTED({ ...this.token, ...args, start: this.start, end: this.token.end ?? this.start });
    }.bind(this);
  }

  get IGNORE(): typeof IGNORE {
    return function ignore(until?: Cursor): ReturnType<typeof IGNORE> {
      return IGNORE({ ...this.token, end: until ?? this.token.end ?? this.start, start: this.start });
    }.bind(this);
  }

  get NOT_IMPLEMENTED(): typeof NOT_IMPLEMENTED {
    return function not_implemented() {
      return new Ignore({
        message: 'Not implemented',
        start: this.start,
        end: this.start,
      });
    }.bind(this);
  }

  get LOG() {
    return this._logger;
  }
}