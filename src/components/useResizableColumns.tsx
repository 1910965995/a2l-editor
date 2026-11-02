import type { TableProps } from 'antd';
import { useState } from 'react';
import { ResizableHeaderCell } from './ResizableHeaderCell';

type Column<T> = NonNullable<TableProps<T>['columns']>[number];

export function useResizableColumns<T>(columns: TableProps<T>['columns']) {
  const [widths, setWidths] = useState<Record<string, number>>({});

  const resizableColumns = (columns ?? []).map((column, index) => {
    const key = String(column.key ?? (typeof column.title === 'string' ? column.title : index));
    const initialWidth = typeof column.width === 'number' ? column.width : undefined;
    const width = widths[key] ?? initialWidth;

    return {
      ...column,
      key,
      width,
      onHeaderCell: () => ({
        width,
        onResize: (nextWidth: number) => {
          setWidths((current) => ({ ...current, [key]: Math.max(40, Math.round(nextWidth)) }));
        },
      }),
    } as Column<T>;
  });

  const totalWidth = resizableColumns.reduce(
    (sum, column) => sum + (typeof column.width === 'number' ? column.width : 0),
    0,
  );

  return {
    columns: resizableColumns,
    totalWidth,
    components: { header: { cell: ResizableHeaderCell } } as unknown as TableProps<T>['components'],
  };
}
