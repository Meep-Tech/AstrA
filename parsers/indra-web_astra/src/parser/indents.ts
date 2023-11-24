export type Indents<T extends string = Indentation> = LineIndents<T> & {
  isReading: boolean;
  current: LineIndents<T>;
  previous: LineIndents<T>[];
  alignsWithPrevious: boolean;
};

export type Space
  = ' ';

export type Tab
  = '\t';

export type IndentChar
  = Space
  | Tab;

export type Indentation
  = Space
  | Tab
  | `${IndentChar}${IndentChar}`
  | `${IndentChar}${IndentChar}${IndentChar}`
  | `${IndentChar}${IndentChar}${IndentChar}`;

export type IndentLevel<T extends string = Indentation>
  = Indentation |
  (T extends IndentChar
    ? T
    : (T extends `${IndentChar}${infer R}`
      ? IndentLevel<R>
      : never));

export type LineIndents<T extends string = Indentation>
  = IndentLevel<T>[];

export default Indents;