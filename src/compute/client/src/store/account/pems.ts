import { AccountPemsStoreModal } from '@/types/store/account';

import {
  createAccountPems,
  destroyAccountPems,
  fetchCurrentAccountPems
} from '@/services/api/account';
import { action, computed, thunk } from 'easy-peasy';
import moment from 'moment';
import _ from 'lodash';

export const pems: AccountPemsStoreModal = {
  hash: '',
  publicPem: '',
  createAt: moment(0),

  hasPems: computed(
    state => !!(state.hash && state.publicPem && state.createAt)
  ),

  hashLabel: computed(state => {
    return _.join(
      _.map(_.chunk(state.hash, 2), chunk => _.join(chunk, '')),
      ':'
    );
  }),

  setPems: action((state, pems) => {
    state.hash = pems.hash;
    state.publicPem = pems.publicPem;
    state.createAt = moment(pems.createAt);
  }),

  resetPems: action(state => {
    state.hash = '';
    state.publicPem = '';
    state.createAt = moment(0);
  }),

  fetchPems: thunk(async actions => {
    const { hash, publicPem, createAt } = await fetchCurrentAccountPems();

    actions.setPems({ hash, publicPem, createAt });

    return { hash, publicPem, createAt };
  }),

  createPems: thunk(async actions => {
    const { hash, publicPem, createAt } = await createAccountPems();

    actions.setPems({ hash, publicPem, createAt });

    return { hash, publicPem, createAt };
  }),

  destroyPems: thunk(async actions => {
    await destroyAccountPems();
    actions.resetPems();
  })
};
