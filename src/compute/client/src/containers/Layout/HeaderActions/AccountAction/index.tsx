import AccountPemsManager from '@/containers/Account/PemsManager';
import { Dropdown, Menu, Avatar } from 'antd';
import { MenuProps } from 'antd/lib/menu';
import React from 'react';

import {
  EllipsisOutlined,
  SettingOutlined,
  LogoutOutlined,
  GlobalOutlined,
  AuditOutlined,
  UserOutlined
} from '@ant-design/icons';

import { betaLink, deployEnv, deployEnvLabel, releaseLink } from '@/constants';
import { useModalActions } from '@/utils/hooks';
import styles from './styles.module.less';

import {
  useAccountLogoutAction,
  useAccountAvatar
} from '@/services/hooks/account';

enum DropdownActions {
  Pems = 'pems',

  Preference = 'preference',

  Settings = 'settings',

  SwitchToRelease = 'switch-to-release',

  SwitchToBeta = 'switch-to-beta',

  Logout = 'logout'
}

function DropdownMenu(props: MenuProps) {
  return (
    <Menu {...props}>
      <Menu.Item key={DropdownActions.Pems}>
        <AuditOutlined />
        Pems Manager
      </Menu.Item>

      <Menu.Divider />

      <Menu.Item key={DropdownActions.Preference} disabled>
        <GlobalOutlined />
        Preference Settings
      </Menu.Item>

      <Menu.Item key={DropdownActions.Settings} disabled>
        <SettingOutlined />
        Account Settings
      </Menu.Item>

      <Menu.Divider />

      {deployEnv !== 'beta' && (
        <Menu.Item
          key={DropdownActions.SwitchToBeta}
          className={styles.multiItemWrapper}
        >
          <a className={styles.multiItem} href={betaLink}>
            <strong>turn to dev</strong>
            <small>using {deployEnvLabel}</small>
          </a>
        </Menu.Item>
      )}

      {deployEnv !== 'release' && (
        <Menu.Item
          key={DropdownActions.SwitchToRelease}
          className={styles.multiItemWrapper}
        >
          <a className={styles.multiItem} href={releaseLink}>
            <strong>turn to release</strong>
            <small>using {deployEnvLabel}</small>
          </a>
        </Menu.Item>
      )}

      <Menu.Divider />

      <Menu.Item key={DropdownActions.Logout}>
        <LogoutOutlined />
        Logout
      </Menu.Item>
    </Menu>
  );
}

export default function Account() {
  const avatar = useAccountAvatar();
  const [, onLogout] = useAccountLogoutAction();
  const [
    pemsManagerVisible,
    openPemsManager,
    closePemsManager
  ] = useModalActions();

  const onClick: MenuProps['onClick'] = ({ key }) => {
    switch (key) {
      case DropdownActions.Pems: {
        openPemsManager();
        return;
      }

      case DropdownActions.Logout: {
        onLogout();
        return;
      }

      default: {
      }
    }
  };

  return (
    <>
      <Dropdown
        overlayClassName={styles.overlay}
        overlay={<DropdownMenu onClick={onClick} />}
        trigger={['click']}
      >
        <span className={styles.action}>
          <span className={styles.ellipsis}>
            <EllipsisOutlined />
          </span>

          <Avatar size={28} src={avatar} icon={<UserOutlined />} />
        </span>
      </Dropdown>

      <AccountPemsManager
        visible={pemsManagerVisible}
        onClose={closePemsManager}
      />
    </>
  );
}
