import MiddleContainer from '@/containers/Layout/MiddleContainer';
import { Switch, Route, useRouteMatch } from 'react-router-dom';
import HomeAside from '@/containers/Home/Aside';
import PublicFunctions from './Public';
import PersonalFunctions from './Personal';

import NotMatch from '@/components/NotMatch';

import React from 'react';

/**
 * MarketIndex
 * @constructor
 */
export default function MarketIndex() {
  const { path } = useRouteMatch();

  return (
    <Switch>
      <Route path={`${path}/personal-functions`} exact>
        <MiddleContainer aside={<HomeAside />}>
          <PersonalFunctions />
        </MiddleContainer>
      </Route>

      <Route path={`${path}/public-functions`} exact>
        <MiddleContainer aside={<HomeAside />}>
          <PublicFunctions />
        </MiddleContainer>
      </Route>

      <Route path="*">
        <NotMatch />
      </Route>
    </Switch>
  );
}
