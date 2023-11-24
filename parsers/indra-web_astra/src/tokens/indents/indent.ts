import { Rule, TokenParser } from "../../parser";

export namespace Indent {
  export class Parser extends TokenParser {
    get defaults() {
      return {
        name: "indent",
        types: ["whitespace", "indent"]
      }
    }

    rule: Rule = ({ NOT_IMPLEMENTED }) =>
      NOT_IMPLEMENTED();
  }

  export const parse
    = Parser.Parse;
}

export default Indent;