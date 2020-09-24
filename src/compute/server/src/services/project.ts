import { Types } from 'mongoose';
import { HttpError } from 'routing-controllers';
import { recursiveTraversalWalk } from '@/utils/project';
import * as fs from 'fs';

/**
 * get project detail
 * @param projectById
 */
export function getProjectById(projectId: Types.ObjectId | string) {
  return recursiveTraversalWalk('../project/demo');
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
