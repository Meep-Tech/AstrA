import {
  Result,
  Match,
  Ignore,
  Unexpexted
} from "./results";
import {
  DEBUG,
  getRuleName,
  Scanner,
  normalizeRule
} from "./rules";
import {
  Context,
  Parent,
  ParserConstructor,
  ParserNamespace,
  Source
} from "./parse";
import { Cursor } from "./cursor";
import { Catch } from "./rules/catch";
import { Tests } from "./test";
import { IRule, NormalizedRule } from "./rule";
import NoMatch from "./results/none";
import Tokenizer, { TokenData } from "./tokenizer";
import Logger from "../utils/logs";

export const Exception
  = globalThis.Error;

export type DefaultTokenData<TTokenData extends TokenData = TokenData>
  = Omit<TTokenData, 'children' | 'properties'> & {
    children?: TTokenData['children'];
    properties?: TTokenData['properties'];
  } & {
    name: string;
  };

export interface IParser<
  TTokenData extends TokenData = any,
  TTokenDefaults extends DefaultTokenData<TTokenData> = any
> extends Parser<TTokenData, TTokenDefaults> { }

export abstract class Parser<
  TTokenData extends TokenData = TokenData,
  TTokenDefaults extends DefaultTokenData<TTokenData> = DefaultTokenData<TTokenData>
> {
  private _normal?: NormalizedRule<TTokenData>;

  //#region Static
  private static readonly _Instances: WeakMap<Function, Parser>
    = new WeakMap();
  private static _Instance?: Parser;

  private static _GetInstance() {
    if (!this._Instances.has(this)) {
      if (typeof this !== 'function') {
        return undefined;
      }

      var instance = new (this as unknown as ParserConstructor<any>)();
      if (!instance) {
        throw new Exception(`Parser._GetInstance() failed to create instance.`);
      }

      this._Instances.set(this, instance);
    }

    return this._Instances.get(this);
  }

  static get Instance(): Parser<any, any> {
    return this._GetInstance();
  }

  static Parse(source: Source, context?: Context): Result {
    return (this.Instance ?? (this as unknown as ParserNamespace).Parser.Instance)
      .parse(source, context);
  }
  //#endregion

  abstract get rule(): IRule<TTokenData, this>;

  get defaults(): TTokenDefaults {
    throw new Exception(`Parser.defaults is not implemented.`);
  }

  get name(): string {
    return this.defaults?.name;
  }

  get isDynamic(): boolean {
    return this.constructor.length > 0;
  }

  constructor() {
    if (!this.name) {
      throw new Exception("Token Rule Type Missing a Name");
    }

    DEBUG.LOG["GRAMMAR"]["INIT"]["TOKENS"][this.name][`_$bg${Math.floor(Math.random() * 255)}`](!this.isDynamic, `START`);
  }

  get normalized(): NormalizedRule<TTokenData> {
    return this._normal
      ??= normalizeRule(this.rule);
  }

  //#region Parse
  parse(code: string, context?: Context): Result;
  parse(code: string, parent?: Parent): Result;
  parse(cursor: Cursor, context?: Context): Result;
  parse(cursor: Cursor, parent?: Parent): Result;
  parse(source: Source, context?: Context): Result
  parse(source: Source, context?: Context): Result {
    // init
    DEBUG.LOG["PARSER"]._$green._$dim[getRuleName(this)]`START!`;
    const cursor
      = source instanceof Cursor
        ? source
        : Cursor.Init(source);
    const tokenBuilder: Tokenizer<TTokenData>
      = this.builderConstructor(cursor, context);

    // parse
    var result = this.normalized(
      new Scanner(cursor, tokenBuilder),
    );

    if (DEBUG.LOG.isEnabled) {
      switch (result.constructor) {
        case Match:
          DEBUG.LOG["PARSER"][getRuleName(this)]`MATCH! \n\t${result.toString().replace(/\n/g, '\n\t')}`;
          break;
        case Unexpexted:
          DEBUG.LOG["PARSER"][getRuleName(this)]`UNEXPECTED! \n\t${result.toString().replace(/\n/g, '\n\t')}`;
          break;
        case Error:
          DEBUG.LOG["PARSER"][getRuleName(this)]`ERROR! \n\t${result.toString().replace(/\n/g, '\n\t')}`;
          break;
        case NoMatch:
          DEBUG.LOG["PARSER"][getRuleName(this)]`UNMATCHED! \n\t${result.toString().replace(/\n/g, '\n\t')}`;
          break;
        case Ignore:
          DEBUG.LOG["PARSER"][getRuleName(this)]`IGNORED! \n\t${result.toString().replace(/\n/g, '\n\t')}`;
          break;
        default:
          throw new Exception(`Unknown Parser Result Type: ${result.constructor.name}`);
      }
    }

    return result;
  }
  //#endregion

  //#region Virtual
  protected readonly tests: Tests;
  [catcherKey: `catchFailed${string}`]: Catch;

  public readonly builderConstructor = (
    cursor: Cursor,
    context: Context
  ): Tokenizer<TTokenData> => ({
    ...(this.defaults as unknown as TTokenData),
    name: this.defaults?.name ?? '',
    types: this.defaults?.types
      ? this.defaults.name
        ? [this.defaults.name, ...this.defaults.types]
        : [...this.defaults.types]
      : [this.defaults.name ?? ''],
    start: cursor.location,
    end: cursor.location,
    parent: (context?.hasOwnProperty('end') ?? true)
      ? context as Parent
      : context.parent,
    children: [
      ...this.defaults.children ?? []
    ],
    properties: {
      ...this.defaults.properties ?? {}
    }
  });
  //#endregion
}

export default Parser;
export {
  Parser as TokenParser,
  Parser as ParserType,
}