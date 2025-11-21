/**
 * Model: The First Moment of Science
 * ----------------------------------
 * In the dialectical architecture, a Model is not merely a passive data set,
 * but the field of particularities or 'immediate beings'—the substrate for all demonstration.
 * It is the necessary ground for any science (e.g., Kinematics in Mechanics),
 * representing the collection of elements, their properties, and (optionally) their relations.
 *
 * This scaffold provides a generic Model system, supporting:
 *   - Elements/particulars (of type T)
 *   - Sets/collections of elements
 *   - Optional graph-like relations between elements
 */


// --- Logical Layer Dependency ---
// These would be imported from the @organon/logic package
import type { Entity, Property, Relation } from '@organon/logic';

// ModelEntity: Extends the logical Entity with model-specific fields
export interface ModelEntity extends Entity {
  // Model-specific or science-specific properties can be added here
}

// ModelProperty: Extends the logical Property (can be customized per model)
export interface ModelProperty extends Property {
  // Model-specific property fields
}

// ModelRelation: Extends the logical Relation (for graphs, networks, etc.)
export interface ModelRelation extends Relation {
  // Model-specific relation fields
}

// Optional: Relation between elements (for graphs, networks, etc.)
export interface ModelRelation {
  source: string; // id of source element
  target: string; // id of target element
  type?: string;  // e.g., 'connected', 'adjacent', etc.
  [key: string]: any;
}


// The generic Model interface
export interface Model<T extends ModelEntity = ModelEntity> {
  entities: T[];
  // Optional: properties and relations
  properties?: ModelProperty[];
  relations?: ModelRelation[];
  // Optional: metadata or schema info
  [key: string]: any;
}


// Example: KinematicsEntity for Mechanics (can be extended)
export interface KinematicsEntity extends ModelEntity {
  position: [number, number, number]; // 3D position
  velocity?: [number, number, number];
  mass?: number;
}

export interface KinematicsModel extends Model<KinematicsEntity> {
  // Additional kinematic-specific properties can be added here
}
