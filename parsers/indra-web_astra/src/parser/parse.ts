import { Cursor } from "./cursor";
import { Ignore, Match, Result, Unexpexted } from "./results";
import { IRule } from "./rule";
import { DEBUG, Scanner, getRuleName, normalizeRule } from "./rules";
import Tokenizer, { TokenData } from "./tokenizer";
import { Parser } from "./parser";
import { hasProp } from "../utils";
import NoMatch from "./results/none";

//#region Types

export type IParserType<TType extends Parser = any>
  = ParserConstructor<TType>
  | IParserNamespace<TType>;

export type ParserOf<C extends IParserType> = C extends IParserType<infer T>
  ? T
  : unknown;

export type ParserConstructor<
  TType extends Parser = Parser
> = {
  new(): TType
  Instance: TType
} & Omit<typeof Parser, 'constructor'>;

export type ParserNamespace<TType extends Parser = Parser>
  = IParserNamespace<TType>;

export interface IParserNamespace<TType extends Parser = any> {
  readonly Parser: ParserConstructor<TType>;
  readonly Data?: TokenData;

  parse(source: Source, context?: Context): Result;
}

export type Source
  = string | Cursor;

export type Context
  = ({
    parent?: Parent,
    partial?: Result,
  } & ({
    cursor?: Cursor,
  } | {
    source?: Source,
  } | {
    code?: Source,
  }))

export type Parent
  = Result;

//#endregion

//#region Functions
export function parse<TRule extends IRule>(
  rule: TRule,
  source: Source,
  parent?: Parent
): Result;
export function parse<TRule extends IRule>(
  rule: TRule,
  cursor: Cursor,
  context?: Context
): Result;
export function parse<TRule extends IRule>(
  rule: TRule,
  context?: Context
): Result
export function parse<TRule extends IRule>(
  rule: TRule,
  sourceOrContext?: Source | Context,
  contextOrParent?: Context | Parent
): Result {
  DEBUG.LOG["PARSER"]._$green._$dim[getRuleName(rule)]`START`;
  DEBUG.LOG["PARSER"][getRuleName(rule)]`NORMALIZE`;
  var normalized = normalizeRule(rule);
  var parent: Result | undefined;
  var cursor: Cursor;

  if (sourceOrContext instanceof Cursor) {
    cursor = sourceOrContext;
  } else if (typeof sourceOrContext === 'string') {
    cursor = Cursor.Init(sourceOrContext);
  } else {
    contextOrParent = sourceOrContext;
  }

  if (contextOrParent.hasOwnProperty('end')) {
    parent = contextOrParent as Result;
  } else {
    if (contextOrParent?.parent) {
      parent = contextOrParent.parent;
    }

    if (!cursor) {
      if (hasProp(contextOrParent, 'cursor') && contextOrParent.cursor instanceof Cursor) {
        cursor = contextOrParent.cursor;
      } else if (hasProp(contextOrParent, 'source')) {
        cursor = contextOrParent.source instanceof Cursor
          ? contextOrParent.source
          : Cursor.Init(contextOrParent.source as string);
      } else if (hasProp(contextOrParent, 'code')) {
        cursor = Cursor.Init(contextOrParent.code as string);
      }
    }
  }

  var builder: Tokenizer;
  if (rule instanceof Parser) {
    builder = rule.builderConstructor(cursor, parent);
    DEBUG.LOG["PARSER"][getRuleName(rule)]["BUILDER"]`INIT`;
  } else if (rule.hasOwnProperty('Parser')) {
    builder = (rule as ParserNamespace).Parser.Instance.builderConstructor(cursor, parent);
    DEBUG.LOG["PARSER"][getRuleName(rule)]["BUILDER"]`INIT`;
  } else {
    DEBUG.LOG["PARSER"][getRuleName(rule)]["BUILDER"]`ANONYMOUS`;
    builder = {
      name: '',
      children: [],
      properties: {},
      types: [],
      start: cursor.location,
      end: cursor.location,
      parent: parent,
      ...cursor,
    };
  }

  var result = normalized(new Scanner(cursor, builder));
  result.end ??= cursor.location;

  if (DEBUG.LOG.isEnabled) {
    switch (result.constructor) {
      case Match:
        DEBUG.LOG["PARSER"][getRuleName(rule)]`MATCH! \n\t${result.toString().replace(/\n/g, '\n\t')}`;
        break;
      case Unexpexted:
        DEBUG.LOG["PARSER"][getRuleName(rule)]`UNEXPECTED! \n\t${result.toString().replace(/\n/g, '\n\t')}`;
        break;
      case Error:
        DEBUG.LOG["PARSER"][getRuleName(rule)]`ERROR! \n\t${result.toString().replace(/\n/g, '\n\t')}`;
        break;
      case NoMatch:
        DEBUG.LOG["PARSER"][getRuleName(rule)]`UNMATCHED! \n\t${result.toString().replace(/\n/g, '\n\t')}`;
        break;
      case Ignore:
        DEBUG.LOG["PARSER"][getRuleName(rule)]`IGNORED! \n\t${result.toString().replace(/\n/g, '\n\t')}`;
        break;
      default:
        throw new globalThis.Error(`Unknown Parser Result Type: ${result.constructor.name}`);
    }
  }
  return result;
}

//#endregion

export default parse;
