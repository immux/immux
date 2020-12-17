import ListItem from '@/components/ListItem';
import React from 'react';
import { Avatar } from 'antd';

import {
  ProjectOutlined,
  UserAddOutlined,
  SettingOutlined,
  DeleteOutlined,
  StarOutlined
} from '@ant-design/icons';

export default function FunctionItem(props: {
  item: string;
}){
  return (
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
      title={props.item}
      extra={<StarOutlined />}
      actions={
        <>
          <UserAddOutlined />
          <DeleteOutlined />
          <SettingOutlined />
        </>
      }
    />
    )
}