import {
  Rule,
  TokenParser,
  TokenData
} from "../parser";

/**
 * A simple Text-Based Entry Key
 */
export namespace Name {
  export const INLINE_WHITESPACE_PATTERN
    = /[ \t]/;

  export const GLOBAL_CHAR_PATTERN
    = /[a-zA-Z@$]/;

  export const NON_ASCII_CHAR_PATTERN: RegExp
    = /[^\x00-\x7F]/;

  export const NUMERIC_CHAR_PATTERN
    = /[0-9]/

  export const MIDDLE_ONLY_LONE_CHAR_PATTERN
    = /[\+\-\*\%\^\~]/;

  export const MIDDLE_ONLY_REPEATABLE_CHAR_PATTERN
    = /_/;

  export class Parser extends TokenParser {
    get defaults() {
      return {
        name: 'name',
        types: ['key', 'identifier']
      }
    }

    rule: Rule<TokenData> = ({ cursor, LOG, UNEXPECTED, MATCH, FAIL }) => {
      var isNumeric: boolean = false;
      var firstLetter = cursor.char;
      if (firstLetter.match(NON_ASCII_CHAR_PATTERN.or(GLOBAL_CHAR_PATTERN))) {
        LOG["READ"]`${firstLetter}`
      } else if (firstLetter.match(NUMERIC_CHAR_PATTERN)) {
        isNumeric = true;
        LOG["READ"]`${firstLetter} (numeric)`
      } else {
        return UNEXPECTED({
          end: cursor,
          found: firstLetter,
          message: 'Names cannot begin with certain symbols.',
          expected: [
            NON_ASCII_CHAR_PATTERN,
            GLOBAL_CHAR_PATTERN,
            NUMERIC_CHAR_PATTERN
          ],
        });
      }

      var lastLoneChar: string | undefined;
      var inTailingWhitespace: boolean = false;
      while (cursor = cursor.next) {
        if (cursor.char.match(INLINE_WHITESPACE_PATTERN)) {
          inTailingWhitespace = true;
          LOG["READ"]`${cursor.char} (tailing-whitespace)`
          lastLoneChar = undefined;
          continue;
        } else if (inTailingWhitespace) {
          return MATCH(cursor.previous!);
        }

        if (cursor.char.match(
          NON_ASCII_CHAR_PATTERN
            .or(GLOBAL_CHAR_PATTERN)
            .or(MIDDLE_ONLY_REPEATABLE_CHAR_PATTERN)
        )) {
          isNumeric = false;
          LOG["READ"]`${cursor.char} ${cursor.char.match(MIDDLE_ONLY_REPEATABLE_CHAR_PATTERN) ? '(middle-only)' : ''}`
        } else if (cursor.char.match(MIDDLE_ONLY_LONE_CHAR_PATTERN)) {
          if (isNumeric) return FAIL({
            end: cursor.location,
            message: 'Names that begin with numeric characters cannot have math symbols following them.'
          });

          if (lastLoneChar === cursor.char) return UNEXPECTED({
            end: cursor.location,
            found: cursor.char,
            expected: [
              NON_ASCII_CHAR_PATTERN,
              GLOBAL_CHAR_PATTERN,
              NUMERIC_CHAR_PATTERN,
              MIDDLE_ONLY_REPEATABLE_CHAR_PATTERN,
              new RegExp(Array.from(MIDDLE_ONLY_LONE_CHAR_PATTERN.source)
                .filter((char) => char !== cursor.char)
                .join('')),
            ],
            message: `The characters: ${MIDDLE_ONLY_LONE_CHAR_PATTERN.source} cannot be repeated in a row in a name.`,
          });

          // only one with explict continue: to avoid clearing the lastLoneChar
          lastLoneChar = cursor.char;
          LOG["READ"]`${cursor.char} (lone-middle-only)`
          continue;
        }
        else if (!cursor.char.match(NUMERIC_CHAR_PATTERN)) {
          return MATCH(cursor.previous!);
        } else {
          LOG["READ"]`${cursor.char} (numeric)`
        }

        lastLoneChar = undefined;
        continue;
      }

      return MATCH(cursor);
    };
  }

  export const parse
    = Parser.Parse;
}

export default Name;
