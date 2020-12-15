import { Card, Button, message } from 'antd';
import React, { useCallback, useState } from 'react';
import { cleansingFileSpaceOrigin } from '@/services/api/cleansing';
import { catchError } from '@/utils';

function CleansingFileSpaceOriginButton() {
  const [loading, setLoading] = useState(false);

  const cleansing = useCallback(async () => {
    if (loading) {
      return;
    }

    try {
      setLoading(true);

      const total = await cleansingFileSpaceOrigin();

      message.success(
        <>
          clean finished total <code>{total}</code> <code>db</code>
        </>
      );
    } catch (err) {
      catchError(err);
    } finally {
      setLoading(false);
    }
  }, [loading, setLoading]);

  return (
    <Button loading={loading} onClick={cleansing}>
      clean <code>db</code>
    </Button>
  );
}

export default function Cleansing() {
  return (
    <Card title={<code>DataBase</code>}>
      <CleansingFileSpaceOriginButton />
    </Card>
  );
}
