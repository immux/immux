import { useStoreActions, useStoreState } from '@/store/hooks';
import { useCallback, useEffect, useState } from 'react';
import { ResultError } from '@/utils/error';
import { catchError } from '@/utils';
import { message } from 'antd';

import _, { add } from 'lodash';

export function useFunctionCollection() {
  const [initialLoad, setInitialLoad] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<ResultError | null>(null);
  const [hasMore, setHasMore] = useState(true);
  const total = useStoreState((state) => state.functions.total);

  const functions = useStoreState(
    (state) => state.functions.personFunctions
  );
  
  const fetchPersonFunctions = useStoreActions(
    (actions) => actions.functions.fetchPersonFunctions
  );

  const clearFunctions = useStoreActions(
    (actions) => actions.functions.clear
  );

  const fetch = useCallback(
    async (pageNum = 1) => {
      try {
        setLoading(true);
        await fetchPersonFunctions({ pageNum, pageSize: 32 });
        setError(null);
        setInitialLoad(true);
      } catch (err) {
        catchError(err);
        setError(new ResultError(err));
      } finally {
        setLoading(false);
      }
    },
    [setLoading, fetchPersonFunctions, setError, setInitialLoad]
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

export function useAddMarket(id: string) {
  const [loading, setLoading] = useState(false);
  const addFunctionMarket = useStoreActions(
    (actions) => actions.functions.addFunctionMarket
  );

  const addMarket = useCallback(
    async () => {
      const hide = message.loading('adding', 0);

      try {
        setLoading(true);
        await addFunctionMarket({functionId: id});
        message.success('add market succuss!');
      } catch (err) {
        catchError(err);
      } finally {
        setLoading(false);
        hide();
      }
    },
    []
  );

  return [loading, addMarket] as [boolean, () => Promise<void>];
}
