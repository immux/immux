import { ActionCreator } from 'easy-peasy';

import { useStoreActions, useStoreState } from '@/store/hooks';
import { catchError, useQuery } from '@/utils';
import { useCallback, useState } from 'react';
import { useHistory } from 'react-router-dom';
import { message } from 'antd';
import _ from 'lodash';
import qs from 'qs';

/**
 * Git redirector hook
 */
export function useGitAuthorizeRedirector(): [string, () => void] {
  const query = useQuery<{ code?: string; redirect?: string }>();
  const queryString = qs.stringify(_.pick(query, ['redirect']));
  const search = queryString ? `?${queryString}` : '';

  // eslint-disable-next-line no-restricted-globals
  const redirect = `${location.origin}${location.pathname}${search}`;

  const redirectToGitAuthorize = () => {
    const search = qs.stringify({
      client_id: process.env.REACT_APP_GIT_APP_ID,
      response_type: 'code',
      redirect_uri: redirect
    });
    console.log(
      `${process.env.REACT_APP_GIT_ORIGIN}/oauth/authorize?${search}`
    );
    // eslint-disable-next-line no-restricted-globals
    location.href = `${process.env.REACT_APP_GIT_ORIGIN}/oauth/authorize?${search}`;
  };

  return [redirect, redirectToGitAuthorize];
}

/**
 * get account profile
 */
export function useAccountProfile(): [string, string, string] {
  const name = useStoreState((state) => state.account.profile.name);
  const email = useStoreState((state) => state.account.profile.email);
  const avatar = useStoreState((state) => state.account.profile.avatar);

  return [name, email, avatar];
}

export function useAccountAvatar() {
  return useStoreState((state) => state.account.profile.avatar);
}

export function useAccountProfileFetcher(): [boolean, () => Promise<void>] {
  const [loading, setLoading] = useState(false);
  const fetchProfile = useStoreActions(
    (actions) => actions.account.fetchProfile
  );
  const fetcher = useCallback(async () => {
    setLoading(true);

    try {
      await fetchProfile();
    } catch (err) {
      catchError(err);
    } finally {
      setLoading(false);
    }
  }, [setLoading, fetchProfile]);

  return [loading, fetcher];
}

export function useAccountPemsFetcher(): [
  boolean,
  () => Promise<void>,
  ActionCreator<void>
] {
  const [loading, setLoading] = useState(true);
  const fetchPems = useStoreActions(
    (actions) => actions.account.pems.fetchPems
  );
  const resetPems = useStoreActions(
    (actions) => actions.account.pems.resetPems
  );

  const fetcher = useCallback(async () => {
    try {
      setLoading(true);
      await fetchPems();
    } catch (err) {
      catchError(err);
    } finally {
      setLoading(false);
    }
  }, [setLoading, fetchPems]);

  return [loading, fetcher, resetPems];
}

export function useAccountLogoutAction(): [boolean, () => Promise<void>] {
  const [loading, setLoading] = useState(false);
  const logout = useStoreActions((actions) => actions.account.logout);
  const history = useHistory();

  const onLogout = useCallback(async () => {
    if (loading) {
      return;
    }

    const hide = message.loading('Logging out...', 0);

    try {
      setLoading(true);
      await logout();
      history.push('/account/login');
    } catch (err) {
      catchError(err);
    } finally {
      setLoading(false);
      hide();
    }
  }, [loading, history, logout]);

  return [loading, onLogout];
}

export function useCreateAccountPems() {
  const [loading, setLoading] = useState(false);
  const createPems = useStoreActions(
    (actions) => actions.account.pems.createPems
  );

  const create = useCallback(async () => {
    try {
      setLoading(true);
      await createPems();
    } catch (err) {
      catchError(err);
    } finally {
      setLoading(false);
    }
  }, [setLoading, createPems]);

  return [loading, create] as [boolean, () => Promise<void>];
}

export function useDestroyAccountPems() {
  const [loading, setLoading] = useState(false);
  const destroyPems = useStoreActions(
    (actions) => actions.account.pems.destroyPems
  );

  const destroy = useCallback(async () => {
    try {
      setLoading(true);
      await destroyPems();
    } catch (err) {
      catchError(err);
    } finally {
      setLoading(false);
    }
  }, [setLoading, destroyPems]);

  return [loading, destroy] as [boolean, () => Promise<void>];
}
