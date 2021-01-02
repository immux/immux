import { useStoreActions, useStoreState } from '@/store/hooks';
import { useCallback, useEffect, useState } from 'react';
import { ResultError } from '@/utils/error';
import { catchError } from '@/utils';

import _ from 'lodash';

export function useMarketFnCollection() {
  const [initialLoad, setInitialLoad] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<ResultError | null>(null);
  const [hasMore, setHasMore] = useState(true);
  const total = useStoreState((state) => state.functions.publicTotal);

  const functions = useStoreState(
    (state) => state.functions.publicFunctions
  );
  
  const fetchPublicFunctions = useStoreActions(
    (actions) => actions.functions.fetchPublicFunctions
  );

  const clearFunctions = useStoreActions(
    (actions) => actions.functions.clear
  );

  const fetch = useCallback(
    async (pageNum = 1) => {
      try {
        setLoading(true);
        await fetchPublicFunctions({ pageNum, pageSize: 32 });
        setError(null);
        setInitialLoad(true);
      } catch (err) {
        catchError(err);
        setError(new ResultError(err));
      } finally {
        setLoading(false);
      }
    },
    [setLoading, fetchPublicFunctions, setError, setInitialLoad]
  );

  useEffect(
    () => {
      fetch();

      return () => clearFunctions();
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    []
  );

  useEffect(() => {
    if (initialLoad && _.size(functions) >= total) {
      setHasMore(false);
    }
  }, [initialLoad, functions, total]);

  return [loading, error, functions, hasMore, fetch] as const;
}
