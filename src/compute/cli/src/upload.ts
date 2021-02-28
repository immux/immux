import { Uploader } from '@/services/uploader';
import { isDevelopment } from '@/constants';

import ora = require('ora');
import path from 'path';
// import { getComputeConfig } from '@/services';

export { Uploader } from '@/services/uploader';

export default async function upload() {
  const spinner = ora();

  try {
    // const computeConfig = await getComputeConfig();
    // const cwd = path.join(process.cwd(), computeConfig.upload.dir);
    const uploader = new Uploader(process.cwd());

    await uploader.exec();
  } catch (err) {
    if (isDevelopment) {
      console.error(err);
    }

    spinner.fail(err.message);
    process.exit(1);
  }
}
