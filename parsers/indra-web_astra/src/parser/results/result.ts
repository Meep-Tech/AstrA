import { Vertex } from "../../trees/vertex";
import { Location } from "../location";

export type IgnoredResultParams
  = 'isError' | 'isEmpty' | 'position' | 'valueOf';

type RecusiveData = {
  start: `${number}@[${number}:${number}]`;
  end: `${number}@[${number}:${number}]`;
  parent: string;
  children: (string | RecusiveData | SimpleData)[] | [];
  properties: {
    [key: string]: (string | RecusiveData | SimpleData)[];
  } | {};
};

type SimpleData = {
  start: string;
  end: string;
  parent: string;
  children: string[];
  properties: {
    [key: string]: string;
  };
};

export abstract class Result implements
  Boolean,
  Vertex<
    Result,
    Result,
    unknown,
    string | symbol
  >
{
  private _order: (Result | string | symbol)[] = [];
  abstract isSuccess: boolean;
  abstract isError: boolean;
  start: Location;
  end: Location;
  name?: string;
  types: string[] = [];

  get isEmpty(): boolean {
    return this.start.index === this.end.index;
  }

  get position() {
    return {
      start: this.start,
      end: this.end
    };
  }

  parent: Result;
  children: Result[];
  properties: { [key: string | symbol]: unknown; };
  get elements(): {
    [index: string | symbol]: unknown;
    [index: number]: Result;
    [Symbol.iterator]: () => IterableIterator<unknown>;
  } {
    var index = 0;
    var arr = []
    var els = {
      [Symbol.iterator]: () => arr[Symbol.iterator]()
    }

    for (var el of this._order) {
      if (typeof el === 'string' || typeof el === 'symbol') {
        els[el] = this.properties[el];
        els[index] = this.properties[el];
        arr.push(this.properties[el]);
      } else {
        els[index] = el;
        arr.push(el);
      }

      index++;
    }

    return els;
  }

  constructor(init: { end: Location } & Partial<Omit<Result, IgnoredResultParams>>) {
    var args = {
      ...init,
      children: this._childrenProxyConstructor(init),
      properties: new Proxy<{ [key: string | symbol]: unknown; }>(init.properties ?? {}, {
        set: (target, prop, value, receiver) => {
          if (!target.hasOwnProperty(prop)) {
            this._order.push(prop);
          }

          return target[prop] = value;
        },
        deleteProperty: (target, prop) => {
          this._order = this._order.filter(v => v !== prop);

          return delete target[prop];
        }
      }),
    }

    Object.assign(this, args);
  }

  valueOf(): boolean {
    return !this.isError;
  }

  toString(options?: { recusive: true }): string {
    return this.constructor.name + ": " + JSON.stringify(
      this.toData({ recusive: options?.recusive }),
      null,
      2
    )
  }

  toData(): SimpleData;
  toData(options: { recusive: true }): RecusiveData;
  toData(options?: { recusive: true }): RecusiveData | SimpleData {
    var data = {
      start: `${this.start?.index} [${this.start?.line}:${this.start?.column}]`,
      end: `${this.end?.index} [${this.end?.line}:${this.end?.column}]`,
    } as any;

    if (this.isError) {
      data.isError = true;
    }

    if (this.isSuccess) {
      data.isSuccess = true;
    }

    if (this.name) {
      data.name = this.name;
    }

    if (this.types?.length > 0) {
      data.types = this.types;
    }

    if (this.parent) {
      data.parent = this.parent.hasOwnProperty('name')
        ? `${this.parent.name}`
        : `{${this.parent.constructor.name}}`;
    }

    if (this.children?.length > 0) {
      data.children = this.children.map(
        c => options?.recusive
          ? c.toData(options)
          : c.constructor.name
      );
    }

    var properties = {};
    var hasProps = false;
    for (var key in this.properties) {
      hasProps = true;
      properties[key] = this.properties[key] instanceof Result
        ? (this.properties[key] as Result).toData(options)
        : (this.properties[key] instanceof Object
          ? `{${this.properties[key].constructor.name}}`
          : `${this.properties[key]}`);
    }

    if (hasProps) {
      data.properties = properties;
    }

    return data as RecusiveData | SimpleData;
  }


  private _childrenProxyConstructor(
    init: { end: Location; }
      & Partial<Omit<Result, IgnoredResultParams>>
  ) {
    return new Proxy<Result[]>(init.children ?? [], {
      get: (target, prop, receiver) => {
        if (prop === 'push') {
          return (value: Result) => {
            this._order.push(value);
            return target.push(value);
          };
        } else if (prop === 'pop') {
          return () => {
            var result = target.pop();
            this._order = this._order.filter(v => v !== result);
          };
        } else if (prop === 'shift') {
          return () => {
            var result = target.shift();
            this._order = this._order.filter(v => v !== result);
          };
        } else if (prop === 'unshift') {
          return (value: Result) => {
            this._order.unshift(value);
            return target.unshift(value);
          };
        } else if (prop === 'splice') {
          return (start: number, deleteCount?: number, ...items: Result[]) => {
            var results = target.splice(start, deleteCount, ...items);
            this._order.splice(start, deleteCount, ...items);
            return results;
          };
        } else if (prop === 'sort') {
          return (compareFn?: (a: Result, b: Result) => number) => {
            target.sort(compareFn);
            this._order.sort((a, b) => {
              if (a instanceof Result && b instanceof Result) {
                return compareFn(a, b);
              } else {
                return 0;
              }
            });
            return target;
          };
        } else if (prop === 'reverse') {
          return () => {
            target.reverse();
            this._order.reverse();
            return target;
          };
        }

        return target[prop];
      },
      set: (target, prop, value, receiver) => {
        if (prop === 'length') {
          var toTrim = this.children.length - value as number;
          while (toTrim > 0) { // remove last child from the order
            var lastChild = this.children[this.children.length - 1];
            this._order = this._order.filter(v => v !== lastChild);
            toTrim--;
          }
        }

        return target[prop] = value;
      },
      deleteProperty: (target, prop) => {
        if (prop === 'length') {
          this._order = this._order.filter(v => v !== this.children[this.children.length - 1]);
        }

        return delete target[prop];
      }
    });
  }

}

export default Result;
export {
  Result as ParseResult
}