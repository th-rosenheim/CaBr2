import { environment } from '../../../environments/environment';

export enum LogLevel {
  TRACE = 'TRACE',
  DEBUG = 'DEBUG',
  INFO = 'INFO',
  WARNING = 'WARNING',
  ERROR = 'ERROR',
}

const createLog = () => {
  if (environment.web) {
    return (level: LogLevel, path: string, ...messages: any[]) => {
      let fn;
      let prefix;
      switch (level) {
        case LogLevel.TRACE:
          fn = console.log;
          prefix = '[TRACE]';
          break;

        case LogLevel.DEBUG:
          fn = console.log;
          prefix = '[DEBUG]';
          break;

        case LogLevel.INFO:
          fn = console.log;
          prefix = '[INFO]';
          break;

        case LogLevel.WARNING:
          fn = console.warn;
          prefix = '[WARNING]';
          break;

        case LogLevel.ERROR:
          fn = console.error;
          prefix = '[ERROR]';
          break;
      }

      fn(prefix, path, ...messages);
    };
  }
  else {
    let invoke = async (cmd: string, args: any): Promise<any> => {
      // defer actual logging until module finished loading
      setTimeout(() => {
        invoke(cmd, args).catch((err) => console.error(err));
      }, 1);
    };
    import('@tauri-apps/api/tauri').then((tauri) => invoke = tauri.invoke);

    return (level: LogLevel, path: string, ...messages: any[]) => {
      invoke('plugin:cabr2_logger|log', {
        level,
        path,
        messages,
      })
        .catch((err) => console.error(err));
    };
  }
};

const log = createLog();

/**
 * This class provides methods that call the logging functions in the backend.
 * It is supposed to be used if you want to log something into the log file,
 * for development purposes use the methods of `console.*`.
 */
export default class Logger {
  path: string;

  /**
   * Creates new logger instance for a specified path or purpose.
   *
   * @param path filepath or other identifier to find source of log.
   */
  constructor(path: string) {
    this.path = path;
  }

  /**
   * Logs all arguments space separated with the level `TRACE`.
   */
  trace(...messages: any[]): void {
    log(LogLevel.TRACE, this.path, ...messages);
  }

  /**
   * Logs all arguments space separated with the level `DEBUG`.
   */
  debug(...messages: any[]): void {
    log(LogLevel.DEBUG, this.path, ...messages);
  }

  /**
   * Logs all arguments space separated with the level `INFO`.
   */
  info(...messages: any[]): void {
    log(LogLevel.INFO, this.path, ...messages);
  }

  /**
   * Logs all arguments space separated with the level `WARNING`.
   */
  warning(...messages: any[]): void {
    log(LogLevel.WARNING, this.path, ...messages);
  }

  /**
   * Logs all arguments space separated with the level `ERROR`.
   */
  error(...messages: any[]): void {
    log(LogLevel.ERROR, this.path, ...messages);
  }
}
