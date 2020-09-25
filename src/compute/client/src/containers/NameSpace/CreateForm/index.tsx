import { Typography, Button, Form, Input, message, Spin } from 'antd';
import React, { useCallback, useState } from 'react';

import styles from './styles.module.less';
import { useStoreActions } from '@/store/hooks';
import { createNameSpace } from '@/services/api/nameSpace';
import { catchError, trimStringValue } from '@/utils';
import { useFetchEditingNameSpace } from '@/services/hooks/nameSpace';
import { useParams, useHistory, useRouteMatch } from 'react-router-dom';

/**
 * creat project hook
 */
function useCreateNameSpace() {
  const history = useHistory();
  const [loading, setLoading] = useState(false);

  const create = useCallback(
    async (data: { name: string; title?: string; description?: string }) => {
      const hide = message.loading('creating', 0);

      try {
        setLoading(true);
        const { nameSpace } = await createNameSpace(data);
        message.success('create succuss');
        history.push(`/name-space/${nameSpace.id}`);
      } catch (err) {
        catchError(err);
      } finally {
        setLoading(false);
        hide();
      }
    },
    [history]
  );

  return [loading, create] as [boolean, () => Promise<void>];
}

/**
 * update hook
 */
function useUpdateNameSpace() {
  const history = useHistory();
  const { id: nameSpaceId = '' } = useParams();
  const [loading, setLoading] = useState(false);
  const updateEditingNameSpace = useStoreActions(
    (actions) => actions.nameSpace.updateEditingNameSpace
  );

  const update = useCallback(
    async (data: { title?: string; description?: string }) => {
      const hide = message.loading('updating', 0);

      try {
        setLoading(true);
        await updateEditingNameSpace({ nameSpaceId, data });
        message.success('update succuss');
        history.push(`/name-space/${nameSpaceId}`);
      } catch (err) {
        catchError(err);
      } finally {
        setLoading(false);
        hide();
      }
    },
    [history, nameSpaceId, updateEditingNameSpace]
  );

  return [loading, update] as [boolean, () => Promise<void>];
}

export default function CreateNameSpaceForm() {
  const { path } = useRouteMatch();
  const isCreating = path.startsWith('/name-space/new');
  const isEditing = path.startsWith('/name-space/edit');

  // init
  const [form] = Form.useForm();
  const [creating, createNameSpace] = useCreateNameSpace();
  const [editing, editNameSpace] = useUpdateNameSpace();

  // editing get form data
  const [loading] = useFetchEditingNameSpace(isEditing, form);

  return (
    <section className={styles.wrapper}>
      <Spin spinning={isEditing && loading}>
        <Typography.Title level={3}>
          {isCreating ? 'create project' : 'edit project'}
        </Typography.Title>

        <Form
          form={form}
          name="create-name-space"
          labelCol={{ span: 4 }}
          wrapperCol={{ span: 16 }}
          onFinish={isCreating ? createNameSpace : editNameSpace}
        >
          <Form.Item
            label="name"
            name="name"
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
          </Form.Item>

          <Form.Item label="title" name="title">
            <Input placeholder="input" />
          </Form.Item>

          <Form.Item label="description" name="description">
            <Input.TextArea placeholder="input" />
          </Form.Item>

          <Form.Item wrapperCol={{ offset: 4, span: 16 }}>
            <Button
              type="primary"
              htmlType="submit"
              loading={isCreating ? creating : editing}
            >
              {isCreating ? 'create' : 'submit'}
            </Button>
          </Form.Item>
        </Form>
      </Spin>
    </section>
  );
}
