import { Context } from 'koa';
import multer = require('multer');
import * as fs from 'fs';

import {
  JsonController,
  BodyParam,
  UseBefore,
  UploadedFiles,
  State,
  Post,
  Ctx,
  HttpError,
} from 'routing-controllers';
import requestPromise = require('request-promise-native');

@JsonController('/cli/upload')
export default class FileHistoryController {
  @Post('')
  async updateFile(@UploadedFiles('files') files: any[], @Ctx() ctx: Context) {
      if (!files.length) {
        throw new HttpError(400, 'please add files');
      }

      for (let i = 0; i < files.length; i++) {
        fs.writeFile(`./uploads/${files[i].originalname}`, files[i].buffer,  "binary",function(err) {
          if(err) {
            throw new HttpError(400, err.message);
          }
        });
      }
      
      return { total: files.length }
  }
}
