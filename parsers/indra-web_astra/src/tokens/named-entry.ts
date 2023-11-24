import {
  TokenParser,
  Rule,
  Result,
  Scanner
} from "../parser";
import { Name } from "./name";
import { Assigner } from "./assigner";
import { Text } from "./value";
import { MutableFieldAssigner } from "./mutable-field-assigner";
import Indent from "./indents";

export namespace NamedEntry {
  export type Data = {
    name: string;
    children?: undefined;
    properties?: {
      key?: Result;
      operator?: Result;
      value?: Result;
    };
  };

  export class Parser extends TokenParser<Data> {
    get defaults() {
      return {
        name: "named-entry",
        types: ["statement", "assignment", "entry"]
      }
    }

    /**
     * named-entry
     *   - key: keys.name
     *   - ?increase-indent | ?gap
     *   - operator: assigner
     *   - ?increase-indent | ?gap
     *   - value: text 
     */
    rule: Rule<Data> = ({
      FIELD,
      OPTIONAL,
      SEQUENCE,
      CHOICE
    }: Scanner<Data>) =>
      SEQUENCE(
        FIELD("key", Name),
        OPTIONAL(//CHOICE(
          Indent.Increase,
          //WS.Gap
        ),//),
        FIELD("operator", Assigner),
        OPTIONAL(CHOICE(
          Indent.Increase,
          //WS.Gap
        )),
        FIELD("value", Text),
      );

    tests = {
      "Mutable Text": {
        input: "key: value",
        expected: {
          name: "named-entry",
          start: 0,
          end: 10,
          properties: {
            key: Name,
            operator: MutableFieldAssigner,
            value: Text
          }
        }
      },
      "Multiline Mutable Text": {
        input: "key:\n\tvalue",
        expected: {
          name: "named-entry",
          start: 0,
          end: 10,
          elements: {
            key: Name,
            operator: MutableFieldAssigner,
            2: Indent.Increase,
            value: Text
          }
        }
      },
      "Multiline Indented Mutable Text": {
        input: "key\n\t: value",
        expected: {
          name: "named-entry",
          start: 0,
          end: 10,
          elements: {
            key: Name,
            1: Indent.Increase,
            operator: MutableFieldAssigner,
            value: Text
          }
        }
      },
      "Multiline Multi-Indented Mutable Text": {
        input: "key\n\t:\n\t\tvalue",
        expected: {
          name: "named-entry",
          start: 0,
          end: 10,
          elements: {
            key: Name,
            1: Indent.Increase,
            operator: MutableFieldAssigner,
            3: Indent.Increase,
            value: Text
          }
        }
      },
      "Multiline Branched Mutable Text": {
        input: "key\n\t:\n\tvalue",
        expected: {
          name: "named-entry",
          start: 0,
          end: 10,
          elements: {
            key: Name,
            1: Indent.Increase,
            operator: MutableFieldAssigner,
            3: Indent.Current,
            value: Text
          }
        }
      },
    }
  }

  export const parse
    = Parser.Parse;
}

export default NamedEntry;