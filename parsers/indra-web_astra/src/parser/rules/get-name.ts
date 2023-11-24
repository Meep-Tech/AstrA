import { ParserConstructor, ParserNamespace } from "../parse";
import { Parser } from "../parser";
import { IRule, Rule } from "../rule";

export function getRuleName(e: IRule): string {
  return typeof e === 'string'
    ? e
    : e instanceof RegExp
      ? e.source
      : e.hasOwnProperty('Parser')
        ? (e as ParserNamespace).Parser.constructor.prototype.defaults?.name
        ?? (e as ParserNamespace).Parser.Instance.defaults?.name
        : e instanceof Parser
          ? e.defaults?.name ?? e.constructor.name
          : (e as ParserConstructor).Parse
            ? (e as ParserConstructor).prototype.defaults.name
            ?? (e as ParserConstructor).Instance.defaults?.name
            ?? (e as ParserConstructor).name
            : e.hasOwnProperty('name')
              ? (e as Rule).name
              : e.toString();
}

export default getRuleName;