import { Exception } from "./parser";
import { Location } from "./location";
import { Writeable } from "../utils";
import { Indents, IndentChar, LineIndents, IndentLevel, Indentation } from "./indents";
import { DEBUG } from ".";

export class Cursor implements Location, Number, Boolean {
  private _previous?: Cursor | null;
  private _next?: Cursor | null;
  private _offset?: number;

  readonly source: string;
  readonly index: number;
  readonly line: number;
  readonly column: number;
  readonly char: string;
  readonly indents: Indents;

  get location(): Location {
    return {
      index: this.index,
      line: this.line,
      column: this.column
    }
  }

  get current(): Cursor {
    return this.copy();
  }

  get previous(): Cursor | undefined {
    var previous = this.getPrevious();

    DEBUG.LOG["PARSER"]["CURSOR"]["MOVE"]["PREVIOUS"]`${this.char}(${this.index}) => ${previous?.char}(${previous?.index})`;
    return previous;
  }

  private getPrevious(): Cursor | undefined {
    return this._previous === null
      ? undefined
      : this._previous ?? (
        () => {
          throw new Exception('Previous cursor is missing! Do not start a new cursor in the middle of a source!');
        }
      )()
  }

  get next(): Cursor | undefined {
    var next = this.getNext();

    DEBUG.LOG["PARSER"]["CURSOR"]["MOVE"]["NEXT"]`${this.char}(${this.index}) => ${next?.char}(${next?.index})`;
    return next;
  }

  private getNext(): Cursor | undefined {
    return this._next === null
      ? undefined
      : (this._next ??= this._read());
  }

  get eof(): boolean {
    return this.index >= this.source.length;
  }

  private constructor(source: string) {
    this.source = source;
  }

  static Init(source: string) {
    var result = Object.assign(new Cursor(source), {
      _previous: null,
      index: 0,
      line: 0,
      column: 0,
      char: source[0],
      indents: {
        previous: [],
        current: [],
        isReading: true,
        alignsWithPrevious: true
      }
    }) as Cursor;

    DEBUG.LOG["PARSER"]["CURSOR"]._$green._$dim["INIT"]`${result.char} @ ${result.index} [${result.line}:${result.column}]`;
    DEBUG.LOG["PARSER"]["CURSOR"]["INIT"]["INDENTS"]["START"]`line: ${result.line}, level: 0`;
    DEBUG.LOG["PARSER"]["CURSOR"]["INIT"]["INDENTS"]["LEVEL"]["START"]`line: ${result.line}, level: 0`;

    return result;
  }

  copy() {
    return this._update();
  }

  at(index: number | Location): Cursor {
    index = typeof index === 'number'
      ? index
      : index.index;
    if (index < 0) throw new Exception('Index must be greater than or equal to 0.');

    if (index === this.index) {
      return this;
    }

    let cursor: Cursor | undefined = this;
    while (cursor.index < index) {
      cursor = cursor.getNext();
      if (!cursor) {
        throw new Exception('Could not find cursor for the requested index: ' + index + '.');
      }
    }

    while (cursor.index > index) {
      cursor = cursor.getPrevious();
      if (!cursor) {
        throw new Exception('Could not find cursor for the requested index: ' + index + '.');
      }
    }

    return cursor;
  }

  to(end: number | Location): string {
    end = typeof end === 'number'
      ? end
      : end.index;

    if (end < 0) throw new Exception('Index must be greater than or equal to 0.');
    if (end > this.source.length) throw new Exception('Index must be less than or equal to the length of the source.');

    return this.source.substring(
      this.index + this._offset ?? 0,
      end + this.at(end)._offset ?? this._offset ?? 0
    );

    // var result = '';
    // var cursor: Cursor | undefined = this;
    // if (end > this.index) {
    //   while ((cursor = cursor.next).index <= end) {
    //     result += cursor.char;
    //   }

    //   return result;
    // }
    // else if (end === this.index) {
    //   return cursor.char;
    // } else {
    //   try {
    //     while ((cursor = cursor.previous).index >= end) {
    //       result = cursor.char + result;
    //     }

    //     return result;
    //   } catch (error) {
    //     console.log("Could not find cursor for the requested index.", {
    //       cursor,
    //       index: end,
    //     });
    //   }
    // }
  }

  peek(): string | undefined;
  peek(count?: number): string | undefined;
  peek(until?: string): string | undefined;
  peek(until?: RegExp): string | undefined;
  peek(limit: number | RegExp | string = 1): string {
    if (typeof limit === 'number') {
      if (limit < 0) {
        if (this.index - limit < 0) return undefined;
        return this.source.slice(this.index - limit, this.index);
      }
      if (this.index + limit > this.source.length) return undefined;
      if (limit === 0) return '';

      return this.source.slice(this.index, this.index + limit);
    } else if (limit instanceof RegExp) {
      let match = limit.exec(this.source.slice(this.index));
      return match
        ? match[0]
        : undefined;
    } else {
      let matches = this.source.slice(this.index).match(limit);
      return matches
        ? matches[0]
        : undefined;
    }
  }

  valueOf(): boolean
  valueOf(): number
  valueOf(): string | boolean | number {
    if (this.eof) {
      return "";
    } else {
      return `${this.index}`
    }
  }

  toString(radix?: number): string {
    var toSerialize = { ...this } as any;
    delete toSerialize._previous;
    delete toSerialize._next;
    delete toSerialize._offset;
    delete toSerialize.source;

    return JSON.stringify(toSerialize);
  }

  toFixed(fractionDigits?: number): string {
    return this.index.toFixed(fractionDigits);
  }

  toExponential(fractionDigits?: number): string {
    return this.index.toExponential(fractionDigits);
  }

  toPrecision(precision?: number): string {
    return this.index.toPrecision(precision);
  }

  toLocaleString(locales?: string | string[], options?: Intl.NumberFormatOptions): string {
    return this.index.toLocaleString(locales, options);
  }

  private _read(): Cursor | null {
    DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["START"]`${this.char} @ ${this.index} [${this.line}:${this.column}]`;

    var offset = this._offset ?? 0;
    DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["OFFSET"]((() => !!offset), `${this.index} + ${offset} = ${this.index + offset}`);
    const nextIndex = this.index + offset + 1;
    if (nextIndex >= this.source.length) return null;

    const next: Writeable<Partial<Cursor>> & { offset: number } = {
      ...this,
      offset: offset,
      previous: this,
      char: this.source[nextIndex],
      index: nextIndex,
      column: this.column + 1,
    };

    if (next.char in ['\f', '\r']) {
      DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["OFFSET"]["INCREASED"](`skipped: '${next.char === '\f' ? "\\f" : "\\r"}', offset: ${next.offset} => ${next.offset + 1}, actual: ${this.index} + ${offset} = ${this.index + offset}`);
      next.offset++;
      next.char = this.source[next.index + next.offset];
    }

    DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["NEXT"]`${this.char}(${this.index}) => ${next.char}(${next.index})`;

    switch (next.char) {
      case '\n':
        _updateLine();
        break;
      case '\t':
      case ' ':
        if (next.indents.isReading) {
          _updateIndentation();
        }
        break;
      default:
        if (next.indents.isReading) {
          next.indents.isReading = false;
          DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["LEVEL"]["END"]`line: ${next.line}, level: ${next.indents.current.length}`;
          DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["END"]`line: ${next.line}, level: ${next.indents.current.length}`;
        }
        break;
    }

    const result = this._update(next);
    DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["DONE"]`${result.char} @ ${result.index} [${result.line}:${result.column}]`;
    return result;

    function _updateLine() {
      next.line++;
      next.column = 0;
      next.indents = { ...next.indents };
      next.indents.previous = [...next.indents.previous, next.indents];
      next.indents.current = [];
      next.indents.isReading = true;
      next.indents.alignsWithPrevious = true;
      DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["LINE"]["INCREASED"]`${next.line - 1} => ${next.line}`;
      DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["START"]`line: ${next.line}, level: 0`;
      DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["LEVEL"]["START"]`line: ${next.line}, level: 0`;
    }

    function _updateIndentation() {
      // init
      const indent = next.char as IndentChar;
      let currentLine: LineIndents<Indentation> = next.indents.current;
      let currentLevel: IndentLevel<Indentation> = currentLine[currentLine.length - 1];

      // copy
      next.indents = { ...next.indents };
      next.indents.current = [...next.indents.current];
      next.indents.current[next.indents.current.length - 1] = `${currentLevel}`;

      if (!next.indents.alignsWithPrevious) {
        // add to the the end of the current indent level
        _append();
      }

      let previousLine: LineIndents<IndentChar> = next.indents.previous[next.indents.previous.length - 1];
      // if the current line is longer than the previous line
      if (currentLine.length > previousLine.length) {
        // add to the the end of the current indent level
        _append();
      } // if the current line is the same length as the previous line
      else if (currentLine.length === previousLine.length) {
        let previousLevel = previousLine[previousLine.length - 1];
        // if the current indent level is longer than the previous indent level (shouldnt happen)
        if (currentLevel.length > previousLevel.length) {
          throw new Exception('Cannot have indented more than the previous line while also matching.');
        } // if the current indent level is shorter than the previous indent level (needs to be finished)
        else if (currentLevel.length < previousLevel.length) {
          const toMatch = previousLevel[currentLevel.length];
          if (toMatch !== indent) {
            // on mismatch; start a new indent level
            DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["MISMATCH"]`char: '${indent}', line: ${next.line}. level: ${currentLine.length}, indent: '${currentLevel}', previous: '${toMatch}'`;
            next.indents.alignsWithPrevious = false;
            DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["LEVEL"]["END"]`line: ${next.line}, level: ${currentLine.length}`;
            next.indents.current.push(indent);
            DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["LEVEL"]["START"]`line: ${next.line}, level: ${currentLine.length}`;
            DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["LEVEL"]["APPEND"]`char: '${indent}', line: ${next.line}. level: ${currentLine.length}, indent: '${currentLevel}'`;
          } else {
            // on match; add to the the end of the current indent level
            _append();
          }
        } // if the current indent level is the same length as the previous indent level (new indent level needs to be started)
        else {
          previousLevel = previousLine[currentLine.length + 1];
          const toMatch = previousLevel[currentLevel.length];

          // if no match, we don't align anymore
          if (toMatch !== indent) {
            DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["MISMATCH"]`char: '${indent}', line: ${next.line}. level: ${currentLine.length}, indent: '${currentLevel}', previous: '${toMatch}'`;
            next.indents.alignsWithPrevious = false;
          }

          // always start a new indent level
          DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["LEVEL"]["END"]`line: ${next.line}, level: ${currentLine.length}`;
          next.indents.current.push(indent);
          DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["LEVEL"]["START"]`line: ${next.line}, level: ${currentLine.length}`;
          DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["LEVEL"]["APPEND"]`char: '${indent}', line: ${next.line}. level: ${currentLine.length}, indent: '${currentLevel}'`;
        }
      } // if the current line is shorter than the previous line (needs to be finished)
      else {
        let previousLevel = previousLine[currentLine.length];
        // if the previous indent level is longer (current needs to be finished)
        if (previousLevel.length > currentLevel.length) {
          let toMatch = previousLevel[currentLevel.length];
          if (toMatch !== indent) {
            next.indents.alignsWithPrevious = false;
            DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["MISMATCH"]`char: '${indent}', line: ${next.line}. level: ${currentLine.length}, indent: '${currentLevel}', previous: '${toMatch}'`;
            DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["LEVEL"]["END"]`line: ${next.line}, level: ${currentLine.length}`;
            next.indents.current.push(indent);
            DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["LEVEL"]["START"]`line: ${next.line}, level: ${currentLine.length}`;
            DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["LEVEL"]["APPEND"]`char: '${indent}', line: ${next.line}. level: ${currentLine.length}, indent: '${currentLevel}'`;
          } else {
            _append();
          }
        } // if the previous indent level is shorter (shouldn't happen)
        else if (previousLevel.length < currentLevel.length) {
          throw new Exception('Cannot have indented more than the previous line while also matching.');
        } // if the previous indent level is the same length (new indent level needs to be started)
        else {
          previousLevel = previousLine[currentLine.length + 1];
          const toMatch = previousLevel[currentLevel.length];

          // if no match, we don't align anymore
          if (toMatch !== indent) {
            DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["MISMATCH"]`char: '${indent}', line: ${next.line}. level: ${currentLine.length}, indent: '${currentLevel}', previous: '${toMatch}'`;
            next.indents.alignsWithPrevious = false;
          }

          // always start a new indent level
          DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["LEVEL"]["END"]`line: ${next.line}, level: ${currentLine.length}`;
          next.indents.current.push(indent);
          DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["LEVEL"]["START"]`line: ${next.line}, level: ${currentLine.length}`;
          DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["LEVEL"]["APPEND"]`char: '${indent}', line: ${next.line}. level: ${currentLine.length}, indent: '${currentLevel}'`;
        }
      }

      function _append() {
        currentLevel
          = currentLine[currentLine.length - 1]
          = (`${currentLevel}${indent} ` as IndentLevel);
        DEBUG.LOG["PARSER"]["CURSOR"]["READ"]["INDENTS"]["LEVEL"]["APPEND"]`char: '${indent}', line: ${next.line}. level: ${currentLine.length}, indent: '${currentLevel}'`;
      }
    }
  }

  private _update(props: Partial<Writeable<Cursor & { previous: Cursor; next: Cursor; }>> = {}): Cursor {
    (props as any)._previous = props.previous;
    (props as any)._next = props.next;
    delete props.previous;
    delete props.next;
    delete props.source;

    var updated = new Cursor(this.source);
    Object.assign(updated, this, props);

    return updated;
  }
}
