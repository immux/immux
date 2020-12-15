import { toMoment } from '@/utils';
import React from 'react';

import styles from './styles.module.less';

export default function FormattedDate(props: {
  value?: Parameters<typeof toMoment>[0];
}) {
  const instance = toMoment(props.value);

  return (
    <span className={styles.formattedDate}>
      {instance ? (
        <>
          {instance.format('YYYY-MM-DD')}
          <small>{instance.format('HH:mm')}</small>
        </>
      ) : (
        '-'
      )}
    </span>
  );
}
