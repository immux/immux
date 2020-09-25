import { MenuItemProps } from 'antd/es/menu/MenuItem';
import React, { ReactNode } from 'react';
import { MenuProps } from 'antd/es/menu';
import { Assign } from 'utility-types';
import { Menu } from 'antd';

import styles from './styles.module.less';
import classNames from 'classnames';
import _ from 'lodash';

function AsideMenu(
  props: Assign<Omit<MenuProps, 'mode'>, { children: ReactNode }>
) {
  return (
    <Menu
      {...props}
      className={classNames(styles.menu, props.className)}
      mode="inline"
    />
  );
}

AsideMenu.Item = function AsideMenuItem(
  props: Assign<MenuItemProps, { actions?: ReactNode }>
) {
  return (
    <Menu.Item {..._.omit(props, ['actions', 'children'])}>
      <div className={styles.content}>{props.children}</div>
      {props.actions && <div className={styles.actions}>{props.actions}</div>}
    </Menu.Item>
  );
};

export default AsideMenu;
