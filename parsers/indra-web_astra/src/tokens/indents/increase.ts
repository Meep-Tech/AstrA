import { Rule, Scanner, IResult } from "../../parser";
import Indent from "./indent";

export namespace IncreaseIndent {

  export class Parser extends Indent.Parser {
    get defaults() {
      return {
        name: "indent-increase",
        types: ["whitespace", "indent"]
      }
    }

    rule: Rule = ({ cursor, UNEXPECTED, MATCH, start }: Scanner): IResult => {
      do if (!cursor.current.char.match(/\s/)) break;
      while (cursor = cursor.next);

      if (cursor.indents.current.length > start.indents.current.length) {
        return MATCH(cursor);``
      } else {
        return UNEXPECTED({
          end: cursor,
          found: cursor.at(cursor.index - cursor.column).to(cursor),
          expected: cursor.indents.current.join(),
          message: "Expected an Increase in Indentation Level"
        });
      }
    }
  }

  export const parse
    = Parser.Parse;
}

export default IncreaseIndent;