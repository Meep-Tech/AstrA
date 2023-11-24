import { Rule, Scanner, IResult } from "../../parser";
import Indent from "./indent";

export namespace CurrentIndent {
  export class Parser extends Indent.Parser {
    get defaults() {
      return {
        name: "indent-current",
        types: ["whitespace", "indent"]
      };
    }

    rule: Rule = ({ start, cursor, MATCH, UNEXPECTED }: Scanner): IResult => {
      do if (!cursor.current.char.match(/\s/)) break;
      while (cursor = cursor.next);

      if (cursor.line !== start.line
        && cursor.indents.current.length === cursor.indents.previous.length
      ) {
        return MATCH(cursor);
      } else {
        return UNEXPECTED({
          end: cursor,
          found: cursor.at(cursor.index - cursor.column).to(cursor),
          expected: cursor.indents.current.join(),
          message: "Expected the same Indentation Level"
        });
      }
    }
  }

  export const parse
    = Parser.Parse;
}

export default CurrentIndent;