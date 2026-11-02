import { useLayoutEffect, useRef, useState } from 'react';

export function useFillTable() {
  const ref = useRef<HTMLDivElement>(null);
  const [bodyHeight, setBodyHeight] = useState(200);

  useLayoutEffect(() => {
    const element = ref.current;
    if (!element) return;

    const observer = new ResizeObserver(() => {
      const header = element.querySelector<HTMLElement>('.ant-table-header');
      const headerHeight = header?.offsetHeight ?? 0;
      const next = Math.max(60, element.clientHeight - headerHeight - 1);
      setBodyHeight((current) => (current === next ? current : next));
    });

    observer.observe(element);
    return () => observer.disconnect();
  }, []);

  return { ref, bodyHeight };
}
