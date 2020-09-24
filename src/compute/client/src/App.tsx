import { AnimatedSwitch } from 'react-router-transition';
import NameSpaceIndex from './containers/NameSpace';
import { Router, Route } from 'react-router-dom';
import Login from './containers/Account/Login';
import SystemLayout from './containers/Layout';
import Toolbox from './containers/Toolbox';
import Home from './containers/Home';
import React from 'react';

import styles from './App.module.less';
import { history } from '@/utils';

function App() {
  return (
    <Router history={history}>
      <AnimatedSwitch
        className={styles.switchWrapper}
        atEnter={{ opacity: 0 }}
        atLeave={{ opacity: 0 }}
        atActive={{ opacity: 1 }}
      >
        <SystemLayout>
          <Route path="/" exact>
            <Home />
          </Route>

          <Route path="/account/login" exact>
            <Login />
          </Route>

          <Route path="/name-space">
            <NameSpaceIndex />
          </Route>

          <Route path="/toolbox">
            <Toolbox />
          </Route>
        </SystemLayout>
      </AnimatedSwitch>
    </Router>
  );
}

export default App;
