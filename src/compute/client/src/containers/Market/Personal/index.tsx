import { SectionCard } from '@/components/Section';
import { Avatar, List, Result, Row, Spin, Tooltip } from 'antd';
import React from 'react';
import { useFunctionCollection } from './hooks';
import ListItem from './item';
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
          renderItem={(item) => ListItem({item})}
        />
      </SectionCard>
  );
}
