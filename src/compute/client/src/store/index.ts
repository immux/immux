import { StoreModel } from '@/types/store';

import { createStore } from 'easy-peasy';
import { nameSpace } from './nameSpace';
import { account } from './account';
import { project } from './project';

const storeModel: StoreModel = {
  nameSpace,
  account,
  project
};

export const store = createStore(storeModel);
