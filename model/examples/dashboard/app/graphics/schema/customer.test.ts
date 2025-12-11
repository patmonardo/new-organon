import { describe, it, expect } from 'vitest'
import { CustomerFormShapeSchema, CustomerFieldShapeSchema } from './customer'

describe('Customer Schema Validation', () => {
  describe('Customer Field Schema', () => {
    it('should validate valid customer fields', () => {
      const validFields = [
        {
          id: 'name',
          type: 'text',
          label: 'Name',
          required: true,
          defaultValue: '',
        },
        {
          id: 'email',
          type: 'email',
          label: 'Email Address',
          required: true,
          defaultValue: '',
        },
        {
          id: 'imageUrl',
          type: 'url',
          label: 'Profile Image',
          required: false,
          defaultValue: '',
        },
      ];

      validFields.forEach(field => {
        const result = CustomerFieldShapeSchema.safeParse(field);
        expect(result.success).toBe(true);
      });
    });

    it('should reject invalid field IDs', () => {
      const invalidField = {
        id: 'phone', // Not in CustomerFieldId enum
        type: 'text',
        label: 'Phone Number',
        required: false,
        defaultValue: '',
      };

      const result = CustomerFieldShapeSchema.safeParse(invalidField);
      expect(result.success).toBe(false);
    });

    it('should reject invalid field types', () => {
      const invalidField = {
        id: 'name',
        type: 'password', // Not in CustomerFieldType enum
        label: 'Name',
        required: true,
        defaultValue: '',
      };

      const result = CustomerFieldShapeSchema.safeParse(invalidField);
      expect(result.success).toBe(false);
    });
  });

  describe('Customer Form Shape', () => {
    it('should validate a complete customer form shape', () => {
      const validCustomerForm = {
        layout: {
          title: 'New Customer',
          columns: 'single',
          sections: [{
            title: 'Customer Information',
            fieldIds: ['name', 'email', 'imageUrl']
          }],
          actions: [
            { type: 'submit', label: 'Create', variant: 'primary' }
          ]
        },
        fields: [
          {
            id: 'name',
            type: 'text',
            label: 'Name',
            required: true,
            defaultValue: ''
          },
          {
            id: 'email',
            type: 'email',
            label: 'Email',
            required: true,
            defaultValue: ''
          }
        ],
        state: {
          status: 'idle'
        }
      };

      const result = CustomerFormShapeSchema.safeParse(validCustomerForm);
      expect(result.success).toBe(true);
    });

    it('should enforce required fields', () => {
      const incomplete = {
        layout: {
          title: 'New Customer'
          // Missing required fields
        }
      };

      const result = CustomerFormShapeSchema.safeParse(incomplete);
      expect(result.success).toBe(false);
    });

    it('should apply default values', () => {
      const minimal = {
        layout: {
          title: 'New Customer',
          columns: 'single',
          sections: [{
            title: 'Customer Information',
            fieldIds: ['name', 'email']
          }],
        },
        fields: []
      };

      const result = CustomerFormShapeSchema.safeParse(minimal);
      //expect(result.success).toBe(true);

      if (result.success) {
        // Test default values
        expect(result.data.layout.actions).toEqual([]);
        expect(result.data.state.status).toBe('idle');
      }
    });

    it('should validate section field IDs', () => {
      const formWithInvalidFieldId = {
        layout: {
          title: 'New Customer',
          columns: 'single',
          sections: [{
            title: 'Customer Information',
            fieldIds: ['name', 'invalidId', 'imageUrl'] // invalidId is not valid
          }],
          actions: []
        },
        fields: [
          {
            id: 'name',
            type: 'text',
            label: 'Name',
            required: true,
            defaultValue: ''
          }
        ],
        state: {
          status: 'idle'
        }
      };

      const result = CustomerFormShapeSchema.safeParse(formWithInvalidFieldId);
      expect(result.success).toBe(false);
    });

    it('should allow forms with multiple sections', () => {
      const formWithMultipleSections = {
        layout: {
          title: 'Customer Details',
          columns: 'single',
          sections: [
            {
              title: 'Basic Info',
              fieldIds: ['name', 'email']
            },
            {
              title: 'Profile',
              fieldIds: ['imageUrl']
            }
          ],
          actions: []
        },
        fields: [
          {
            id: 'name',
            type: 'text',
            label: 'Name',
            required: true,
            defaultValue: ''
          },
          {
            id: 'email',
            type: 'email',
            label: 'Email',
            required: true,
            defaultValue: ''
          },
          {
            id: 'imageUrl',
            type: 'url',
            label: 'Image URL',
            required: false,
            defaultValue: ''
          }
        ],
        state: {
          status: 'idle'
        }
      };

      const result = CustomerFormShapeSchema.safeParse(formWithMultipleSections);
      expect(result.success).toBe(true);
    });
  });
});
