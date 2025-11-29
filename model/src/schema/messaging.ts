import { z } from 'zod';
import { FormShapeSchema } from './shape';

/**
 * Messaging Schema: Outlook-like Mail & Calendar
 * ----------------------------------------------
 * Represents emails, calendar events, tasks, and contacts.
 */

// --- Email / Message ---

export const EmailAddressSchema = z.object({
  name: z.string().optional(),
  email: z.string().email(),
});

export const MessageShapeSchema = FormShapeSchema.extend({
  type: z.literal('message').default('message'),
  subject: z.string(),
  body: z.string(), // HTML or Text
  from: EmailAddressSchema,
  to: z.array(EmailAddressSchema),
  cc: z.array(EmailAddressSchema).optional(),
  bcc: z.array(EmailAddressSchema).optional(),
  sentAt: z.date().or(z.string()).optional(),
  read: z.boolean().default(false),
  folder: z.string().default('inbox'), // inbox, sent, trash, etc.
  attachments: z.array(z.object({
    id: z.string(),
    name: z.string(),
    url: z.string(),
    size: z.number().optional(),
    type: z.string().optional(),
  })).optional(),
});

// --- Calendar / Event ---

export const EventStatusSchema = z.enum(['confirmed', 'tentative', 'cancelled']);

export const EventShapeSchema = FormShapeSchema.extend({
  type: z.literal('event').default('event'),
  title: z.string(),
  description: z.string().optional(),
  start: z.date().or(z.string()),
  end: z.date().or(z.string()),
  allDay: z.boolean().default(false),
  location: z.string().optional(),
  attendees: z.array(EmailAddressSchema).optional(),
  organizer: EmailAddressSchema.optional(),
  status: EventStatusSchema.default('confirmed'),
  recurrence: z.string().optional(), // RRule string
});

// --- Contact ---

export const ContactShapeSchema = FormShapeSchema.extend({
  type: z.literal('contact').default('contact'),
  firstName: z.string(),
  lastName: z.string().optional(),
  email: z.array(z.string().email()).optional(),
  phone: z.array(z.string()).optional(),
  company: z.string().optional(),
  jobTitle: z.string().optional(),
  avatar: z.string().optional(),
});

// Export types
export type EmailAddress = z.infer<typeof EmailAddressSchema>;
export type MessageShape = z.infer<typeof MessageShapeSchema>;
export type EventStatus = z.infer<typeof EventStatusSchema>;
export type EventShape = z.infer<typeof EventShapeSchema>;
export type ContactShape = z.infer<typeof ContactShapeSchema>;
