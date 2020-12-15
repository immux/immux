import { Link, LinkProps } from 'react-router-dom';
import { CardProps } from 'antd/es/card';
import React, { ReactNode } from 'react';
import { ColProps } from 'antd/es/col';
import { Card, Col, Tag } from 'antd';

import styles from './styles.module.less';
import classNames from 'classnames';
import { PresetColors } from '@/types';

export interface ProjectCardProps {
  className?: string;

  span?: ColProps['span'];

  extraIcon?: ReactNode;

  icon?: ReactNode;

  iconRotate?: boolean;

  tag?: string;

  actions?: ReactNode;

  fixedIcons?: ReactNode;

  title: ReactNode;

  description?: ReactNode;

  // prettier-ignore
  type: 'primary' | 'default' | PresetColors;

  href?: string;

  target?: string;

  to?: LinkProps['to'];

  onClick?: CardProps['onClick'];
}

export default function ProjectCard(props: ProjectCardProps) {
  let card = (
    <Card bordered={false} onClick={props.onClick}>
      {props.extraIcon && (
        <div className={styles.extraIcon}>{props.extraIcon}</div>
      )}

      {(props.fixedIcons || props.actions) && (
        <div className={styles.icons}>
          {props.actions && (
            <div className={styles.actions}>{props.actions}</div>
          )}

          {props.fixedIcons && (
            <div className={styles.fixedIcons}>{props.fixedIcons}</div>
          )}
        </div>
      )}

      {props.icon && (
        <div
          className={classNames(styles.cardIcon, {
            [styles.rotate]: props.iconRotate
          })}
        >
          {props.icon}
        </div>
      )}

      <div className={styles.cardContent}>
        <div className={styles.cardHeader}>
          {props.tag && <Tag>{props.tag}</Tag>}
          <div className={styles.cardTitle}>{props.title}</div>
        </div>

        {props.description && (
          <div className={styles.cardDesc}>{props.description}</div>
        )}
      </div>
    </Card>
  );

  if (props.to) {
    card = <Link to={props.to}>{card}</Link>;
  } else if (props.href) {
    card = (
      <a href={props.href} target={props.target}>
        {card}
      </a>
    );
  }

  return (
    <Col
      className={classNames(styles.card, styles[props.type], props.className)}
      span={props.span}
    >
      {card}
    </Col>
  );
}

ProjectCard.defaultProps = {
  iconRotate: true,
  type: 'default',
  span: 6
};
