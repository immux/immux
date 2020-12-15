import { AsideMenu } from '@/components/Aside';
import React, { ReactNode } from 'react';
import { CardProps } from 'antd/es/card';
import { Assign } from 'utility-types';
import { Card } from 'antd';

import styles from './styles.module.less';
import classNames from 'classnames';

function isAsideMenuChildren(children?: ReactNode) {
  let isAsideMenu = false;

  React.Children.forEach<typeof AsideMenu | any>(children, (child) => {
    isAsideMenu = isAsideMenu || child.type === AsideMenu;
  });

  return isAsideMenu;
}

export default function AsideCard(
  props: Assign<Omit<CardProps, 'bordered'>, { children?: ReactNode }>
) {
  return (
    <Card
      {...props}
      className={classNames(styles.card, props.className, {
        [styles.paddingContent]: !isAsideMenuChildren(props.children)
      })}
      bordered={false}
    />
  );
}
