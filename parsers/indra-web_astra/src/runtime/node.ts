export class Node {
  private _values: Node[] = [];

  private _entries: Map<PropertyKey, number> = new Map();

  has(key: PropertyKey): boolean {
    return this._entries.has(key);
  }

  get(key: PropertyKey): Node {
    return this._values[this._entries.get(key)];
  }

  set(key: PropertyKey, value: Node): void {
    var index: number;
    if (typeof key === 'number') {
      index = key;
      this._values.splice(index, 0, value);
    } else {
      index = this._values.length;
      this._values.push(value);
    }

    this._entries.set(key, index);
  }

  *keys(): IterableIterator<PropertyKey> {
    for (var i in this._values) {
      const v = this._values[i];
      if (v !== undefined) {
        yield i;
      }
    }

    for (var i in this._entries) {
      if (typeof i !== 'number') {
        yield i;
      }
    }
  }

  values(): IterableIterator<Node> {
    return this._values.values();
  }
}
