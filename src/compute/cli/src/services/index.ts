import { ComputeConfig } from '@/types';

import { INI_FILENAME, INI_FILEPATH } from '@/constants';
import chalk = require('chalk');
import util = require('util');
import fs = require('fs');
import _ from 'lodash';
import ini from 'ini';

const readFileAsync = util.promisify(fs.readFile);

export async function getComputeConfig() {
  if (!fs.existsSync(INI_FILEPATH)) {
    throw new Error(`Configuration file ${chalk.gray(INI_FILENAME)} does not exist`);
  }

  const config = ini.parse(
    await readFileAsync(INI_FILEPATH, { encoding: 'utf8' })
  ) as ComputeConfig;

  if (!_.has(config, ['upload', 'dir'])) {
    throw new Error(`Upload catalog ${chalk.gray('upload.dir')} unspecified`);
  }

  return config;
}
