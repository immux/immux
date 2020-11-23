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
import requestPromise = require('request-promise-native');

@JsonController('/cli/upload')
export default class FileHistoryController {
  @Post('')
  async updateFile(
    @UploadedFiles('files') files: any[], 
    @Ctx() ctx: Context,
    @BodyParam('name') ProjectName: string,
  ){
    if (!files.length) {
      throw new HttpError(400, 'please add files');
    }

    await dirExists(`./uploads/${ProjectName}`);

    for (let i = 0; i < files.length; i++) {
      fs.writeFile(
        `./uploads/${ProjectName}/${files[i].originalname}`,
        files[i].buffer,
        'binary',
        function (err) {
          if (err) {
            throw new HttpError(400, err.message);
          }
        }
      );
    }

    return { total: files.length };
  }
}
