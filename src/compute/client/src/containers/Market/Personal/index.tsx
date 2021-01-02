import { SectionCard } from '@/components/Section';
import ListItem from '@/components/ListItem';
import { Avatar, List, Result, Row, Spin, Tooltip } from 'antd';
import React from 'react';
import { useFunctionCollection } from './hooks';
import { FunctionInfo } from '@/types/store/functions';

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
  const [
    loading,
    error,
    functions,
    hasMore,
    fetchPersonFunctions
  ] = useFunctionCollection();

  if (error) {
    return <Result {...error.props} />;
  }

  console.log('functions', functions);

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
        <List<FunctionInfo>
          dataSource={functions}
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
              title={`${item.projectId}/${item.name}`}
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
