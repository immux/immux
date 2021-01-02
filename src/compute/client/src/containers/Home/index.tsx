import MiddleContainer from '@/containers/Layout/MiddleContainer';
import { SectionCard } from '@/components/Section';
import ListItem from '@/components/ListItem';
import { Avatar, List } from 'antd';
import HomeAside from './Aside';
import React from 'react';

import {
  UnorderedListOutlined,
  SortAscendingOutlined,
  ProjectOutlined,
  UserAddOutlined,
  SettingOutlined,
  DeleteOutlined,
  StarOutlined
} from '@ant-design/icons';

import styles from './styles.module.less';
import Histories from '@/containers/Home/Histories';

export default function Home() {
  return (
    <MiddleContainer aside={<HomeAside />}>
      <SectionCard title="Recent actions">
        <Histories />
      </SectionCard>
    </MiddleContainer>
  );
}
