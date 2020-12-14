import Cards from '@/containers/NameSpace/Collection/Cards';
import { Tabs, Badge } from 'antd';
import React from 'react';

import { useStoreState } from '@/store/hooks';

/**
 * PublicFunction
 * @constructor
 */
export default function PublicFunction() {
  const total = useStoreState((state) => state.nameSpace.collection.total);

  return (
    <Tabs>
      <Tabs.TabPane
        key="total"
        tab={
          <>
            All projects
            <Badge count={total} />
          </>
        }
      >
        <Cards />
      </Tabs.TabPane>
    </Tabs>
  );
}
