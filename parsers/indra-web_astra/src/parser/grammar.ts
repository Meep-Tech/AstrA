import {
  Assigner,
  CurrentIndent,
  DecreaseIndent,
  IncreaseIndent,
  Indent,
  MutableFieldAssigner,
  Name,
  Text,
  Number,
  NamedEntry,
  Value
} from "../tokens";
import { ParserNamespace } from "./parse";
import { DEBUG } from "./rules";

export abstract class Grammar {
  public static readonly Root: ParserNamespace;
  public static readonly Tokens = [
    Indent,
    CurrentIndent,
    DecreaseIndent,
    IncreaseIndent,
    Name,
    NamedEntry,
    Value,
    Number,
    Text,
    Assigner,
    MutableFieldAssigner,
  ]

  public static Init() {
    DEBUG.LOG["GRAMMAR"]._$blueBright[`INIT`]`START`;
    DEBUG.LOG["GRAMMAR"]["INIT"]["TOKENS"]`START`;

    for (const type of this.Tokens) {
      try {
        type.Parser.Instance;
      } catch (error) {
        DEBUG.LOG["GRAMMAR"]["INIT"]["TOKENS"]["ERROR"](error.toString(), { error });
      }
    }

    DEBUG.LOG["GRAMMAR"]["INIT"]["TOKENS"]`DONE`;
    DEBUG.LOG["GRAMMAR"]["INIT"]`DONE`
    console.log("\n");
  }
}