import Cards from './Cards';
import { Tabs, Badge } from 'antd';
import React from 'react';

import { useStoreState } from '@/store/hooks';

/**
 * PublicFunction
 * @constructor
 */
export default function PublicFunction() {
  const total = useStoreState((state) => state.functions.publicTotal);

  return (
    <Tabs>
      <Tabs.TabPane
        key="total"
        tab={
          <>
            Function Market
            <Badge count={total} />
          </>
        }
      >
        <Cards />
      </Tabs.TabPane>
    </Tabs>
  );
}
