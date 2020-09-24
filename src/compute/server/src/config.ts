import { ComputeConfig } from '@/types';

import { requireYaml } from '@/utils';
import _ = require('lodash');
import * as path from 'path';

const deployEnv = process.env.DEPLOY_ENV || 'development';
const configFolderPath = path.join(process.cwd(), 'config');

const config: ComputeConfig = _.merge(
  {},
  requireYaml(path.join(configFolderPath, 'base.yaml')),
  requireYaml(path.join(configFolderPath, `${deployEnv}.yaml`))
);

export default config;
