import React, { Children, Fragment, cloneElement, ReactElement } from 'react';
import { RightOutlined, HomeFilled } from '@ant-design/icons';
import { Link } from 'react-router-dom';
import { Tooltip } from 'antd';

import styles from './styles.module.less';
import classNames from 'classnames';

export default function NavigationBreadcrumb(props: {
  children: ReactElement | ReactElement[];
}) {
  const children = Children.map(props.children, (child, index) => (
    <Fragment key={child.key || index}>
      <RightOutlined className={styles.separator} />
      {cloneElement(child, {
        className: classNames(styles.breadcrumb, child.props.className)
      })}
    </Fragment>
  ));

  return (
    <div className={styles.navigation}>
      <h1>
        <Tooltip title="return home">
          <Link className={styles.home} to="/">
            <HomeFilled />
          </Link>
        </Tooltip>

        <img src={`${process.env.PUBLIC_URL}/logo.png`} alt="compute" />
      </h1>

      {children}
    </div>
  );
}
