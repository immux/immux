import React, { ReactNode, useEffect, useState } from 'react';
import { Spin } from 'antd';

import { useStoreActions, useStoreState } from '@/store/hooks';
import styles from './styles.module.less';
import { catchError } from '@/utils';

function useAuthorized() {
  const [loading, setLoading] = useState(false);
  const fetchProfile = useStoreActions(
    (actions) => actions.account.fetchProfile
  );

  const authorize = async () => {
    setLoading(true);

    try {
      await fetchProfile();
    } catch (err) {
      catchError(err);
    } finally {
      setLoading(false);
    }
  };

  return [loading, authorize] as [boolean, () => Promise<void>];
}

export default function Authorized(props: { children: ReactNode }) {
  const logged = useStoreState((state) => state.account.logged);
  const [loading, authorize] = useAuthorized();

  useEffect(
    () => {
      if (!logged) {
        authorize();
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    []
  );

  return (
    <Spin className={styles.spin} size="large" spinning={loading}>
      {props.children}
    </Spin>
  );
}
