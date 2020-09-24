import React, { ReactElement, ReactNode } from 'react';
import { Affix, Layout, Spin } from 'antd';

import styles from './styles.module.less';

export default function MiddleContainer(props: {
  loading: boolean;
  aside?: ReactElement;
  error?: ReactElement;
  children?: ReactNode;
}) {
  let children: ReactElement;

  if (props.loading) {
    children = <Spin className={styles.middleContainerSpin} />;
  } else if (props.error) {
    children = props.error;
  } else {
    children = (
      <>
        {props.aside && (
          <Layout.Sider className={styles.middleContainerAside} width={304}>
            <Affix offsetTop={90}>{props.aside}</Affix>
          </Layout.Sider>
        )}

        <Layout.Content
          id="content"
          className={styles.middleContainerContentWrapper}
        >
          <div className={styles.middleContainerContent}>{props.children}</div>
        </Layout.Content>
      </>
    );
  }

  return <div className={styles.middleContainer}>{children}</div>;
}

MiddleContainer.defaultProps = {
  loading: false
};
