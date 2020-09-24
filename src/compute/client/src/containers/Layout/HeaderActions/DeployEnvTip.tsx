import { ConstructionIcon, LabIcon } from '@/components/icons';
import { Divider } from 'antd';
import React from 'react';

import styles from './styles.module.less';
import { deployEnv } from '@/constants';
import classNames from 'classnames';

export default function DeployEnvTip() {
  if (deployEnv === 'release') {
    return null;
  }

  return (
    <>
      <div className={classNames(styles.deployEnv, styles[deployEnv])}>
        {deployEnv === 'development' ? <ConstructionIcon /> : <LabIcon />}
        <code>{deployEnv}</code>
      </div>

      <Divider type="vertical" />
    </>
  );
}
