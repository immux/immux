import { Skeleton } from 'antd';
import React from 'react';

import styles from './styles.module.less';

export default function PemsSkeleton() {
  return (
    <article className={styles.pems}>
      <header>
        <Skeleton.Avatar active size="large" shape="square" />
      </header>

      <main>
        <Skeleton paragraph={{ rows: 2 }} />
      </main>

      <footer>
        <Skeleton.Button active />
        <Skeleton.Button active />
      </footer>
    </article>
  );
}
