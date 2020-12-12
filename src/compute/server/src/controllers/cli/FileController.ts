import { Context } from 'koa';
import * as fs from 'fs';

import {
  JsonController,
  BodyParam,
  UseBefore,
  UploadedFiles,
  State,
  Post,
  Ctx,
  HttpError
} from 'routing-controllers';

import { dirExists } from '@/utils';
import { AccessTicketMiddleware } from '@/middlewares/account';
import { AccountSchema } from '@/types/models/Account';
import { createNameSpace } from '@/services/nameSpace';
import { createFunction } from '@/services/functions';
import _ = require('lodash');

import requestPromise = require('request-promise-native');

@JsonController('/cli/upload')
export default class FileHistoryController {
  @Post('')
  @UseBefore(AccessTicketMiddleware)
  async updateFile(
    @State('account') account: AccountSchema,
    @Ctx() ctx: Context,
    @UploadedFiles('files') files: any[], 
    @BodyParam('name') ProjectName: string,
  ){
    if (!files.length) {
      throw new HttpError(400, 'please add files');
    }

    await dirExists(`./uploads/${ProjectName}`);

    for (let i = 0; i < files.length; i++) {
      const fileName = files[i].originalname;

      createFunction(
        account,
        {
          name: fileName,
          projectId: ProjectName,
        }
      );

      fs.writeFile(
        `./uploads/${ProjectName}/${fileName}`,
        files[i].buffer,
        'binary',
        function (err) {
          if (err) {
            throw new HttpError(400, err.message);
          }
        }
      );
    }

    const nameSpace = createNameSpace( account, { name: ProjectName });
 
    if (!nameSpace) {
      throw new HttpError(404, 'nameSpace not found');
    }

    return { total: files.length };
  }
}
