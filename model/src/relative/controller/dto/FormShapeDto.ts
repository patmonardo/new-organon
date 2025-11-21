// src/controller/dto/form-shape.dto.ts
import { ApiProperty } from '@nestjs/swagger';

export class FormShapeDto {
  @ApiProperty({ type: [Object] })
  fields: any[];

  @ApiProperty({ type: Object, required: false })
  meta?: any;
}
