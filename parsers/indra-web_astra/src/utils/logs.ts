import { isTemplateStringsArray } from ".";
import styles from "console-log-colors";

type StyleFN
  = typeof styles.red;

type StyleKey
  = keyof StyleFN;

type Logger
  = LogWriter
  & {
    isEnabled: boolean;
    isVerbose: boolean;
    //$: { [style in keyof typeof styles]: Styles & LogWriter }
  }
  & LoggerConstructor
  & Styles
  & Titler

export interface ConditionalLogger {
  (
    condition: ((...args: any[]) => boolean) | boolean,
    ...args: Parameters<typeof console.log>
  ): ReturnType<typeof console.log>;
};

type KeyStyle
  = { [$style in `_$${StyleKey}`]: Logger; };

type MessageStyle
  = { [$style in `$${StyleKey}`]: Logger; };

type Styles
  = KeyStyle
  & MessageStyle;

type Titler
  = { [key: string]: Logger; };

type LoggerConstructor = {
  new(): Logger;
  new(...keys: string[]): Logger;
};

type LogWriter
  = ConditionalLogger
  & typeof console.log;

const _LOG_KEYS: string[] = [];
const _LOG_VERBOSE
  = process.argv.includes('--verbose')
  || process.argv.includes('-v');
const _LOG_ENABLED
  = _LOG_VERBOSE
  || process.argv.includes('--debug')
  || process.argv.includes('-d');
var _LOG_STYLE: StyleFN = styles.reset;
const _KEY_STYLES: {
  [key: string]: StyleFN
} = {};

const Logger = function Logger(
  ...keys: string[] | []
): Logger {
  return new Proxy(
    _LOG_ENABLED
      ? (keys.length > 0
        ? (...args: any[]) => {
          _LOG_KEYS.unshift(...keys);
          _log(...args)
        }
        : _log)
      : () => { },
    {
      get: (_, propKey, proxy) => {
        if (!_LOG_ENABLED) {
          return proxy;
        }

        if (propKey === 'isEnabled') {
          return true;
        }

        if (propKey === 'isVerbose') {
          return _LOG_VERBOSE;
        }

        if (typeof propKey === 'string') {
          if (propKey.startsWith('_$')) {
            if (propKey === '_$') {
              throw new Error("'_$' Styler for Logger not yet implemented")
            } else {
              var lastKey = (_LOG_KEYS[_LOG_KEYS.length - 1] ?? keys[keys.length - 1]);
              if (_KEY_STYLES[lastKey]) {
                _KEY_STYLES[lastKey] = _KEY_STYLES[lastKey][propKey.slice(2)];
              } else {
                _KEY_STYLES[lastKey] = styles[propKey.slice(2)];
              }
            }
          } else if (propKey.startsWith('$')) {
            if (propKey === '$') {
              throw new Error("'$' Styler for Logger not yet implemented")
            } else {
              _LOG_STYLE = _LOG_STYLE[propKey.slice(1)];
            }
          } else {
            _LOG_KEYS.push(propKey);
          }
        }

        return proxy;
      }
    }) as Logger;
} as unknown as LoggerConstructor;

const _log = (...args: any[]) => {
  if (args.length === 0) {
    return new Logger();
  }

  if (typeof args[0] === 'function') {
    if (!args[0]()) {
      _LOG_KEYS.length = 0;
      return;
    } else {
      args = args.slice(1);
    }
  } else if (typeof args[0] === 'boolean') {
    if (!args[0]) {
      _LOG_KEYS.length = 0;
      return;
    } else {
      args = args.slice(1);
    }
  } else if (isTemplateStringsArray(args[0])) {
    args = [String.raw({ raw: args[0] }, ...args.slice(1))];
  }

  if (_LOG_KEYS.length || _LOG_STYLE !== styles.reset) {
    var firstString = args.findIndex(arg => typeof arg === 'string');
    if (firstString > -1) {
      if (_LOG_KEYS.length) {
        args[firstString] = _LOG_STYLE(`[${_LOG_KEYS.map(
          k => `${_KEY_STYLES[k] ? _KEY_STYLES[k](k) : k}`
        ).join('][')}]: ${args[firstString]}`);
      } else {
        args[firstString] = _LOG_STYLE(args[firstString]);
      }
    }
  }

  console.log(...args);
  _LOG_KEYS.length = 0;
  _LOG_STYLE = styles.reset;
};

export { Logger };
export default Logger;