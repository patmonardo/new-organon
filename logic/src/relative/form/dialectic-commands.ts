import type { DialecticState, Moment, Force, Invariant } from '@schema';

// --- Commands ---

export type DialecticStateTransitionCmd = {
  kind: 'dialectic.state.transition';
  payload: {
    fromStateId: string;
    toStateId: string;
    dialecticState: DialecticState;
  };
  meta?: Record<string, unknown>;
};

export type DialecticMomentActivateCmd = {
  kind: 'dialectic.moment.activate';
  payload: {
    stateId: string;
    moment: Moment;
  };
  meta?: Record<string, unknown>;
};

export type DialecticForceApplyCmd = {
  kind: 'dialectic.force.apply';
  payload: {
    stateId: string;
    force: Force;
  };
  meta?: Record<string, unknown>;
};

export type DialecticInvariantCheckCmd = {
  kind: 'dialectic.invariant.check';
  payload: {
    stateId: string;
    invariants: Invariant[];
  };
  meta?: Record<string, unknown>;
};

export type DialecticEvaluateCmd = {
  kind: 'dialectic.evaluate';
  payload: {
    dialecticState: DialecticState;
    context?: any;
  };
  meta?: Record<string, unknown>;
};

export type DialecticCommand =
  | DialecticStateTransitionCmd
  | DialecticMomentActivateCmd
  | DialecticForceApplyCmd
  | DialecticInvariantCheckCmd
  | DialecticEvaluateCmd;

// --- Events ---

export type DialecticStateTransitionedEvent = {
  kind: 'dialectic.state.transitioned';
  payload: {
    fromState: string;
    toState: string;
    mechanism?: string;
  };
  meta?: Record<string, unknown>;
};

export type DialecticMomentActivatedEvent = {
  kind: 'dialectic.moment.activated';
  payload: {
    stateId: string;
    moment: string;
  };
  meta?: Record<string, unknown>;
};

export type DialecticForceAppliedEvent = {
  kind: 'dialectic.force.applied';
  payload: {
    stateId: string;
    force: string;
    effect: string;
    targetState?: string;
  };
  meta?: Record<string, unknown>;
};

export type DialecticInvariantViolatedEvent = {
  kind: 'dialectic.invariant.violated';
  payload: {
    stateId: string;
    invariant: string;
    reason: string;
  };
  meta?: Record<string, unknown>;
};

export type DialecticInvariantSatisfiedEvent = {
  kind: 'dialectic.invariant.satisfied';
  payload: {
    stateId: string;
    count: number;
  };
  meta?: Record<string, unknown>;
};

export type DialecticEvaluatedEvent = {
  kind: 'dialectic.evaluated';
  payload: {
    stateId: string;
    concept: string;
    phase: string;
  };
  meta?: Record<string, unknown>;
};

export type DialecticEvent =
  | DialecticStateTransitionedEvent
  | DialecticMomentActivatedEvent
  | DialecticForceAppliedEvent
  | DialecticInvariantViolatedEvent
  | DialecticInvariantSatisfiedEvent
  | DialecticEvaluatedEvent;
