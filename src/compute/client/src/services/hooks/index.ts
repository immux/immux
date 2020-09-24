import { useEffect, useRef } from 'react';

export function useIsMountedRef() {
  const isMountedRef = useRef<boolean>(false);

  useEffect(() => {
    isMountedRef.current = true;

    return () => {
      isMountedRef.current = false;
    };
  });

  return isMountedRef;
}
