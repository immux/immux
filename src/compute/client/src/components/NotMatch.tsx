import { ButtonProps } from 'antd/es/button';
import { Button, Result } from 'antd';
import React from 'react';

import { useHistory } from 'react-router-dom';
import styles from './styles.module.less';
import classNames from 'classnames';
export default function NotMatch(props: { header: boolean }) {
  const history = useHistory();

  const redirectToHome: ButtonProps['onClick'] = (event) => {
    event.preventDefault();
    history.push('/');
  };

  return (
    <Result
      className={classNames(styles.notMatch, { [styles.header]: props.header })}
      status={404}
      title="OOPS!"
      subTitle="You visited a page that does not exist"
      extra={
        <Button type="primary" href="/" onClick={redirectToHome}>
          go home
        </Button>
      }
    />
  );
}

NotMatch.defaultProps = {
  header: true
};
