import React, { createContext, ReactNode, useRef } from 'react';
import { findDOMNode } from 'react-dom';
import { Layout } from 'antd';

import styles from './styles.module.less';
import classNames from 'classnames';

type DOMNode = ReturnType<typeof findDOMNode>;

export const ContentElementContext = createContext<DOMNode>(null);
export const NoticeElementContext = createContext<DOMNode>(null);

export interface FullscreenContainerProps {
  header?: ReactNode;
  children?: ReactNode;
  aside?: ReactNode;
  className?: string;
  notice?: ReactNode;
}

function FullscreenContainer(props: FullscreenContainerProps) {
  const refContent = useRef<any>(null);
  const contentEl = findDOMNode(refContent.current);
  const refNotice = useRef<HTMLDivElement>(null);
  const noticeEl = findDOMNode(refNotice.current);

  const heightStyle: { height?: string } = {};

  if (noticeEl) {
    heightStyle.height = `calc(100vh - ${
      90 + (noticeEl as HTMLDivElement).clientHeight
    }px)`;
  }

  return (
    <ContentElementContext.Provider value={contentEl}>
      <NoticeElementContext.Provider value={noticeEl}>
        <Layout className={styles.fullscreenContainer}>
          {props.notice && <div ref={refNotice}>{props.notice}</div>}
          {props.header && <Layout.Header>{props.header}</Layout.Header>}

          <Layout.Content>
            <Layout>
              {props.aside && (
                <Layout.Sider width={256} style={heightStyle}>
                  <div className={styles.containerAside}>{props.aside}</div>
                </Layout.Sider>
              )}

              <Layout.Content
                ref={refContent}
                className={classNames(styles.containerContent, props.className)}
                style={heightStyle}
              >
                {props.children}
              </Layout.Content>
            </Layout>
          </Layout.Content>
        </Layout>
      </NoticeElementContext.Provider>
    </ContentElementContext.Provider>
  );
}

FullscreenContainer.Header = function FullscreenContainerHeader(props: {
  actions?: ReactNode;
  children?: ReactNode;
}) {
  return (
    <nav className={styles.containerHeader}>
      <section className={styles.containerHeaderBody}>{props.children}</section>
      <section className={styles.containerHeaderActions}>
        {props.actions}
      </section>
    </nav>
  );
};

export default FullscreenContainer;
