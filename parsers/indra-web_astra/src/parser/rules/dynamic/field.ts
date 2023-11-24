import { IResult, Ignore } from "../../results";
import Rule, { IRule } from "../../rule";
import { TokenParser } from "../../parser";
import getRuleName from "../get-name";

export namespace Field {
  export class Parser extends TokenParser {
    private readonly _key: string;
    private readonly _value: IRule;

    get defaults() {
      return {
        name: 'field'
      }
    }

    get key(): string {
      return this._key;
    }

    get isDynamic(): boolean {
      return this.constructor === Parser;
    }

    get value(): IRule {
      return this._value;
    }

    constructor(key: string, rule: IRule) {
      super();
      this._key = key;
      this._value = rule;
    }

    rule: Rule = ({ cursor, token, LOG, IGNORE, FAIL, PARSE: PARSE }) => {
      LOG["FIELD"][this.key]["START"]`${getRuleName(this.value)}`;
      var result = PARSE(this._value, cursor, token.parent);
      if (result.isSuccess || result instanceof Ignore) {
        LOG["FIELD"]["MATCH"]`${getRuleName(this.value)}`;
        token.parent.properties[this._key] = result;
        return IGNORE(result.end);
      } else {
        LOG["FIELD"]["FAIL"]`${getRuleName(this.value)}`;
        return FAIL(result);
      }
    };
  }
}

export default Field;