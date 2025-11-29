declare module 'duckdb' {
  export default class Database {
    constructor(path: string, callback?: (err: Error | null) => void);
    connect(): Connection;
  }

  export class Connection {
    all(sql: string, callback: (err: Error | null, rows: any[]) => void): void;
    run(sql: string, callback?: (err: Error | null) => void): void;
    close(callback?: (err: Error | null) => void): void;
  }
}
