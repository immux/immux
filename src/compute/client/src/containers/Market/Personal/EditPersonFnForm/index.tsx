import { Typography, Button, Form, Input, message, Spin } from 'antd';
import React, { useCallback, useState } from 'react';

import styles from './styles.module.less';
import { useStoreActions } from '@/store/hooks';
import { catchError, trimStringValue } from '@/utils';
import { useFetchEditingFunction } from '@/services/hooks/functions';
import { useParams, useHistory, useRouteMatch } from 'react-router-dom';

/**
 * edit hook
 */
function useEditFunction() {
  const history = useHistory();
  const { id: functionId = '' } = useParams();
  const [loading, setLoading] = useState(false);
  const updateEditingFunction = useStoreActions(
    (actions) => actions.functions.updateEditingFunction
  );

  const update = useCallback(
    async (data: { title?: string; description?: string, price?: number }) => {
      const hide = message.loading('updating', 0);

      try {
        setLoading(true);
        await updateEditingFunction({ functionId, data });
        message.success('update succuss');
        history.push(`/market/personal-functions`);
      } catch (err) {
        catchError(err);
      } finally {
        setLoading(false);
        hide();
      }
    },
    [history, functionId, updateEditingFunction]
  );

  return [loading, update] as [boolean, () => Promise<void>];
}

export default function EditPersonFnForm() {
  const { path } = useRouteMatch();
  const isEditing = path.startsWith('/market/edit');
  // init
  const [form] = Form.useForm();
  const [editing, editFunction] = useEditFunction();

  // editing get form data
  const [loading] = useFetchEditingFunction(isEditing, form);
  
  return (
    <section className={styles.wrapper}>
      <Spin spinning={isEditing && loading}>
        <Typography.Title level={3}>
          Edit function info
        </Typography.Title>

        <Form
          form={form}
          name="create-name-space"
          labelCol={{ span: 4 }}
          wrapperCol={{ span: 16 }}
          onFinish={editFunction}
        >
          {/* <Form.Item
            label="functionName"
            name="functionName"
            rules={[
              { required: true, message: 'please enter name' },
              {
                pattern: /[^/]$/,
                message: 'name can not end of「/」',
                transform: trimStringValue
              }
            ]}
          >
            <Input placeholder="input" />
          </Form.Item> */}

          <Form.Item label="title" name="title">
            <Input placeholder="title" />
          </Form.Item>

          <Form.Item label="description" name="description">
            <Input.TextArea placeholder="description" />
          </Form.Item>

          <Form.Item label="price" name="price">
            <Input.TextArea placeholder="price" />
          </Form.Item>

          <Form.Item wrapperCol={{ offset: 4, span: 16 }}>
            <Button
              type="primary"
              htmlType="submit"
              loading={editing}
            >
              submit
            </Button>
          </Form.Item>
        </Form>
      </Spin>
    </section>
  );
}
