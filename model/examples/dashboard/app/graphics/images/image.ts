import type { OperationResult } from '@schema/base';
import { ImageShape } from '@graphics/schema/image'

export abstract class Image<T extends ImageShape> {

  constructor(protected readonly data?: unknown) { }

  protected abstract create(): OperationResult<T>
  protected abstract edit(): OperationResult<T>
  protected abstract render(): OperationResult<T>
}
