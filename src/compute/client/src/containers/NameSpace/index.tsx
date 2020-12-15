import MiddleContainer from '@/containers/Layout/MiddleContainer';
import { Switch, Route, useRouteMatch } from 'react-router-dom';
import HomeAside from '@/containers/Home/Aside';
import CreateNameSpaceForm from './CreateForm';
import NameSpaceCollection from './Collection';
import NotMatch from '@/components/NotMatch';
import NameSpaceProfile from './Profile';

import React from 'react';

/**
 * NameSpaceIndex
 * @constructor
 */
export default function NameSpaceIndex() {
  const { path } = useRouteMatch();

  return (
    <Switch>
      <Route path={`${path}/edit/:id`} exact>
        <CreateNameSpaceForm />
      </Route>

      <Route path={`${path}/new`} exact>
        <CreateNameSpaceForm />
      </Route>

      <Route path={`${path}/:nameSpaceId([0-9a-fA-F]{24})`}>
        <NameSpaceProfile />
      </Route>

      <Route path={path} exact>
        <MiddleContainer aside={<HomeAside />}>
          <NameSpaceCollection />
        </MiddleContainer>
      </Route>

      <Route path="*">
        <NotMatch />
      </Route>
    </Switch>
  );
}
