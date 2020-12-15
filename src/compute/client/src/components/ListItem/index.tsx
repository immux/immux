import React, { ReactNode, HTMLAttributes } from 'react';
import { List } from 'antd';

import styles from './styles.module.less';
import classNames from 'classnames';

export default function ListItem(props: {
  className?: string;
  mode: 'default' | 'project';
  href?: string;
  target?: string;
  avatar?: ReactNode;
  extra?: ReactNode;
  extraClassName?: string;
  title?: ReactNode;
  actions?: ReactNode;
  children?: ReactNode;
  onAnchorClick?: HTMLAttributes<HTMLAnchorElement>['onClick'];
}) {
  const item = (
    <List.Item
      className={classNames(
        styles.listItem,
        styles[props.mode],
        props.className
      )}
      extra={
        <>
          {props.actions && (
            <div className={styles.actions}>{props.actions}</div>
          )}
          {props.extra && (
            <div className={classNames(styles.extra, props.extraClassName)}>
              {props.extra}
            </div>
          )}
        </>
      }
    >
      <List.Item.Meta avatar={props.avatar} title={props.title} />
      {props.children}
    </List.Item>
  );

  // prettier-ignore
  return (
    props.href ?
      (
        <a
          className={classNames(styles.listItemAnchor, { [styles.hoverable]: props.mode === 'default' })}
          href={props.href}
          target={props.target}
          onClick={props.onAnchorClick}
        >
          {item}
        </a>
      ) :
      item
  );
}

ListItem.defaultProps = {
  mode: 'default'
};
