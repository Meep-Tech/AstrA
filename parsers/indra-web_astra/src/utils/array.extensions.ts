declare global {
  interface Array<T> {
    first(): T | undefined;
    last(): T | undefined;
    any(): boolean;
    exclude(...args: T[]): T[];
  }
}

Object.defineProperty(Array.prototype, 'first', {
  value: function first<T>(): T | undefined {
    return this[0];
  }
});

Object.defineProperty(RegExp.prototype, 'last', {
  value: function last<T>(): T | undefined {
    return this[this.length - 1];
  }
});

Object.defineProperty(Array.prototype, 'any', {
  value: function any<T>(): boolean {
    return this.length > 0;
  }
});

Object.defineProperty(Array.prototype, 'exclude', {
  value: function exclude<T>(...args: T[]): T[] {
    return this.filter((x: T) => !args.includes(x));
  }
});