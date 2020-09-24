import { TransitionGroup, CSSTransition } from 'react-transition-group';
import React, { ReactNode } from 'react';

import classNames from 'classnames';
import styles from './styles.module.less';

export default function FadeTransition(props: {
  children: ReactNode;
  className?: string;
  in: boolean;
}) {
  return (
    <TransitionGroup className={classNames(styles.wrapper, props.className)}>
      <CSSTransition
        key={props.in ? 'element-1' : 'element-2'}
        classNames="fade"
        addEndListener={(node, done) => {
          node.addEventListener('transitionend', done, false);
        }}
      >
        {props.children}
      </CSSTransition>
    </TransitionGroup>
  );
}
