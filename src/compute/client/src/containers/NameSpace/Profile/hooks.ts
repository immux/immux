import { ResultError } from '@/utils/error';

import { useStoreActions, useStoreState } from '@/store/hooks';
import { useParams } from 'react-router-dom';
import { useEffect, useState } from 'react';
import { catchError } from '@/utils';

export function useNameSpaceId() {
  const { nameSpaceId } = useParams<{ nameSpaceId: string }>();
  return nameSpaceId;
}

export function useNameSpace() {
  const nameSpaceId = useNameSpaceId();
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<ResultError | null>(null);
  const nameSpace = useStoreState((state) => state.nameSpace.entry);

  const fetchNameSpace = useStoreActions(
    (actions) => actions.nameSpace.fetchNameSpace
  );

  const clearNameSpace = useStoreActions(
    (actions) => actions.nameSpace.clearNameSpace
  );

  useEffect(
    () => {
      (async function runEffectAsync() {
        if (nameSpaceId) {
          try {
            setLoading(true);
            await fetchNameSpace({ nameSpaceId });
            setError(null);
          } catch (err) {
            catchError(err);
            setError(new ResultError(err));
          } finally {
            setLoading(false);
          }
        }
      })();

      return () => clearNameSpace();
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    []
  );

  return [nameSpace, loading, error] as const;
}

export function useProjectId() {
  const { nameSpaceId } = useParams<{ nameSpaceId: string }>();
  return nameSpaceId;
}

export function useProjectFolders() {
  const projectId = useProjectId();
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<ResultError | null>(null);
  const projectFolders = useStoreState((state) => state.project.folders);

  const fetchProjectFolders = useStoreActions(
    (actions) => actions.project.fetchProjectFolders
  );

  const clearProjectFolders = useStoreActions(
    (actions) => actions.project.clearProjectFolders
  );

  useEffect(
    () => {
      (async function runEffectAsync() {
        if (projectId) {
          try {
            setLoading(true);
            await fetchProjectFolders({ projectId });
            setError(null);
          } catch (err) {
            catchError(err);
            setError(new ResultError(err));
          } finally {
            setLoading(false);
          }
        }
      })();

      return () => clearProjectFolders();
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    []
  );

  return [projectFolders, loading, error] as const;
}
