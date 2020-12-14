import { SectionCard } from '@/components/Section';
import ListItem from '@/components/ListItem';
import { Avatar, List } from 'antd';
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

export default function PersonalFunction() {
  return (
      <SectionCard
        title="My functions"
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
  );
}
