import { IResult, Result } from "./results";
import { Parser } from "./parser";
import { Scanner } from "./rules";
import { IParserType } from "./parse";
import { TokenData } from "./tokenizer";

export type IRule<
  TTokenData extends TokenData = any,
  TParser extends Parser<TTokenData> = any
> = Rule<TTokenData>
  | IParserType<TParser>
  | Parser<TTokenData>
  | string
  | RegExp;

export type Rule<TTokenData extends TokenData = TokenData>
  = QuickRule<TTokenData>;

export type NormalizedRule<TTokenData extends TokenData>
  = (($: Scanner<TTokenData>) => Result);

type QuickRule<TTokenData extends TokenData>
  = (($: Scanner<TTokenData>) => IResult);

export default Rule;