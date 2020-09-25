import MiddleContainer from '@/containers/Layout/MiddleContainer';
import { Switch, Route, useRouteMatch } from 'react-router-dom';
import NotMatch from '@/components/NotMatch';
import DbTool from './DbTool';
import Cleansing from './Cleansing';
import React from 'react';

export default function Toolbox() {
  const { path } = useRouteMatch();

  return (
    <Switch>
      <Route path={`${path}/jql`}>
        <MiddleContainer>
          <DbTool />
        </MiddleContainer>
      </Route>

      <Route path={`${path}/cleansing`}>
        <MiddleContainer>
          <Cleansing />
        </MiddleContainer>
      </Route>

      <Route path="*">
        <NotMatch />
      </Route>
    </Switch>
  );
}
