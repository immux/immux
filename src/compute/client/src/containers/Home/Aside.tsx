import { AsideMenu, AsideCard } from '@/components/Aside';
import { Link, useRouteMatch } from 'react-router-dom';
import React, { useEffect, useState } from 'react';
import { Menu } from 'antd';

import {
  AppstoreOutlined,
  ProjectOutlined,
  BuildOutlined,
  HomeOutlined
} from '@ant-design/icons';

import styles from './styles.module.less';
import _ from 'lodash';

const NAME_SPACE_KEY = 'name-space';
const HOME_KEY = 'home';
const MY_FUNCTIONS_KEY = 'my-functions';
const MARKET_KEY = 'market';

export default function HomeAside() {
  const [selectedKeys, setSelectedKeys] = useState<string[]>([]);
  const matchHomePage = useRouteMatch({ path: '/', exact: true });
  const matchNameSpace = useRouteMatch('/name-space');

  useEffect(() => {
    const conditions: Array<[ReturnType<typeof useRouteMatch>, string[]]> = [
      [matchHomePage, [HOME_KEY]],
      [matchNameSpace, [NAME_SPACE_KEY]]
    ];

    for (const [match, keys] of conditions) {
      if (match && !_.isEqual(selectedKeys, keys)) {
        setSelectedKeys(keys);
        break;
      }
    }
  }, [selectedKeys, setSelectedKeys, matchNameSpace, matchHomePage]);

  return (
    <div className={styles.aside}>
      <AsideMenu selectedKeys={selectedKeys}>
        <Menu.Item key={HOME_KEY}>
          <Link to="/">
            <HomeOutlined />
            Dashboard
          </Link>
        </Menu.Item>

        <Menu.Item key={NAME_SPACE_KEY}>
          <Link to="/name-space">
            <ProjectOutlined />
            MyProjects
          </Link>
        </Menu.Item>

        <Menu.Item key="2" disabled>
          <AppstoreOutlined />
          SDK
        </Menu.Item>
      </AsideMenu>

      <AsideCard className={styles.section} title="Function Market">
        <AsideMenu selectedKeys={[]}>
          <Menu.Item key="MARKET_KEY">
            <Link to="/market/public-functions">
              <AppstoreOutlined />
              Market
            </Link>  
          </Menu.Item>

          <Menu.Item key="MY_FUNCTIONS_KEY">
            <Link to="/market/personal-functions">
              <BuildOutlined />
              MyFunctions
            </Link>  
          </Menu.Item>
        </AsideMenu>
      </AsideCard>
    </div>
  );
}
