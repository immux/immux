import FormattedDate from '@/components/FormateedDate';
import AccountLink from '@/components/AccountLink';
import { ComputedNameSpace } from '@/types/models';
import { AsideCard } from '@/components/Aside';
import React, { useState } from 'react';
import { Drawer } from 'antd';

import styles from './styles.module.less';

export default function NameSpaceAside(props: {
  nameSpace?: ComputedNameSpace;
  visible: boolean;
  setVisible: React.Dispatch<React.SetStateAction<boolean>>;
}) {
  if (!props.nameSpace) {
    return null;
  }

  return (
    <Drawer
      className={styles.drawer}
      maskStyle={{ background: 'transparent' }}
      title="project info"
      placement="right"
      width={320}
      visible={props.visible}
      destroyOnClose={true}
      onClose={props.setVisible.bind(null, false)}
    >
      {props.nameSpace.title && (
        <AsideCard title="title">{props.nameSpace.title}</AsideCard>
      )}

      {props.nameSpace.description && (
        <AsideCard title="description">{props.nameSpace.description}</AsideCard>
      )}

      <AsideCard title="creator">
        <AccountLink className={styles.date} {...props.nameSpace.creator} />
      </AsideCard>

      <AsideCard title="createAt">
        <FormattedDate value={props.nameSpace.createAt} />
      </AsideCard>
    </Drawer>
  );
}
