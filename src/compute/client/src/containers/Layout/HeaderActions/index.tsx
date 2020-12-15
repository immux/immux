import { Tooltip, Popover, Divider } from 'antd';
import AccountAction from './AccountAction';
import React, { useState } from 'react';

import {
  SafetyCertificateOutlined,
  QuestionCircleOutlined
} from '@ant-design/icons';

import styles from './styles.module.less';
import moment from 'moment';
import classNames from 'classnames';
import DeployEnvTip from '@/containers/Layout/HeaderActions/DeployEnvTip';

export default function HeaderActions() {
  const [hovered, setHovered] = useState(false);

  return (
    <>
      <DeployEnvTip />

      <Popover
        overlayClassName={styles.versionOverlay}
        content={
          <dl>
            <dt>Compile</dt>
            <dd>
              {moment(process.env.COMPUTE_BUILD_TIME).format('YYYY-MM-DD HH:mm')}
            </dd>
          </dl>
        }
        onVisibleChange={setHovered.bind(null)}
      >
        <span
          className={classNames(styles.version, { [styles.hover]: hovered })}
        >
          <SafetyCertificateOutlined />
          <code>{process.env.COMPUTE_VERSION}</code>
        </span>
      </Popover>

      <Divider type="vertical" />

      <Tooltip title="help">
        <a className={styles.iconBtn} href="/" target="_blank">
          <QuestionCircleOutlined />
        </a>
      </Tooltip>

      <Divider type="vertical" />

      <AccountAction />
    </>
  );
}
