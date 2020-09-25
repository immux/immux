import ScrollableModal from '@/components/ScrollableModal';
import FadeTransition from '@/components/FadeTransition';
import Pems from '@/containers/Account/PemsManager/Pems';
import React, { useEffect } from 'react';
import PemsSkeleton from './PemsSkeleton';
import { Alert, Card } from 'antd';

import { useAccountPemsFetcher } from '@/services/hooks/account';
import styles from './styles.module.less';

export default function AccountPemsManager(props: {
  visible: boolean;
  onClose: () => void;
}) {
  const [loading, fetchPems, resetPems] = useAccountPemsFetcher();

  useEffect(() => {
    if (props.visible) {
      fetchPems();
    }
  }, [props.visible, fetchPems]);

  return (
    <ScrollableModal
      title="pems manager"
      visible={props.visible}
      maskClosable={false}
      onCancel={props.onClose}
      afterClose={() => resetPems()}
    >
      <Alert
        className={styles.alert}
        type="info"
        message={
          <ul>
            <li>
              <a href="/" target="_blank">
                How to use the certificate
              </a>
            </li>
            <li>
              The certificate is the certificate for the user to interact with
              the system, please keep it properly
            </li>
          </ul>
        }
      />

      <Card className={styles.card}>
        <FadeTransition in={loading}>
          {loading ? <PemsSkeleton /> : <Pems />}
        </FadeTransition>
      </Card>
    </ScrollableModal>
  );
}
