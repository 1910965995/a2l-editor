import { useLayoutEffect, useRef, useState } from 'react';

export function useFillTable() {
  const ref = useRef<HTMLDivElement>(null);
  const [bodyHeight, setBodyHeight] = useState(200);

  useLayoutEffect(() => {
    const element = ref.current;
    if (!element) return;

    const update = () => {
      const header = element.querySelector<HTMLElement>('.ant-table-header');
      const headerHeight = header?.offsetHeight ?? 0;
      const next = Math.max(60, element.clientHeight - headerHeight - 1);
      setBodyHeight((current) => (current === next ? current : next));
    };

    const observer = new ResizeObserver(update);
    observer.observe(element);
    const timer = window.setTimeout(update, 0);

    return () => {
      window.clearTimeout(timer);
      observer.disconnect();
    };
  }, []);

  return { ref, bodyHeight };
}
