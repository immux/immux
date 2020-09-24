import { ModalProps } from 'antd/lib/modal';
import React, { ReactNode } from 'react';
import { Modal } from 'antd';

import styles from './styles.module.less';
import classNames from 'classnames';

export interface ScrollableModalProps
  extends Omit<ModalProps, 'width' | 'centered' | 'footer' | 'destroyOnClose'> {
  children?: ReactNode;
}

export default function ScrollableModal(props: ScrollableModalProps) {
  return (
    <Modal
      {...props}
      wrapClassName={classNames(props.wrapClassName, styles.modal)}
      width=""
      centered
      footer={null}
      destroyOnClose
    />
  );
}
