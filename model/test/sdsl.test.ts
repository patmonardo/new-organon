import { describe, it, expect, beforeEach, afterEach } from "vitest";
import {
  FormShape,
  FormModel,
  SimpleFormModel,
  FormView,
  SimpleFormView,
  FormController,
  SimpleFormController,
  createFormMVC,
  JSONAdapter,
  HTMLAdapter,
  defaultAdapters
} from '../src/sdsl';

// Factory for fresh test shape each time
function createTestShape(): FormShape {
  return {
    id: 'test-form',
    name: 'Test Form',
    title: 'Test Form Title',
    fields: [
      { id: 'name', type: 'text', label: 'Name', required: true, value: 'John' },
      { id: 'email', type: 'email', label: 'Email', required: true, value: 'john@example.com' },
      { id: 'age', type: 'number', label: 'Age', value: 30 },
    ]
  };
}

describe("MVC SDSL", () => {
  // Clear storage after each test to ensure isolation
  afterEach(() => {
    SimpleFormModel.clearStorage();
  });

  describe("FormModel", () => {
    let model: SimpleFormModel<FormShape>;

    beforeEach(() => {
      model = new SimpleFormModel(createTestShape());
    });

    it('holds form shape', () => {
      expect(model.shape.id).toBe('test-form');
      expect(model.shape.fields.length).toBe(3);
    });

    it('gets field value', () => {
      expect(model.getField('name')).toBe('John');
      expect(model.getField('age')).toBe(30);
    });

    it('sets field value and marks dirty', () => {
      expect(model.isDirty).toBe(false);
      model.setField('name', 'Jane');
      expect(model.getField('name')).toBe('Jane');
      expect(model.isDirty).toBe(true);
    });

    it('validates required fields', () => {
      model.setField('name', '');
      const result = model.validate();
      expect(result.status).toBe('error');
      expect(result.errors?.['name']).toBeDefined();
    });

    it('saves and loads', async () => {
      const saveResult = await model.save();
      expect(saveResult.status).toBe('success');

      const model2 = new SimpleFormModel({ ...createTestShape(), id: 'test-form' });
      const loadResult = await model2.load();
      expect(loadResult.status).toBe('success');
      expect(model2.getField('name')).toBe('John');
    });
  });

  describe("FormView", () => {
    let model: SimpleFormModel<FormShape>;
    let view: SimpleFormView<FormShape>;

    beforeEach(() => {
      model = new SimpleFormModel(createTestShape());
      view = new SimpleFormView(model, 'view');
    });

    it('renders DisplayDocument', () => {
      const doc = view.render();
      expect(doc.title).toBe('Test Form Title');
      expect(doc.layout.type).toBe('card'); // view mode uses card
      expect(doc.layout.children.length).toBe(3); // 3 fields
    });

    it('renders in edit mode with actions', () => {
      view.setMode('edit');
      const doc = view.render();
      expect(doc.layout.type).toBe('stack'); // edit mode uses stack
      expect(doc.layout.children.length).toBe(4); // 3 fields + actions
    });

    it('serializes to JSON', () => {
      const result = view.serialize('json');
      expect(result.status).toBe('success');
      expect(result.data).toHaveProperty('layout');
    });

    it('serializes to HTML', () => {
      const result = view.serialize('html');
      expect(result.status).toBe('success');
      expect(typeof result.data).toBe('string');
      expect(result.data).toContain('<html>');
    });
  });

  describe("FormController", () => {
    let controller: SimpleFormController<FormShape>;

    beforeEach(() => {
      controller = new SimpleFormController(createTestShape(), 'edit');
    });

    it('orchestrates model and view', () => {
      expect(controller.model).toBeDefined();
      expect(controller.view).toBeDefined();
      expect(controller.mode).toBe('edit');
    });

    it('displays document', () => {
      const doc = controller.display();
      expect(doc.title).toBe('Test Form Title');
      expect(doc.layout).toBeDefined();
    });

    it('creates handler', () => {
      const handler = controller.createHandler();
      expect(handler.onSubmit).toBeDefined();
      expect(handler.onCancel).toBeDefined();
      expect(handler.onChange).toBeDefined();
    });

    it('executes submit action', async () => {
      const result = await controller.executeAction('submit');
      expect(result.status).toBe('success');
    });

    it('validates before submit', async () => {
      controller.model.setField('name', '');
      const result = await controller.executeAction('submit');
      expect(result.status).toBe('error');
    });
  });

  describe("createFormMVC factory", () => {
    it('creates complete MVC stack', () => {
      const mvc = createFormMVC(createTestShape(), 'view');
      expect(mvc).toBeInstanceOf(SimpleFormController);
      expect(mvc.model).toBeInstanceOf(SimpleFormModel);
      expect(mvc.view).toBeInstanceOf(SimpleFormView);
    });
  });

  describe("Adapters", () => {
    let controller: SimpleFormController<FormShape>;

    beforeEach(() => {
      controller = new SimpleFormController(createTestShape(), 'view');
    });

    it('JSONAdapter transforms to object', () => {
      const adapter = new JSONAdapter();
      const output = adapter.adapt(controller);
      expect(output.content).toHaveProperty('type');
      expect(output.handler).toBeDefined();
    });

    it('HTMLAdapter transforms to string', () => {
      const adapter = new HTMLAdapter();
      const output = adapter.adapt(controller);
      expect(typeof output.content).toBe('string');
      expect(output.content).toContain('<div');
    });

    it('default registry has adapters', () => {
      expect(defaultAdapters.get('json')).toBeDefined();
      expect(defaultAdapters.get('html')).toBeDefined();
      expect(defaultAdapters.list()).toContain('json');
    });
  });
});
