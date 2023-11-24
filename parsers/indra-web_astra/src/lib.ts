import './utils';
import * as Utils from './utils';
export { Utils };
export * from './utils';

import * as Parsers from './parser';
export { Parsers };
export * from './parser';

import * as Trees from './trees';
export { Trees };
export * from './trees';

import * as Tokens from './tokens';
export { Tokens };
export * from './tokens';

import * as Runtime from './runtime';
export { Runtime };
export * from './runtime';

import * as Nodes from './nodes';
export { Nodes };
export * from './nodes';

import * as Tests from './tests';
export { Tests };
export * from './tests';

import { Grammar } from './parser/grammar';
Grammar.Init();
export { Grammar };

export default {
  //...AstrA,
  ...Parsers,
  ...Trees,
  ...Tokens,
  ...Runtime,
  ...Nodes,
  ...Tests,
  ...Utils,
  Grammar,
  Parsers,
  Trees,
  Tokens,
  Runtime,
  Nodes,
  Tests,
  Utils,
}
