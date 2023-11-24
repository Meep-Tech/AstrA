import { IParser } from "./parser";
import { Result, Match } from "./results";
import { Location } from "./location";

export type TokenData = {
  name?: string;
  types?: string[];
  start?: Location;
  end?: Location;
  children?: Result[];
  properties?: { [x: string | symbol]: unknown; };
};

export type DataOf<C extends IParser> = C extends IParser<infer T>
  ? T
  : unknown;

export type Tokenizer<TData extends TokenData = TokenData>
  = TData &
  Readonly<Pick<Match, 'parent'>> &
  Required<Pick<TData, 'name' | 'types' | 'start' | 'end' | 'children' | 'properties'>>;

export namespace Tokenizer {
  export type Data = TokenData;
}

export default Tokenizer;