import React from 'react';
import ListItem from '@/components/ListItem';
import { List } from 'antd';

export default function Histories() {
  return (
    <List
      dataSource={[]}
      locale={{ emptyText: 'No data' }}
      renderItem={(item) => <ListItem>{item}</ListItem>}
    />
  );
}
