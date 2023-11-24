import { Location } from "./location";
import { IParserType, IParserNamespace } from "./parse";
import { Error, Result, Match } from "./results";
import { DEBUG, getRuleName } from "./rules";
import { Parser } from "./parser";
import Logger from "../utils/logs";

export type Tests = {
  [name: string]: Test;
}

export type Test = ({
  input: string;
} | {
  inputs: string[];
}) & {
  expected: Expected;
}

export type Expected
  = IParserType
  | string
  | boolean
  | number
  | null
  | undefined
  | Partial<Error>
  | typeof Error
  | typeof globalThis.Error
  | globalThis.Error
  | {
    name?: string;
    types?: string[];
    start?: number | Partial<Location>;
    end?: number | Partial<Location>;
    children?: Expected[];
    properties?: {
      [name: string]: Expected;
    },
    elements?: {
      [name: string]: Expected;
      [index: number]: Expected;
    }
  }

const LOG
  = new Logger("TESTS")._$yellow;

export function test(type: IParserType, options?: {
  throwUnexpectedErrors?: boolean,
  limit?: number,
}) {
  const ruleName = getRuleName(type);

  LOG["PARSER"]._$green._$dim[ruleName]`START`;
  const parser: Parser = type instanceof Parser
    ? type
    : (type as any).Parser.Instance;
  var tests: Tests
    = (parser as any).tests;
  const results = {};

  if (!tests) {
    LOG["PARSER"][ruleName]`NO TESTS FOUND`;
    return results;
  }

  for (const testName in tests) {
    const test = tests[testName];
    const expected = test.expected;
    if (test.hasOwnProperty('inputs')) {
      const inputs = (test as { inputs: string[]; }).inputs;
      LOG["PARSER"][ruleName]["TEST"]._$yellow[testName]`START (With ${inputs.length} Input Variants)`;
      var index = 0;
      for (const input of inputs) {
        LOG["PARSER"][ruleName]["TEST"][testName][`${index}`]`START`;
        _testParse(testName, input, expected);
        LOG["PARSER"][ruleName]["TEST"][testName][`${index}`]`DONE`;
        index++;
      }
    } else {
      LOG["PARSER"][ruleName]["TEST"]._$yellow[testName]`START`;
      const { input } = test as { input: string };
      _testParse(testName, input, expected);
    }

    LOG["PARSER"][ruleName]["TEST"][testName]`DONE`;

    if (options?.limit !== undefined) {
      options.limit--;
      if (options.limit <= 0) {
        break;
      }
    }
  }

  return results;

  /** @internal */
  function _testParse(
    testName: string,
    input: string,
    expected: Expected,
    index?: number,
  ) {
    LOG["PARSER"][ruleName]["TEST"]._$yellow[testName][`${index}`](index !== undefined,
      "Input:\n\t┏" + ("\n" + input).toString().replace(/\n/g, '\n\t┃\t') + '\n\t┗');
    LOG["PARSER"][ruleName]["TEST"]._$yellow[testName](index === undefined,
      "Input:\n\t┏" + ("\n" + input).toString().replace(/\n/g, '\n\t┃\t') + '\n\t┗');
    const result = {
      success: false,
      input: input,
      expected: expected,
      actual: null as Result | globalThis.Error,
    };

    try {
      result.actual = parser.parse(input);
      LOG["PARSER"][ruleName]["TEST"]._$yellow[testName][`${index}`](index !== undefined,
        "Result:\n\t" + ("\n" + result.actual.toString()).replace(/\n/g, '\n\t>>\t') + '\n\t');
      LOG["PARSER"][ruleName]["TEST"]._$yellow[testName](index === undefined,
        "Result:\n\t" + ("\n" + result.actual.toString()).replace(/\n/g, '\n\t>>\t') + '\n\t');

      if (verifyTestResults(
        result.expected,
        result.actual
      )) {
        LOG["PARSER"][ruleName]["TEST"]._$green[testName][`${index}`](index !== undefined,
          "Success!");
        LOG["PARSER"][ruleName]["TEST"]._$green[testName](index === undefined,
          "Success!");
        result.success = true;
      } else {
        LOG["PARSER"][ruleName]["TEST"]._$red[testName][`${index}`](index !== undefined,
          "Failure!");
        LOG["PARSER"][ruleName]["TEST"]._$red[testName](index === undefined,
          "Failure!");
      }
    } catch (error) {
      result.actual = error;
      LOG["PARSER"][ruleName]["TEST"]._$yellow[testName][`${index}`](index !== undefined,
        "Threw:\n\t" + ("\n" + error.toString()).replace(/\n/g, '\n\t>>\t') + '\n\t');
      LOG["PARSER"][ruleName]["TEST"]._$yellow[testName](index === undefined,
        "Threw:\n\t" + ("\n" + error.toString()).replace(/\n/g, '\n\t>>\t') + '\n\t');

      if (expected instanceof error.constructor) {
        if (expected.hasOwnProperty('message')) {
          if ((expected as globalThis.Error).message === error.message) {
            LOG["PARSER"][ruleName]["TEST"]._$green[testName][`${index}`](index !== undefined,
              "Success!");
            LOG["PARSER"][ruleName]["TEST"]._$green[testName](index === undefined,
              "Success!");
            result.success = true;
          } else {
            LOG["PARSER"][ruleName]["TEST"]._$red[testName][`${index}`](index !== undefined,
              "Failure!");
            LOG["PARSER"][ruleName]["TEST"]._$red[testName](index === undefined,
              "Failure!");
          }
        } else {
          LOG["PARSER"][ruleName]["TEST"]._$green[testName][`${index}`](index !== undefined,
            "Success!");
          result.success = true;
        }
      } else if (options.throwUnexpectedErrors) {
        LOG["PARSER"][ruleName]["TEST"]._$red[testName][`${index}`](index !== undefined,
          "Failure!");
        LOG["PARSER"][ruleName]["TEST"]._$red[testName](index === undefined,
          "Failure!");
        throw error;
      } else {
        LOG["PARSER"][ruleName]["TEST"]._$red[testName][`${index}`](index !== undefined,
          "Failure!");
        LOG["PARSER"][ruleName]["TEST"]._$red[testName](index === undefined,
          "Failure!");
      }
    }

    console.log('\n');
  }
}

function verifyTestResults(
  expected: Expected,
  actual: unknown,
): boolean {
  if (expected instanceof Parser) {
    var expectedType = expected.defaults.name;
    var actualTypes = (actual as Match).types;

    return expectedType in actualTypes;
  } else if (expected instanceof Error) {
    return actual instanceof expected.constructor
      && (actual as Error).message === expected.message;
  } else if (expected instanceof globalThis.Error) {
    return actual instanceof expected.constructor
      && (actual as globalThis.Error).message === expected.message;
  } else if (expected instanceof Function) {
    if (expected.prototype instanceof Parser) {
      var expectedType = expected.prototype.defaults.name;
      var actualTypes = (actual as Match).types;

      return expectedType in actualTypes;
    } else if (expected.prototype instanceof Error) {
      return actual instanceof expected;
    } else if (expected.prototype instanceof globalThis.Error) {
      return actual instanceof expected;
    }
  } else if (expected instanceof Object) {
    if (expected.hasOwnProperty('Parser')) {
      var expectedType = (expected as IParserNamespace).Parser.Instance.name;
      var actualTypes = (actual as Match).types;

      return expectedType in actualTypes;
    } else {
      var obj: Exclude<Extract<Expected, object>, IParserNamespace | Function | globalThis.Error>
        = expected as any;

      if (obj.hasOwnProperty('name')) {
        if ((obj as any).name !== (actual as Match).name) {
          return false;
        }
      }

      if (obj.hasOwnProperty('types')) {
        for (const type of (obj as any).types) {
          if (!(type in (actual as Match).types)) {
            return false;
          }
        }
      }

      if (obj.hasOwnProperty('start')) {
        if ((obj as any).start !== (actual as Match).start) {
          return false;
        }
      }

      if (obj.hasOwnProperty('end')) {
        if ((obj as any).end !== (actual as Match).end) {
          return false;
        }
      }

      if (obj.children?.length) {
        if (!(actual as Expected)!.hasOwnProperty('children') || !(actual as any)!.children?.any()) {
          return false;
        }

        for (const index in obj.children) {
          var expectedChild = obj.children[index] as Expected;
          var actualChild = (actual as any).children[index];
          if (!verifyTestResults(expectedChild, actualChild)) {
            return false;
          }
        }
      }

      if (obj.hasOwnProperty('properties')) {
        if (!(actual as Expected)!.hasOwnProperty('properties')) {
          return false;
        }

        for (const name of Object.keys(obj.properties ?? {})) {
          var expectedProperty = obj.properties![name] as Expected;
          var actualProperty = (actual as any).properties[name];
          if (!verifyTestResults(expectedProperty, actualProperty)) {
            return false;
          }
        }
      }

      if (obj.hasOwnProperty('elements')) {
        var actualElements = (actual as any).elements;
        var expectedElements = obj.elements;

        if (!actualElements) {
          return false;
        }

        for (const index in Object.keys(expectedElements ?? {})) {
          var expectedElement = expectedElements![index] as Expected;
          var actualElement = actualElements[index];
          if (!verifyTestResults(expectedElement, actualElement)) {
            return false;
          }
        }
      }

      return true;
    }
  } else {
    return expected === actual;
  }
}
