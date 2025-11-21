import { Controller, Post, Body, BadRequestException } from '@nestjs/common';
import { ApiTags, ApiOperation, ApiBody, ApiResponse, ApiExtraModels, getSchemaPath } from '@nestjs/swagger';
import { FormShapeDto } from './dto/FormShapeDto';
// import { ZodFormShape } from './zod/form-shape.zod'; // If using Zod

@ApiTags('forms')
@ApiExtraModels(FormShapeDto)
@Controller('forms')
export class FormController {
  @Post('process')
  @ApiOperation({ summary: 'Process a FormShape workflow' })
  @ApiBody({ type: FormShapeDto, examples: {
    basic: {
      summary: 'Basic FormShape',
      value: {
        fields: [{ id: 'name', type: 'text', value: '', label: 'Name' }],
        meta: { workflow: 'basic' }
      }
    }
  }})
  @ApiResponse({ status: 200, description: 'Form processed successfully', schema: { $ref: getSchemaPath(FormShapeDto) } })
  @ApiResponse({ status: 400, description: 'Validation error' })
  async processForm(@Body() formShape: FormShapeDto) {
    // Example validation (replace with Zod/class-validator as needed)
    if (!formShape.fields || !Array.isArray(formShape.fields)) {
      throw new BadRequestException('Invalid form shape: fields must be an array');
    }

    // Workflow orchestration (call a service, process steps, etc.)
    // const result = await this.workflowService.run(formShape);

    // Event hooks (pre/post processing)
    // this.eventBus.emit('form:processed', result);

    // Custom response formatting
    return {
      status: 'processed',
      timestamp: Date.now(),
      form: formShape,
      // result,
    };
  }
}
