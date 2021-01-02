import { DeploymentUnitOutlined, AppstoreOutlined } from '@ant-design/icons';
import { MarketFunction, Account } from '@/types/models';
import React from 'react';

import _ from 'lodash';

/**
 * @param marketFunction
 */
export function getFunctionLink(marketFunction: MarketFunction) {
  return `/public-functions/${marketFunction.id}`;
}

/**
 * @param nameSpace
 */
export function getFunctionIcon(marketFunction: MarketFunction) {
  return marketFunction.marketStatus ? <DeploymentUnitOutlined /> : <AppstoreOutlined />;
}

/**
 * @param nameSpace
 * @param rootIcon
 */
export function getFunctionTitle(marketFunction: MarketFunction) {
  const name = <code>{marketFunction.name}</code>;

  return name;
}
