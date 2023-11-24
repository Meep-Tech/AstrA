import {
  Splayed
} from "../parser";

export namespace Assigner {
  export class Parser extends Splayed.Parser {
    get defaults() {
      return {
        name: 'assigner',
        types: ['operator']
      }
    }

    // rule: Rule<TokenData> = ({ cursor, token, UNEXPECTED }: Helpers): Results => {
    //   var result: Results;
    //   if (result = MutableFieldAssigner.parse(cursor, token)) {
    //     return result;
    //   } else {
    //     return UNEXPECTED({
    //       end: cursor,
    //       found: cursor.char,
    //       message: 'Expected an assignment operator.',
    //       expected: [MutableFieldAssigner]
    //     });
    //   }
    // }
  }

  export const parse
    = Parser.Parse;
}

export default Assigner;