import { FunctionsStoreModel, FunctionInfo } from '@/types/store/functions';

import { action, thunk } from 'easy-peasy';

import { fetchPersonFunctions, fetchPublicFunctions, addFunctionMarket } from '@/services/api/functions';

export const functions: FunctionsStoreModel = {
  // State
  // --------------------------------------------------------------------------
  total: 0,
  
  personFunctions: [],

  publicFunctions: [],

  // Action
  // --------------------------------------------------------------------------

  setTotal: action((state, total = 0) => {
    state.total = total;
  }),

  setPersonFunctions: action((state, { functions }) => {
    state.personFunctions = functions;
  }),

  setPublicFunctions: action((state, { functions }) => {
    state.publicFunctions = functions;
  }),

  clear: action((state) => {
    state.total = 0;
    state.personFunctions = [];
    state.publicFunctions = [];
  }),

  // Thunk
  // --------------------------------------------------------------------------

  fetchPersonFunctions: thunk(async (actions, params) => {
    const { total, functions } = await fetchPersonFunctions(
      params
    );

    actions.setTotal(total);

    actions.setPersonFunctions({ total, functions });
  }),

  fetchPublicFunctions: thunk(async (actions, { creator }) => {
    actions.setPublicFunctions(await fetchPublicFunctions(creator));
  }),

  addFunctionMarket: thunk(async (actions, { functionId }) => {
    await addFunctionMarket(functionId);
  }),
};