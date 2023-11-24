import { IResult, Result, UNEXPECTED } from "../results";
import { DEBUG } from "./scanner";
import { Parser } from "../parser";
import { IRule, NormalizedRule, Rule } from "../rule";
import { TokenData } from "../tokenizer";
import { ParserConstructor } from "../parse";

export function normalizeRule<TTokenData extends TokenData = TokenData>(
  rule: IRule
): NormalizedRule<TTokenData> {
  var parser
    = _normalize<TTokenData>(rule);

  return $ => {
    var result: IResult = parser($);

    while (!(result instanceof Result)) {
      parser = _normalize(result);
      result = parser($);
    }

    return result;
  };
}

export default normalizeRule;

function _normalize<TTokenData extends TokenData = TokenData>(rule: IRule): Rule<TTokenData> {
  if (rule instanceof Function) {
    if (rule.prototype instanceof Parser) {
      return normalizeRule((rule as ParserConstructor).Instance.rule);
    } else {
      if (rule.length === 1) {
        return rule as Rule<TTokenData>;
      } else {
        return $ => (rule as Function)($.cursor, $.token, DEBUG);
      }
    }
  } // match the text exactly
  else if (typeof rule === 'string') {
    return ({ cursor, MATCH }) => {
      const found = cursor.peek(rule.length);
      return found === rule
        ? MATCH(cursor.at(rule.length))
        : UNEXPECTED({
          end: cursor.at(rule.length),
          expected: rule,
          found
        });
    };
  } else if (rule instanceof RegExp) {
    return ({ cursor, MATCH }) => {
      const found = cursor.peek(rule);
      return found
        ? MATCH(cursor.at(found.length))
        : UNEXPECTED({
          end: cursor.at(found.length),
          expected: rule,
          found
        });
    };
  } else if (rule instanceof Parser) {
    return normalizeRule((rule as Parser).rule);
  } else {
    return normalizeRule((rule.Parser.Instance as Parser).rule);
  }
}
