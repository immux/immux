import { CodeTwoTone, BugTwoTone } from '@ant-design/icons';
import { MenuDotsIcon } from '@/components/icons';
import React, { useState } from 'react';
import { Col, Drawer, Row } from 'antd';

import { primaryColor } from '@/constants';
import styles from './styles.module.less';
import { Link } from 'react-router-dom';

export default function ToolboxDrawer() {
  const [visible, setVisible] = useState(false);

  return (
    <>
      <MenuDotsIcon
        className={styles.icon}
        onClick={setVisible.bind(null, true)}
      />

      <Drawer
        className={styles.drawer}
        maskStyle={{ background: 'transparent' }}
        title={
          <>
            <MenuDotsIcon
              className={styles.icon}
              onClick={setVisible.bind(null, false)}
            />
            tools
          </>
        }
        placement="left"
        width={320}
        visible={visible}
        destroyOnClose={true}
        onClose={setVisible.bind(null, false)}
      >
        <Row className={styles.apps} gutter={8}>
          <Col span={8}>
            <div className={styles.appWrapper}>
              <Link
                className={styles.app}
                to="/toolbox/jql"
                onClick={setVisible.bind(null, false)}
              >
                <CodeTwoTone twoToneColor={primaryColor} />
                <p>db tools</p>
              </Link>
            </div>
          </Col>

          <Col span={8}>
            <div className={styles.appWrapper}>
              <Link
                className={styles.app}
                to="/toolbox/cleansing"
                onClick={setVisible.bind(null, false)}
              >
                <BugTwoTone twoToneColor={primaryColor} />
                <p>clean data</p>
              </Link>
            </div>
          </Col>
        </Row>
      </Drawer>
    </>
  );
}
