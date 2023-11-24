import {
  Rule,
  MATCH,
  UNEXPECTED,
} from "../parser";
import Assigner from "./assigner";

export namespace MutableFieldAssigner {
  export class Parser extends Assigner.Parser {
    get defaults() {
      return {
        name: 'mutable-field-assigner',
        types: ['assigner', 'operator']
      }
    }

    rule: Rule = ({ cursor }) =>
      (cursor.char === ':' && cursor.next.char.match(/\S/))
        ? MATCH(cursor)
        : UNEXPECTED({
          end: cursor,
          found: cursor.char,
          expected: /:\S/
        });
  }

  export const parse
    = Parser.Parse;
}

export default MutableFieldAssigner;