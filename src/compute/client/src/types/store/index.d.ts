import { NameSpaceStoreModel } from './nameSpace';
import { AccountStoreModel } from './account';
import { ProjectStoreModel } from './project';

export interface StoreModel {
  nameSpace: NameSpaceStoreModel;
  account: AccountStoreModel;
  project: ProjectStoreModel;
}
