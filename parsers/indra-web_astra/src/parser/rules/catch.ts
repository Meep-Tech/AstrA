import { Cursor } from "../cursor";
import { IParserType, ParserOf } from "../parse";
import { IResult } from "../results";
import { DataOf } from "../tokenizer";

export type CatchFunction<TType extends IParserType>
  = Catch<TType>
  & { type: TType };

export type CatchConfig<TType extends IParserType> = {
  type: TType;
  catch: Catch<TType>;
};

export type Catcher<TType extends IParserType = IParserType>
  = CatchFunction<TType>
  | CatchConfig<TType>

export function tryCatch<TType extends IParserType>(
  rule: TType,
  catcher: Catch<TType>
): Catch<TType> {
  return Object.assign(
    catcher,
    { type: rule }
  );
}

export type Catch<TType extends IParserType = IParserType> = (
  cursor: Cursor,
  token: DataOf<ParserOf<TType>>
) => IResult;

export default Catch;