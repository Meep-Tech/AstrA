/**
 * Require at least one of the given properties in T.
 */
export type RequireAtLeastOne<
  T,
  Keys extends keyof T = keyof T
> =
  Pick<T, Exclude<keyof T, Keys>>
  & { [K in Keys]-?:
    Required<Pick<T, K>>
    & Partial<Pick<T, Exclude<Keys, K>>>
  }[Keys]


export function isTemplateStringsArray(value: any)
  : value is TemplateStringsArray {
  return value.hasOwnProperty("raw")
    && value.hasOwnProperty("length");
}

export function hasProp<T extends object, K extends PropertyKey>(
  obj: T,
  key: K
): obj is T & Record<K, unknown> {
  return obj.hasOwnProperty(key);
}

/**
 * Used to remove readonly from a type's properties.
 */
export type Writeable<
  T extends { [x: string]: any },
  K extends string = keyof T extends string ? keyof T : string
> = {
    [P in K]: T[P];
  }