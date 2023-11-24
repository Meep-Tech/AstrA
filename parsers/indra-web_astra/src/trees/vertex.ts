export type Vertex<
  TType extends IVertex,
  TChild extends IVertex = TType,
  TValue = unknown,
  TKey extends string | number | symbol = string,
  TParent = TChild
> = TType & {
  parent: TParent;
  children: TChild[];
  properties: {
    [key in TKey]: TValue;
  };
};

export interface IVertex extends Vertex<
  any, any, any, any, any
> { }

