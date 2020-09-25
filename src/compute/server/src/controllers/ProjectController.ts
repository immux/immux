import { AccessTokenMiddleware } from '@/middlewares/account';

import {
  JsonController,
  UseBefore,
  HttpError,
  Param,
  Get,
  Post,
  BodyParam
} from 'routing-controllers';

import { getProjectById, saveCodeChange } from '@/services/project';
import { PRE_FILE } from '@/constants';

@JsonController('/project')
export default class ProjectIdController {
  /**
   * get project detail
   * @param projectId
   */
  @Get('/:projectId')
  @UseBefore(AccessTokenMiddleware)
  async getNameSpace(@Param('projectId') projectId: string) {
    const project = await getProjectById(projectId);

    if (!project) {
      throw new HttpError(404, 'project not found');
    }

    return project;
  }

  /**
   * save code change
   * @param code File content
   * @param dir File location
   */
  @Post('/:projectId')
  @UseBefore(AccessTokenMiddleware)
  async addOrUpdateFileHistory(
    @Param('projectId') projectId: string,
    @BodyParam('code') code: string,
    @BodyParam('dir') dir: string
  ) {
    const regExp = new RegExp(`^${PRE_FILE}`);

    const modifyDir = dir.replace(regExp, '');

    return saveCodeChange(projectId, code, modifyDir);
  }
}
