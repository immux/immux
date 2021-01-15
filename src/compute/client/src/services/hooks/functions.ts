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
export function useFetchEditingFunction(
  shouldFetch: boolean = true,
  form?: FormInstance
) {
  const { id: functionId = '' } = useParams();
  const [loading, setLoading] = useState(false);
  const fetchEditingFunction = useStoreActions(
    (actions) => actions.functions.fetchEditingFunction
  );

  useEffect(() => {
    (async function runAsyncEffect() {
      if (!shouldFetch) {
        return;
      }

      setLoading(true);

      try {
        const editFunction = await fetchEditingFunction({ functionId });
        if (form)
          form.setFieldsValue({
            price: editFunction?.price,
            title: editFunction?.title,
            description: editFunction?.description
          });
      } catch (err) {
        catchError(err);
      } finally {
        setLoading(false);
      }
    })();
  }, [fetchEditingFunction, form, shouldFetch, functionId]);

  return [loading];
}
