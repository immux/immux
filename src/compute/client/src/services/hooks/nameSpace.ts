import { FormInstance } from 'antd/es/form';

import { useStoreActions } from '@/store/hooks';
import { useParams } from 'react-router-dom';
import { useEffect, useState } from 'react';
import { catchError } from '@/utils';

/**
 * get editing namespace
 * @param shouldFetch
 * @param form
 */
export function useFetchEditingNameSpace(
  shouldFetch: boolean = true,
  form?: FormInstance
) {
  const { id: nameSpaceId = '' } = useParams();
  const [loading, setLoading] = useState(false);
  const fetchEditingNameSpace = useStoreActions(
    (actions) => actions.nameSpace.fetchEditingNameSpace
  );

  useEffect(() => {
    (async function runAsyncEffect() {
      if (!shouldFetch) {
        return;
      }

      setLoading(true);

      try {
        const nameSpace = await fetchEditingNameSpace({ nameSpaceId });
        if (form)
          form.setFieldsValue({
            name: nameSpace?.name,
            title: nameSpace?.title,
            description: nameSpace?.description
          });
      } catch (err) {
        catchError(err);
      } finally {
        setLoading(false);
      }
    })();
  }, [fetchEditingNameSpace, form, shouldFetch, nameSpaceId]);

  return [loading];
}
