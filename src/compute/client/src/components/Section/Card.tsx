import { CardProps } from 'antd/es/card';
import { Card } from 'antd';
import React from 'react';

import styles from './styles.module.less';
import classNames from 'classnames';

export default function SectionCard(props: Omit<CardProps, 'bordered'>) {
  return (
    <Card
      {...props}
      className={classNames(styles.card, props.className)}
      bordered={false}
    />
  );
}
