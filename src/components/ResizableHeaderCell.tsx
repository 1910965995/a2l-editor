import type { PointerEvent as ReactPointerEvent, ThHTMLAttributes } from 'react';

export interface ResizableHeaderCellProps extends ThHTMLAttributes<HTMLTableCellElement> {
  width?: number;
  onResize?: (width: number) => void;
}

export function ResizableHeaderCell({ width, onResize, children, ...rest }: ResizableHeaderCellProps) {
  const handlePointerDown = (event: ReactPointerEvent<HTMLSpanElement>) => {
    if (!onResize || width === undefined) return;

    event.preventDefault();
    event.stopPropagation();

    const startX = event.clientX;
    const startWidth = width;
    const previousUserSelect = document.body.style.userSelect;
    document.body.style.userSelect = 'none';
    document.body.style.cursor = 'col-resize';

    const handleMove = (moveEvent: PointerEvent) => {
      onResize(Math.max(40, startWidth + moveEvent.clientX - startX));
    };

    const handleUp = () => {
      document.removeEventListener('pointermove', handleMove);
      document.removeEventListener('pointerup', handleUp);
      document.body.style.userSelect = previousUserSelect;
      document.body.style.cursor = '';
    };

    document.addEventListener('pointermove', handleMove);
    document.addEventListener('pointerup', handleUp);
  };

  return (
    <th {...rest}>
      {children}
      {width !== undefined && onResize && (
        <span className="col-resize-handle" onPointerDown={handlePointerDown} />
      )}
    </th>
  );
}
