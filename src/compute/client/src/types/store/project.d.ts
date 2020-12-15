import { NameSpace, Account, ComputedNameSpace } from '@/types/models';
import { NonUndefined, PromiseType } from 'utility-types';
import { Action, Computed, Thunk } from 'easy-peasy';
import { StoreModel } from '@/types/store';

import { fetchProjectFolders } from '@/services/api/project';

export interface FileNodeInfo {
  children: FileNodeInfo[];
  pNode: string;
  title: string;
  key: string;
  extname?: string;
  detail?: string;
  isLeaf?: boolean;
}

export interface ProjectStoreModel {
  // State
  // --------------------------------------------------------------------------

  folders: FileNodeInfo[];

  activeNode: FileNodeInfo;

  // Action
  // --------------------------------------------------------------------------

  setProjectFolders: Action<
    ProjectStoreModel,
    PromiseType<ReturnType<typeof fetchProjectFolders>>
  >;

  setActiveNode: Action<ProjectStoreModel, FileNodeInfo>;

  clearProjectFolders: Action<ProjectStoreModel, void>;

  // Thunk
  // --------------------------------------------------------------------------

  fetchProjectFolders: Thunk<
    ProjectStoreModel,
    { projectId: string },
    any,
    StoreModel,
    Promise<void>
  >;
}
