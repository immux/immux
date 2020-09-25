import { AccountStoreModel } from '@/types/store/account';

import { getItem, setItem, removeItem } from '@/utils/storage';
import { action, computed, thunk } from 'easy-peasy';
import { useStoreState } from '@/store/hooks';
import { pems } from './pems';
import moment from 'moment';
import _ from 'lodash';

import { STORAGE_ACCESS_TOKEN } from '@/constants';
import {
  fetchCurrentAccountProfile,
  login,
  logout
} from '@/services/api/account';

const storageToken = getItem<{ accessToken: string; expiresIn: number }>(
  STORAGE_ACCESS_TOKEN
);

/**
 * get `accessToken` state hook
 */
export function useAccessTokenState(): string | null {
  const accessToken = useStoreState((state) => state.account.accessToken);
  const expiresIn = useStoreState((state) => state.account.expiresIn);

  // prettier-ignore
  return (
    !accessToken || !expiresIn || expiresIn.isBefore(Date.now()) ?
      null :
      accessToken
  );
}

export const account: AccountStoreModel = {
  pems: pems,

  profile: {
    id: '',
    name: '',
    email: '',
    avatar: '',
    gender: 0,
    createAt: moment(0)
  },

  accessToken: _.get(storageToken, ['accessToken'], ''),
  expiresIn: moment(_.get(storageToken, ['expiresIn'], 0)),

  logged: computed((state) => !!state.profile.email),

  setProfile: action((state, account) => {
    _.assign(state.profile, account);
  }),

  setAccessToken: action((state, { accessToken, expiresIn }) => {
    state.accessToken = accessToken;
    state.expiresIn = expiresIn;

    setItem(STORAGE_ACCESS_TOKEN, { accessToken, expiresIn: +expiresIn });
  }),

  resetProfile: action((state) => {
    state.profile.id = '';
    state.profile.name = '';
    state.profile.email = '';
    state.profile.avatar = '';
    state.profile.gender = 0;
    state.profile.createAt = moment(0);
  }),

  resetAccessToken: action((state) => {
    state.accessToken = '';
    state.expiresIn = moment(0);

    removeItem(STORAGE_ACCESS_TOKEN);
  }),

  fetchProfile: thunk(async (actions) => {
    actions.setProfile(await fetchCurrentAccountProfile());
  }),

  login: thunk(async (actions, { code, redirectUri }) => {
    const { accessToken, expiresIn, account } = await login(code, redirectUri);

    actions.setAccessToken({ accessToken, expiresIn: moment(expiresIn) });
    //@ts-ignore
    actions.setProfile(account);
  }),

  logout: thunk(() => logout())
};
