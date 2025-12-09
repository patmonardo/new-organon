import React, { useState, useCallback } from 'react';
import { DisplayDocument, FormHandler } from '../../../sdsl/types';
import { RadixAdapter, RadixRenderContext } from './radix-adapter';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

export interface RadixControllerProps {
  document: DisplayDocument;
  data?: Record<string, unknown>;
  onAction?: (actionId: string, data?: unknown) => void;
  adapter?: RadixAdapter;
}

interface ControllerState {
  // Dialogs / Popovers / Dropdowns
  openStates: Record<string, boolean>;
  // Tabs / Accordions
  valueStates: Record<string, string | string[]>;
}

// ---------------------------------------------------------------------------
// Controller Component
// ---------------------------------------------------------------------------

export function RadixController({
  document,
  data = {},
  onAction,
  adapter = new RadixAdapter(),
}: RadixControllerProps) {
  // --- State Management ---
  const [state, setState] = useState<ControllerState>({
    openStates: {},
    valueStates: {},
  });

  // --- Handlers ---

  const handleOpenChange = useCallback((id: string, open: boolean) => {
    setState(prev => ({
      ...prev,
      openStates: { ...prev.openStates, [id]: open },
    }));
  }, []);

  const handleValueChange = useCallback((id: string, value: string | string[]) => {
    setState(prev => ({
      ...prev,
      valueStates: { ...prev.valueStates, [id]: value },
    }));
  }, []);

  // The "Brain" of the controller: Intercepts actions and updates state
  const handleAction = useCallback(
    (actionId: string, actionData?: unknown) => {
      // 1. Check for internal control actions (e.g., "open-dialog:settings")
      if (actionId.startsWith('radix:')) {
        const [_, command, targetId] = actionId.split(':');

        if (command === 'open') {
          handleOpenChange(targetId, true);
          return;
        }
        if (command === 'close') {
          handleOpenChange(targetId, false);
          return;
        }
        if (command === 'toggle') {
          setState(prev => ({
            ...prev,
            openStates: { ...prev.openStates, [targetId]: !prev.openStates[targetId] },
          }));
          return;
        }
        if (command === 'set-value') {
          // Expects actionData to contain { value: ... }
          const val = (actionData as any)?.value;
          if (val !== undefined) {
            handleValueChange(targetId, val);
          }
          return;
        }
      }

      // 2. Propagate to external handler
      onAction?.(actionId, actionData);
    },
    [handleOpenChange, handleValueChange, onAction]
  );

  // --- Context Construction ---

  // We inject the state and handlers into the context so components can read/write them
  const context: RadixRenderContext & { controller: any } = {
    data,
    handler: {
      onAction: handleAction,
    },
    // Expose raw state for components to read (e.g. is this dialog open?)
    // In a real implementation, we might want a more formal Context provider,
    // but passing it through the render context works for this architecture.
    controller: {
      state,
      setOpen: handleOpenChange,
      setValue: handleValueChange,
    },
  };

  // --- Render ---

  return (
    <div className="radix-controller-root">
      {adapter.render(document, context)}
    </div>
  );
}
