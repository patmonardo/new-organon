import { ReactController } from '../../src/sdsl/react-controller';

type ControllerConstructor = new (mode?: any, initialData?: any) => ReactController<any>;

export class ControllerRegistry {
  private static routes: Map<string, ControllerConstructor> = new Map();

  static register(path: string, controller: ControllerConstructor) {
    this.routes.set(path, controller);
  }

  static get(path: string): ControllerConstructor | undefined {
    return this.routes.get(path);
  }
}
