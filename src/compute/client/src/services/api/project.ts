import { FileNodeInfo } from '@/types/store/project';

import { createInstance } from '@/utils/axios';

const instance = createInstance('/api/project');

/**
 * get project folders
 * @param projectId
 */
export async function fetchProjectFolders(projectId: string) {
  return instance.get<
    any,
    {
      children: FileNodeInfo[];
    }
  >(`/${projectId}`);
}

/**
 * save code change
 * @param projectId
 */
export async function fetchSaveCode(
  projectId: string,
  code: string,
  dir: string | number
) {
  return instance.post<any, number>(`/${projectId}`, {
    code,
    dir
  });
}
