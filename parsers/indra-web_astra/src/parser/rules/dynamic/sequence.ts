import { TokenParser } from "../../parser";
import { IResult } from "../../results/results";
import Rule, { IRule } from "../../rule";
import getRuleName from "../get-name";
import { Ignore, Match, NoMatch } from "../../results";

export namespace Sequence {
  export class Parser extends TokenParser {
    readonly elements: Readonly<IRule[]>;

    get defaults() {
      return {
        name: 'seq'
      }
    }

    get isDynamic(): boolean {
      return this.constructor === Parser;
    }

    constructor(elements: IRule[]) {
      super();
      this.elements = Object.freeze([...elements]);
    }

    rule: Rule = ({ cursor, token, LOG, MATCH, FAIL, PARSE: PARSE }) => {
      LOG["SEQ"]["START"]`${this.elements.length} elements: ${this.elements.map(r => '\n\t - ' + getRuleName(r)).join()}`
      for (var rule of this.elements) {
        LOG["SEQ"]["EL"][getRuleName(rule)]`START`;
        var result = PARSE(rule, cursor, token);

        if (result instanceof Error || result instanceof NoMatch) {
          LOG["SEQ"]["EL"][getRuleName(rule)]`FAIL`;
          return FAIL(result);
        }

        cursor = cursor.at(result.end);
        if (result instanceof Ignore) {
          LOG["SEQ"]["EL"][getRuleName(rule)]("IGNORED");
        } else {
          token.children.push(result);
          LOG["SEQ"]["EL"][getRuleName(rule)]`APPENDED`;
        }
      }

      return MATCH(cursor);
    };
  }
}

export default Sequence;
