import InfiniteScroll from 'react-infinite-scroller';
import { Result, Row, Spin, Tooltip } from 'antd';
import ProjectCard from '@/components/ProjectCard';
import React, { MouseEvent } from 'react';

import {
  AppstoreAddOutlined,
  SettingOutlined,
  PlusOutlined
} from '@ant-design/icons';

import { useNameSpaceCollection } from './hooks';
import { useHistory } from 'react-router-dom';
import styles from './styles.module.less';
import _ from 'lodash';

import {
  getNameSpaceTitle,
  getNameSpaceIcon,
  getNameSpaceLink
} from '@/services/nameSpace';

export default function Cards() {
  const history = useHistory();

  const [
    loading,
    error,
    nameSpaces,
    hasMore,
    fetchNameSpaces
  ] = useNameSpaceCollection();

  if (error) {
    return <Result {...error.props} />;
  }

  const redirectToEditNameSpace = (
    nameSpaceId: string,
    event: MouseEvent<HTMLDivElement>
  ) => {
    event.preventDefault();
    history.push(`/name-space/edit/${nameSpaceId}`);
  };

  const cards = _.map(nameSpaces, (nameSpace) => {
    return (
      <ProjectCard
        key={nameSpace.id}
        className={styles.card}
        type={nameSpace.root ? 'volcano' : 'default'}
        to={getNameSpaceLink(nameSpace)}
        title={getNameSpaceTitle(nameSpace, false)}
        description={nameSpace.description}
        icon={getNameSpaceIcon(nameSpace)}
        actions={
          <Tooltip title="edit project info">
            <SettingOutlined
              onClick={redirectToEditNameSpace.bind(null, nameSpace.id)}
            />
          </Tooltip>
        }
      />
    );
  });

  return (
    <InfiniteScroll
      pageStart={1}
      loadMore={fetchNameSpaces}
      hasMore={!loading && hasMore}
      loader={<Spin key="spin" className={styles.spin} />}
      initialLoad={false}
      useWindow={false}
      getScrollParent={() => document.querySelector('#container')}
    >
      <Row gutter={16}>
        <ProjectCard
          type="primary"
          title="new project"
          to="/name-space/new"
          icon={<AppstoreAddOutlined />}
          extraIcon={<PlusOutlined />}
        />
        {cards}
      </Row>
    </InfiniteScroll>
  );
}
