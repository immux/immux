import { useStoreActions, useStoreState } from '@/store/hooks';
import { useCallback, useEffect, useState } from 'react';
import { ResultError } from '@/utils/error';
import { catchError } from '@/utils';

import _ from 'lodash';

export function useNameSpaceCollection() {
  const [initialLoad, setInitialLoad] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<ResultError | null>(null);
  const [hasMore, setHasMore] = useState(true);
  const total = useStoreState((state) => state.nameSpace.collection.total);

  const nameSpaces = useStoreState(
    (state) => state.nameSpace.collection.entries
  );

  const fetchNameSpaces = useStoreActions(
    (actions) => actions.nameSpace.collection.fetchNameSpaces
  );

  const clearNameSpaces = useStoreActions(
    (actions) => actions.nameSpace.collection.clear
  );

  const fetch = useCallback(
    async (pageNum = 1) => {
      try {
        setLoading(true);
        await fetchNameSpaces({ pageNum, pageSize: 32 });
        setError(null);
        setInitialLoad(true);
      } catch (err) {
        catchError(err);
        setError(new ResultError(err));
      } finally {
        setLoading(false);
      }
    },
    [setLoading, fetchNameSpaces, setError, setInitialLoad]
  );

  useEffect(
    () => {
      fetch();

      return () => clearNameSpaces();
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    []
  );

  useEffect(() => {
    if (initialLoad && _.size(nameSpaces) >= total) {
      setHasMore(false);
    }
  }, [initialLoad, nameSpaces, total]);

  return [loading, error, nameSpaces, hasMore, fetch] as const;
}
