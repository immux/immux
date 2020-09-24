import React, { useEffect } from 'react';
import { Spin } from 'antd';

import { useGitAuthorizeRedirector } from '@/services/hooks/account';
import { useStoreActions } from '@/store/hooks';
import { useHistory } from 'react-router-dom';
import styles from './styles.module.less';
import { useQueryParam } from '@/utils';
import _ from 'lodash';

export default function Login() {
  const resetProfile = useStoreActions(
    (actions) => actions.account.resetProfile
  );
  const resetAccessToken = useStoreActions(
    (actions) => actions.account.resetAccessToken
  );
  const login = useStoreActions((actions) => actions.account.login);

  const [redirectUri, redirectToGitAuthorize] = useGitAuthorizeRedirector();
  const redirect = useQueryParam<string>('redirect', '/');
  const code = useQueryParam<string>('code');
  const history = useHistory();

  useEffect(
    () => {
      (async function runEffectAsync() {
        resetProfile();
        resetAccessToken();

        if (!code) {
          redirectToGitAuthorize();
          return;
        }

        try {
          await login({ code, redirectUri });
          history.push(redirect);
        } catch (err) {
          const errMsg = _.get(err, ['response', 'data', 'errMsg']);

          if (errMsg === 'authcode check failure') {
            redirectToGitAuthorize();
          }
        }
      })();
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    []
  );

  return (
    <main className={styles.login}>
      <Spin size="large" />
    </main>
  );
}
