import React, { useCallback } from 'react';
import FileSaver from 'file-saver';
import { Button } from 'antd';

import { useStoreState } from '@/store/hooks';
import { deployEnv } from '@/constants';

const publicPemFilename = (() => {
  switch (deployEnv) {
    case 'release': {
      return 'compute_rsa.pub';
    }

    case 'beta': {
      return 'compute_beta_rsa.pub';
    }

    default: {
      return 'compute_dev_rsa.pub';
    }
  }
})();

export default function SaveButton() {
  const publicPem = useStoreState((state) => state.account.pems.publicPem);
  const onClick = useCallback(() => {
    FileSaver.saveAs(
      new Blob([publicPem], { type: 'text/plain;charset=utf-8' }),
      publicPemFilename
    );
  }, [publicPem]);

  return <Button onClick={onClick}>download</Button>;
}
