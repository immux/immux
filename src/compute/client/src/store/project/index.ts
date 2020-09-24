import { ProjectStoreModel, FileNodeInfo } from '@/types/store/project';

import { action, thunk } from 'easy-peasy';

import { fetchProjectFolders } from '@/services/api/project';

export const project: ProjectStoreModel = {
  // State
  // --------------------------------------------------------------------------

  folders: [],

  activeNode: {
    children: [],
    pNode: '',
    title: '',
    key: '',
    extname: '',
    detail: '',
    isLeaf: false
  },

  // Action
  // --------------------------------------------------------------------------

  setProjectFolders: action((state, { children }) => {
    state.folders = children;
  }),

  setActiveNode: action((state, node: FileNodeInfo) => {
    state.activeNode = node;
  }),

  clearProjectFolders: action((state) => {
    state.folders = [];
  }),

  // Thunk
  // --------------------------------------------------------------------------

  fetchProjectFolders: thunk(async (actions, { projectId }) => {
    actions.setProjectFolders(await fetchProjectFolders(projectId));
  })
};
