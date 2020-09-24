import { StoreProvider } from 'easy-peasy';
import { Router } from 'react-router-dom';
import { ConfigProvider } from 'antd';
import ReactDOM from 'react-dom';
import React from 'react';
import App from './App';

import * as serviceWorker from './serviceWorker';
import zh_CN from 'antd/es/locale/zh_CN';
import { history } from '@/utils';
import { store } from '@/store';

import './index.less';

ReactDOM.render(
  <ConfigProvider locale={zh_CN}>
    <StoreProvider store={store}>
      <Router history={history}>
        <App />
      </Router>
    </StoreProvider>
  </ConfigProvider>,
  document.getElementById('root')
);

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();
