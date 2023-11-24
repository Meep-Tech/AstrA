import {
  Rule,
  NOT_IMPLEMENTED,
  tryCatch,
  Splayed
} from "../parser";

export namespace Value {
  export class Parser extends Splayed.Parser {
    get defaults() {
      return {
        name: 'value'
      }
    }
  }

  export const parse
    = Parser.Parse;
}

export default Value;

export namespace Number {
  export class Parser extends Value.Parser {
    get defaults() {
      return {
        name: 'number'
      }
    }

    rule: Rule
      = NOT_IMPLEMENTED;
  }

  export const parse
    = Parser.Parse;
}

export namespace Text {
  export class Parser extends Value.Parser {
    get defaults() {
      return {
        name: 'text'
      }
    }

    rule: Rule
      = NOT_IMPLEMENTED;

    readonly catchFailedNumber
      = tryCatch(
        Number,
        NOT_IMPLEMENTED
      );
  }

  export const parse
    = Parser.Parse;
}