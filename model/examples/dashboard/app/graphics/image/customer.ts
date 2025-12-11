//@graphics/cards/revenue.tsx
import type { OperationResult } from '@/data/schema/base';
import { ImageShapeSchema, type ImageShape } from '@graphics/schema/image';
import { Image } from './image';

export class CustomerImage extends Image<ImageShape> {
  constructor(private readonly config: Partial<ImageShape>) {
    super(config);
  }

  protected create(): OperationResult<ImageShape> {
    const result = ImageShapeSchema.safeParse({
      src: this.config.src || '/placeholder-customer.png',
      alt: this.config.alt || 'Customer image',
      size: {
        width: 100,
        height: 100,
        aspectRatio: '1/1'
      },
      fit: 'cover',
      position: 'center',
      loading: 'lazy',
      quality: 80
    });

    if (!result.success) {
      return {
        data: null,
        status: "error",
        message: "Invalid image configuration",
      };
    }

    return {
      data: result.data,
      status: "success",
      message: "Image configuration created successfully"
    };
  }

  protected edit(): OperationResult<ImageShape> {
    return this.edit();
  }

  render(): OperationResult<ImageShape> {
    return this.render();
  }
}
