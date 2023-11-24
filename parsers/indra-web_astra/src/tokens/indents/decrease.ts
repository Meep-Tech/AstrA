import { Rule, Scanner, IResult, TokenParser } from "../../parser";
import { Indent } from "./indent";

export namespace DecreaseIndent {
  export class Parser extends Indent.Parser {
    get defaults() {
      return {
        name: "indent-decrease",
        types: ["whitespace", "indent"]
      };
    }

    rule: Rule = ({ cursor, MATCH, UNEXPECTED }: Scanner): IResult => {
      do if (!cursor.current.char.match(/\s/)) break;
      while (cursor = cursor.next);

      if (cursor.indents.current.length < cursor.indents.previous.length) {
        return MATCH(cursor);
      } else {
        return UNEXPECTED({
          end: cursor,
          found: cursor.at(cursor.index - cursor.column).to(cursor),
          expected: cursor.indents.current.join(),
          message: "Expected a Decrease in Indentation Level"
        });
      }
    }
  }

  export const parse
    = Parser.Parse;
}

export default DecreaseIndent;