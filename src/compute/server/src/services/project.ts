import { Types } from 'mongoose';
import { HttpError } from 'routing-controllers';
import { recursiveTraversalWalk } from '@/utils/project';
import * as fs from 'fs';

import { getNameSpaceById } from './NameSpace';

/**
 * get project detail
 * @param projectById
 */
export async function getProjectById(projectId: Types.ObjectId | string) {
  // return recursiveTraversalWalk('../project/demo');
  const project = await getNameSpaceById(projectId);
  return recursiveTraversalWalk(`./uploads/${project.name}`);
}

export async function saveCodeChange(
  projectId: Types.ObjectId | string,
  code: string,
  dir: string
) {
  if (!fs.existsSync(dir)) {
    throw new HttpError(404, 'file not found');
  }

  await fs.writeFileSync(dir, code, { encoding: 'utf8' });

  return projectId;
}
