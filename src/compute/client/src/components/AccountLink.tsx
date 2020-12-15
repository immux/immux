import { WomanOutlined, ManOutlined, UserOutlined } from '@ant-design/icons';
import { Account } from '@/types/models';
import { Avatar, Tooltip } from 'antd';
import { Gender } from '@/constants';
import React from 'react';

import styles from './styles.module.less';
import { Assign } from 'utility-types';
import classNames from 'classnames';

function GenderIcon(props: { className?: string; gender?: Gender }) {
  switch (props.gender) {
    case Gender.Female: {
      return <WomanOutlined className={props.className} />;
    }

    case Gender.Male: {
      return <ManOutlined className={props.className} />;
    }

    default: {
      return null;
    }
  }
}

export default function AccountLink(
  account: Assign<Partial<Account>, { className?: string }>
) {
  return (
    <Tooltip title={<code>{account.email}</code>}>
      <span className={classNames(styles.accountLink, account.className)}>
        <Avatar
          className={styles.avatar}
          size="small"
          icon={<UserOutlined />}
          src={account.avatar}
        />
        {account.name}
        <GenderIcon className={styles.gender} gender={account.gender} />
      </span>
    </Tooltip>
  );
}
