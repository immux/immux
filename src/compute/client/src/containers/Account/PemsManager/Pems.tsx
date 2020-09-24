import FadeTransition from '@/components/FadeTransition';
import { AuditOutlined } from '@ant-design/icons';
import { Popconfirm, Button, Empty } from 'antd';
import SaveButton from './SaveButton';
import React from 'react';

import { useStoreState } from '@/store/hooks';
import styles from './styles.module.less';

import {
  useDestroyAccountPems,
  useCreateAccountPems,
  useAccountProfile
} from '@/services/hooks/account';

export default function Pems() {
  const hasPems = useStoreState((state) => state.account.pems.hasPems);
  const hash = useStoreState((state) => state.account.pems.hashLabel);
  const createAt = useStoreState((state) => state.account.pems.createAt);
  const [name, email] = useAccountProfile();
  const [createLoading, createPems] = useCreateAccountPems();
  const [destroyLoading, destroyPems] = useDestroyAccountPems();

  return (
    <FadeTransition in={hasPems}>
      {hasPems ? (
        <article className={styles.pems}>
          <header>
            <AuditOutlined />
          </header>

          <main>
            <h2>
              <strong>
                {name}
                <small>
                  <code>{email}</code>
                </small>
              </strong>
              <code>{hash}</code>
            </h2>
            <p>{createAt.format('YYYY-MM-DD HH:mm')} generate</p>
          </main>

          <footer>
            <SaveButton />

            <Popconfirm
              okType="danger"
              okText="destroy"
              title="Whether to delete the current certificate"
              onConfirm={destroyPems}
            >
              <Button danger loading={destroyLoading}>
                destroy
              </Button>
            </Popconfirm>
          </footer>
        </article>
      ) : (
        <Empty
          image={Empty.PRESENTED_IMAGE_SIMPLE}
          description="No certificate is generated, please click the button below to generate"
        >
          <Button type="primary" loading={createLoading} onClick={createPems}>
            generate pem
          </Button>
        </Empty>
      )}
    </FadeTransition>
  );
}
