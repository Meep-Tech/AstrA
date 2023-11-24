declare global {
  interface RegExp {
    or(...args: RegExp[]): RegExp;
    and(...args: RegExp[]): RegExp;
  }
}

Object.defineProperty(RegExp.prototype, 'or', {
  value: function or(...args: RegExp[]): RegExp {
    return new RegExp(
      [this, ...args]
        .map((r) => r.source)
        .reduce((a, b) =>
          a.length === 1 && b.length === 1
            ? `[${a}${b}]`
            : a.startsWith('[')
              && a[1] !== '^'
              && a.endsWith(']')
              && b.startsWith('[')
              && b[1] !== '^'
              && b.endsWith(']')
              ? `[${a.slice(0, -1)}${b.slice(1)}]`
              : `${a}|${b}`)
    ) as unknown as RegExp;
  }
});

Object.defineProperty(RegExp.prototype, 'then', {
  value: function then(...args: RegExp[]): RegExp {
    return new RegExp(
      [this, ...args]
        .map((r) => r.source)
        .reduce((a, b) => `${a}${b}`)
    ) as unknown as RegExp;
  }
});