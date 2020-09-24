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

      <SectionCard
        className={styles.section}
        title="Recent projects"
        extra={
          <>
            <UnorderedListOutlined />
            <SortAscendingOutlined />
          </>
        }
      >
        <List<string>
          dataSource={[
            'test1',
            'test12',
            'test13',
            'test14',
            'test15',
            'test16'
          ]}
          renderItem={(item) => (
            <ListItem
              mode="project"
              href="#"
              avatar={
                <Avatar
                  shape="square"
                  icon={<ProjectOutlined />}
                  size="large"
                />
              }
              title={item}
              extra={<StarOutlined />}
              actions={
                <>
                  <UserAddOutlined />
                  <DeleteOutlined />
                  <SettingOutlined />
                </>
              }
            />
          )}
        />
      </SectionCard>
    </MiddleContainer>
  );
}
