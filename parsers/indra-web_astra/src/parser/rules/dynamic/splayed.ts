import { DEBUG, getRuleName } from '..';
import { IRule } from '../../rule';
import { Choice } from './choice';

export namespace Splayed {
  export abstract class Parser extends Choice.Parser {
    private static readonly _child_types: WeakMap<Function, IRule[]>
      = new WeakMap();

    get defaults() {
      return {
        name: "splayed"
      }
    }

    get isDynamic(): boolean {
      return this.constructor === Parser;
    }

    get options(): IRule[] {
      return Parser._child_types.get(this.constructor);
    }

    constructor() {
      super([]);
      DEBUG.LOG["GRAMMAR"]["INIT"]["TOKENS"][this.name]["SPLAYED"]`START`;
      var parent = Object.getPrototypeOf(this.constructor) as Function;
      var current = this.constructor as Function;
      if (parent === Splayed.Parser) {
        DEBUG.LOG["GRAMMAR"]["INIT"]["TOKENS"][this.name]["SPLAYED"]["BASE"](this.name);
        Parser._child_types.set(current, []);
      } else if (current !== Splayed.Parser) {
        var currentOptions = Parser._child_types.get(parent);
        if (!currentOptions) {
          currentOptions = [];
          DEBUG.LOG["GRAMMAR"]["INIT"]["TOKENS"][this.name]["SPLAYED"]["BASE"](getRuleName(parent as IRule));
        }

        DEBUG.LOG["GRAMMAR"]["INIT"]["TOKENS"][this.name]["SPLAYED"]["CHILD"]`${this.name} -splays-> ${getRuleName(parent as IRule)}`
        currentOptions.push(current as IRule);
        Parser._child_types.set(parent, currentOptions);
      }
    }
  }
}

export default Splayed;