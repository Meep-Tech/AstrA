import { Match } from "../..";
import { TokenParser } from "../../parser";
import Rule, { IRule } from "../../rule";
import getRuleName from "../get-name";

export namespace Optional {
  export class Parser extends TokenParser {
    readonly target: IRule;

    get defaults() {
      return {
        name: 'optional'
      }
    }

    get isDynamic(): boolean {
      return this.constructor === Parser;
    }

    constructor(rule: IRule) {
      super();
      this.target = rule;
    }

    rule: Rule = ({ cursor, token, PARSE: PARSE, IGNORE, LOG }) => {
      LOG["OPTIONAL"]["START"]`${getRuleName(this.target)}`;
      var result = PARSE(this.target, cursor, token);
      if (result instanceof Match) {
        LOG["OPTIONAL"]["MATCH"]`${getRuleName(this.target)}`;
        return result;
      } else {
        LOG["OPTIONAL"]["IGNORE"]`${getRuleName(this.target)}`;
        return IGNORE();
      }
    }
  }
}

export default Optional;