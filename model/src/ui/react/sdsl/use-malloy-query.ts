import { useState, useCallback, useMemo } from 'react';
import { DataModel, ViewQuery, ViewSpec, DataView } from '../../../data/sdsl';
import { SemanticDataService, SemanticResult } from '../../../execution/semantic-hydrator';

export interface UseMalloyQueryResult {
  query: ViewQuery;
  updateQuery: (updates: Partial<ViewQuery>) => void;
  runQuery: () => Promise<SemanticResult | null>;
  reset: () => void;
  view: DataView; // The underlying DataView object
  sqlPlan: string;
  result: SemanticResult | null;
  isExecuting: boolean;
  error: Error | null;
}

export function useMalloyQuery(
  source: DataModel,
  initialQuery: ViewQuery = {},
  options?: { service?: SemanticDataService } // Use Service abstraction
): UseMalloyQueryResult {
  const [query, setQuery] = useState<ViewQuery>(initialQuery);
  const [result, setResult] = useState<SemanticResult | null>(null);
  const [isExecuting, setIsExecuting] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const updateQuery = useCallback((updates: Partial<ViewQuery>) => {
    setQuery((prev) => ({ ...prev, ...updates }));
    setResult(null); // Clear previous result on change
  }, []);

  const reset = useCallback(() => {
    setQuery(initialQuery);
    setResult(null);
    setError(null);
  }, [initialQuery]);

  const view = useMemo(() => {
    return source.view(query);
  }, [source, query]);

  const sqlPlan = useMemo(() => {
    return view.toPlan();
  }, [view]);

  const runQuery = useCallback(async () => {
    if (!options?.service) {
      console.warn('No semantic service provided to useMalloyQuery');
      return null;
    }

    setIsExecuting(true);
    setError(null);
    try {
      const res = await options.service.execute(view);
      setResult(res);
      return res;
    } catch (err) {
      console.error('Query execution failed:', err);
      setError(err instanceof Error ? err : new Error(String(err)));
      return null;
    } finally {
      setIsExecuting(false);
    }
  }, [view, options?.service]);

  return {
    query,
    updateQuery,
    runQuery,
    reset,
    view,
    sqlPlan,
    result,
    isExecuting,
    error,
  };
}
