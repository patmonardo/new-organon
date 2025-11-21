/**
 * View: The Middle Moment of the Model
 * ------------------------------------
 * In the dialectical architecture, a View is not just a display or rendering,
 * but the mediating moment between Model (particulars) and Controller (universal/principle).
 *
 * At the level of Logical Form, a View is akin to a Property (predicate, aspect, or feature).
 * At the level of Logical Model, a View is a representation or perspective on the Model—
 * a way of selecting, filtering, or presenting aspects of the Model's entities, properties, or subgraphs.
 *
 * This scaffold provides a generic View system, supporting:
 *   - Representations (named or typed views on a Model)
 *   - Perspectives (filters, projections, or lenses)
 *   - Property-based or topic-based views
 */

// --- Logical Layer Dependency ---
// These would be imported from the @organon/logic package
import type { Property } from '@organon/logic';
import type { Model, ModelEntity, ModelProperty } from './model';

// A View can be a named representation or projection of a Model
export interface View<T extends ModelEntity = ModelEntity> {
  id: string;
  name?: string;
  // The Model this view is based on
  model: Model<T>;
  // Properties or topics this view focuses on (can be logical or model properties)
  properties?: (ModelProperty | Property)[];
  // Optional: filter or selection criteria (predicate, query, etc.)
  filter?: (entity: T) => boolean;
  // Optional: projection or transformation (e.g., for tabular, graph, or visual views)
  project?: (entity: T) => any;
  // Optional: perspective or lens (e.g., user, agent, or context)
  perspective?: string;
  // Optional: metadata or schema info
  [key: string]: any;
}

// Example: TopicView for a Model (property-based view)
export interface TopicView<T extends ModelEntity = ModelEntity> extends View<T> {
  topic: string; // e.g., 'position', 'energy', 'status', etc.
}
