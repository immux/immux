import { Action, Computed, Thunk } from 'easy-peasy';
import { Account } from '@/types/models';
import { Moment } from 'moment';
import { StoreModel } from '@/types/store/index';

export interface AccountPemsStoreModal {
  hash: string;

  publicPem: string;

  createAt: Moment;

  hasPems: Computed<AccountPemsStoreModal, boolean>;

  hashLabel: Computed<AccountPemsStoreModal, string>;

  setPems: Action<
    AccountPemsStoreModal,
    { hash: string; publicPem: string; createAt: Moment | Date | number }
  >;

  resetPems: Action<AccountPemsStoreModal>;

  fetchPems: Thunk<
    AccountPemsStoreModal,
    void,
    any,
    StoreModel,
    Promise<{ hash: string; publicPem: string; createAt: number }>
  >;

  createPems: Thunk<
    AccountPemsStoreModal,
    void,
    any,
    StoreModel,
    Promise<{ hash: string; publicPem: string; createAt: number }>
  >;

  destroyPems: Thunk<AccountPemsStoreModal>;
}

export interface AccountStoreModel {
  pems: AccountPemsStoreModal;

  profile: Account;
  accessToken: string;
  expiresIn: Moment;

  logged: Computed<AccountStoreModel, boolean>;

  setProfile: Action<AccountStoreModel, Account>;

  setAccessToken: Action<
    AccountStoreModel,
    { accessToken: string; expiresIn: Moment }
  >;

  resetProfile: Action<AccountStoreModel>;

  resetAccessToken: Action<AccountStoreModel>;

  fetchProfile: Thunk<AccountStoreModel>;

  login: Thunk<AccountStoreModel, { code: string; redirectUri: string }>;

  logout: Thunk<AccountStoreModel>;
}
