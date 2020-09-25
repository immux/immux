import ToolboxDrawer from '@/containers/Toolbox/Drawer';
import HeaderNavigation from './HeaderNavigation';
import Authorized from '@/components/Authorized';
import { useRouteMatch } from 'react-router-dom';
import HeaderActions from './HeaderActions';
import React, { ReactNode } from 'react';
import { Layout, Row, Col } from 'antd';

import styles from './styles.module.less';
import classNames from 'classnames';

export default function SystemLayout(props: {
  divider: 'box-shadow' | 'border';
  children?: ReactNode;
  contentClassName?: string;
}) {
  const matchToolboxHome = useRouteMatch({ path: '/toolbox', exact: true });
  let matchAccountLoginRoute = useRouteMatch('/account/login');

  if (matchAccountLoginRoute || matchToolboxHome) {
    return <>{props.children}</>;
  }

  const headerClass = classNames(styles.header, {
    [styles.bordered]: props.divider === 'border'
  });

  return (
    <Authorized>
      <Layout className={styles.layout}>
        <Layout.Header className={headerClass}>
          <Row>
            <Col className={styles.systemBar} span={12}>
              <ToolboxDrawer />
              <HeaderNavigation />
            </Col>

            <Col className={styles.actions} span={12}>
              <HeaderActions />
            </Col>
          </Row>
        </Layout.Header>

        <Layout
          id="container"
          className={classNames(styles.content, props.contentClassName)}
        >
          {props.children}
        </Layout>
      </Layout>
    </Authorized>
  );
}

SystemLayout.defaultProps = {
  divider: 'box-shadow'
};
