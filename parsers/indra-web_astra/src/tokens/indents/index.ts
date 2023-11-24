import { IncreaseIndent } from './increase';
export { IncreaseIndent };

import { DecreaseIndent } from './decrease';
export { DecreaseIndent };

import { CurrentIndent } from './current';
export { CurrentIndent };

import { Indent as IndentParser } from './indent';
export const Indent
  = Object.assign(
    IndentParser,
    {
      Increase: IncreaseIndent,
      Decrease: DecreaseIndent,
      Current: CurrentIndent
    });
export default Indent;