import { Scanner, getRuleName } from '..';
import { Ignore, Result, IResult, Match } from '../../results';
import { IRule } from '../../rule';
import * as Parsers from '../../parser';

export namespace Choice {
  export class Parser extends Parsers.Parser {
    private readonly _options: Readonly<IRule[]>;

    get defaults() {
      return {
        name: "choice"
      }
    }

    get isDynamic(): boolean {
      return this.constructor === Parser;
    }

    get options(): Readonly<IRule[]> {
      return this._options;
    }

    constructor(options: IRule[]) {
      super();
      this._options = Object.freeze([...options]);
    }

    rule: IRule = ({
      cursor,
      token,
      PARSE: PARSE,
      NONE,
      LOG
    }: Scanner): IResult => {
      var results: Result[] = [];
      var firstIgnoredIndex: number | undefined;
      const options = this.options;

      const key = token.types.includes('splayed')
        ? "SPLAY"
        : "CHOICE";
      LOG[key]["START"]`${options.length} options: ${options.map(r => '\n\t - ' + getRuleName(r)).join(',')}`
      var index = 0;
      for (const option in options) {
        LOG[key]["TRY"][getRuleName(option)]['START']`#${index}`;
        var result = PARSE(option, cursor, token);
        results.push(result);
        if (result instanceof Ignore) {
          LOG[key]["TRY"][getRuleName(option)]["IGNORE"]`#${index} ${firstIgnoredIndex !== undefined
            ? ' (*First Ignored)'
            : ''}`;
          firstIgnoredIndex ??= index;

          index++;
          continue;
        } else if (!(result instanceof Match)) {
          LOG[key]["TRY"][getRuleName(option)]["FAIL"]`#${index}`;
          var catchers = this.options
            .map(option => option[`catchFailed${getRuleName(option)}`])
            .filter(catcher => catcher);

          LOG[key]["TRY"][getRuleName(option)]["CATCH"]["START"](catchers.any, `match failed, trying catchers: ${catchers.map(c => '\n\t - ' + getRuleName(c)).join(',')}`);
          var catcherIndex = 0;
          for (const catcher of catchers) {
            LOG[key]["TRY"][getRuleName(option)]["CATCH"][getRuleName(catcher)][`START`]`#${catcherIndex} (${index}:${catcherIndex + index})`;
            result = catcher(cursor, token.parent);
            results.push(result);
            if (result instanceof Ignore) {
              LOG[key]["TRY"][getRuleName(option)]["CATCH"][getRuleName(catcher)][`IGNORE`]`#${catcherIndex} (${index}:${catcherIndex + index}) ${firstIgnoredIndex !== undefined
                ? ' (*First Ignored)'
                : ''}`;
              firstIgnoredIndex ??= index + catcherIndex;
              catcherIndex++;

              continue;
            } else if (!(result instanceof Match)) {
              LOG[key]["TRY"][getRuleName(option)]["CATCH"][getRuleName(catcher)][`FAIL`]`#${catcherIndex} (${index}:${catcherIndex + index})`;
              catcherIndex++;
              continue;
            } else {
              LOG[key]["TRY"][getRuleName(option)]["CATCH"][getRuleName(catcher)][`MATCH`]`#${catcherIndex} (${index}:${catcherIndex + index})`;
              LOG[key]["TRY"][getRuleName(option)]["CATCH"][`MATCH`]`#${index}`;
              LOG[key]["TRY"][getRuleName(option)][`MATCH`]`#${index}`;
              LOG[key]["MATCH"]`#${index}`;
              return result;
            }
          }

          LOG[key]["TRY"][getRuleName(option)]["CATCH"]["NONE"]`#${index}`;
          index++;
          continue;
        } else {
          LOG[key]["TRY"][getRuleName(option)]["MATCH"]`#${index}`;
          LOG[key]["MATCH"]`#${index}`;
          return result;
        }
      }

      LOG[key]["IGNORED"](() => firstIgnoredIndex !== undefined, `No match; returning first ignored: ${firstIgnoredIndex}`);
      LOG[key]["FAIL"](() => firstIgnoredIndex === undefined, `No match.`);
      return results[firstIgnoredIndex] ?? NONE({
        children: results,
      });
    }
  }
}

export default Choice;
