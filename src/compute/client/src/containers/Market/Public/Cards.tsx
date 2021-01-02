import InfiniteScroll from 'react-infinite-scroller';
import { Result, Row, Spin, Tooltip } from 'antd';
import FunctionCard from '@/components/FunctionCard';
import React, { MouseEvent } from 'react';

import {
  MoneyCollectOutlined,
  PayCircleFilled,
  ShoppingCartOutlined,
  DollarOutlined,
} from '@ant-design/icons';

import { useMarketFnCollection } from './hooks';
import { useHistory } from 'react-router-dom';
import styles from './styles.module.less';
import _ from 'lodash';

import {
  getFunctionTitle,
  getFunctionIcon,
  getFunctionLink
} from '@/services/marketFunction';

export default function Cards() {
  const history = useHistory();

  const [
    loading,
    error,
    marketFunctions,
    hasMore,
    fetchMarketFunctions
  ] = useMarketFnCollection();

  if (error) {
    return <Result {...error.props} />;
  }

  const orderFunction = (
    functionId: string,
    event: MouseEvent<HTMLDivElement>
  ) => {
    event.preventDefault();
  };

  const cards = _.map(marketFunctions, (marketFunction) => {
    return (
      <FunctionCard
        key={marketFunction.id}
        className={styles.card}
        type={"volcano"}
        to={getFunctionLink(marketFunction)}
        title={getFunctionTitle(marketFunction)}
        description={marketFunction.description}
        icon={getFunctionIcon(marketFunction)}
        actions={
          <Tooltip title="Order this function">
            <ShoppingCartOutlined
              style={{fontSize: '28px'}}
              onClick={orderFunction.bind(null, marketFunction.id)}
            />
          </Tooltip>
        }
      />
    );
  });

  return (
    <InfiniteScroll
      pageStart={1}
      loadMore={fetchMarketFunctions}
      hasMore={!loading && hasMore}
      loader={<Spin key="spin" className={styles.spin} />}
      initialLoad={false}
      useWindow={false}
      getScrollParent={() => document.querySelector('#container')}
    >
      <Row gutter={16}>
        {cards}
      </Row>
    </InfiniteScroll>
  );
}
