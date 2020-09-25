import { useCallback, useState, RefObject, useEffect } from 'react';

/**
 * ModalActions
 */
export function useModalActions(defaultVisible = false) {
  const [visible, setVisible] = useState(defaultVisible);
  const open = useCallback(() => setVisible(true), [setVisible]);
  const close = useCallback(() => setVisible(false), [setVisible]);

  return [visible, open, close] as const;
}

export interface ResizeObserverEntry {
  target: HTMLElement;
  contentRect: DOMRectReadOnly;
}
export type ObserverCallback = (entry: DOMRectReadOnly) => void;

export const useResizeObserver = (
  ref: RefObject<HTMLElement>,
  callback: ObserverCallback
) => {
  useEffect(() => {
    if (ref.current) {
      const resizeObserver = new (window as any).ResizeObserver(
        (entries: ResizeObserverEntry[]) => {
          callback(entries[0].contentRect);
        }
      );

      resizeObserver.observe(ref.current);

      return () => {
        resizeObserver.disconnect();
      };
    }
  }, [ref]);
};
