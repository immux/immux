import ListItem from '@/components/ListItem';
import React from 'react';
import { Avatar, Button, message, Modal } from 'antd';
import { FunctionInfo } from '@/types/store/functions';
import { ResultError } from '@/utils/error';
import { catchError } from '@/utils';
import { useStoreActions, useStoreState } from '@/store/hooks';

import {
  ProjectOutlined,
  UserAddOutlined,
  SettingOutlined,
  DeleteOutlined,
  StarOutlined,
  PlusOutlined,
} from '@ant-design/icons';

export default function FunctionItem(props: {
  item: FunctionInfo;
}){
  const info = props.item;

  // const [ loading, addMarket ] = useAddMarket(info.id);

  const addFunctionMarket = useStoreActions(
    (actions) => actions.functions.addFunctionMarket
  );

  const addMarket = async() => {
    Modal.confirm({
      title: '函数市场',
      content: (
        <>
          确认将<strong>{` ${info.projectId}/${info.name} `}</strong>添加至函数市场吗？
        </>
      ),

      okButtonProps: { danger: true },

      onOk: async () => {
        try {
          await addFunctionMarket({functionId: info.id});
          message.success('add market succuss!');
        } catch (err) {
          catchError(err);
        }
      }
    });
  }

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
      title={`${info.projectId}/${info.name}`}
      extra={
        <Button
          type="primary"
          shape="circle"
          size="small"
          icon={ <PlusOutlined /> }
          onClick={ addMarket }
        ></Button>
      }
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