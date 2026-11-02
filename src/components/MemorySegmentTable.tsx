import { PlusOutlined } from '@ant-design/icons';
import { Button, Space, Table } from 'antd';
import type { TableProps } from 'antd';
import type { CSSProperties } from 'react';
import { useAppStore } from '../store';
import {
  MEM_ATTRIBUTES,
  MEM_MEMORY_TYPES,
  PRG_TYPES,
  type MemorySegment,
} from '../types';
import { EditableHex, EditableNumber, EditableSelect, EditableText } from './Editable';
import { useFillTable } from './useFillTable';

const PRG_TYPE_OPTIONS = PRG_TYPES.map((value) => ({ label: value, value }));
const MEMORY_TYPE_OPTIONS = MEM_MEMORY_TYPES.map((value) => ({ label: value, value }));
const ATTRIBUTE_OPTIONS = MEM_ATTRIBUTES.map((value) => ({ label: value, value }));

export function MemorySegmentTable() {
  const project = useAppStore((state) => state.project);
  const patchMemorySegment = useAppStore((state) => state.patchMemorySegment);
  const removeMemorySegment = useAppStore((state) => state.removeMemorySegment);
  const beginAddMemorySegment = useAppStore((state) => state.beginAddMemorySegment);
  const beginEditMemorySegment = useAppStore((state) => state.beginEditMemorySegment);
  const { ref, bodyHeight } = useFillTable();

  if (!project) return null;

  const offsetColumn = (title: string, key: 'offset0' | 'offset1' | 'offset2' | 'offset3' | 'offset4') => ({
    title,
    width: 70,
    render: (_: unknown, record: MemorySegment, index: number) => (
      <EditableNumber
        integer
        value={record[key]}
        onChange={(value) => patchMemorySegment(index, { [key]: value })}
      />
    ),
  });

  const columns: TableProps<MemorySegment>['columns'] = [
    {
      title: '名称',
      width: 130,
      render: (_, record, index) => (
        <EditableText value={record.name} onChange={(value) => patchMemorySegment(index, { name: value })} />
      ),
    },
    {
      title: '描述',
      width: 180,
      render: (_, record, index) => (
        <EditableText
          value={record.longIdentifier}
          onChange={(value) => patchMemorySegment(index, { longIdentifier: value })}
        />
      ),
    },
    {
      title: '程序类型',
      width: 160,
      render: (_, record, index) => (
        <EditableSelect
          value={record.prgType}
          options={PRG_TYPE_OPTIONS}
          onChange={(value) => patchMemorySegment(index, { prgType: value })}
        />
      ),
    },
    {
      title: '内存类型',
      width: 120,
      render: (_, record, index) => (
        <EditableSelect
          value={record.memoryType}
          options={MEMORY_TYPE_OPTIONS}
          onChange={(value) => patchMemorySegment(index, { memoryType: value })}
        />
      ),
    },
    {
      title: '属性',
      width: 95,
      render: (_, record, index) => (
        <EditableSelect
          value={record.attribute}
          options={ATTRIBUTE_OPTIONS}
          onChange={(value) => patchMemorySegment(index, { attribute: value })}
        />
      ),
    },
    {
      title: '起始地址',
      width: 120,
      render: (_, record, index) => (
        <EditableHex value={record.address} onChange={(value) => patchMemorySegment(index, { address: value })} />
      ),
    },
    {
      title: '大小',
      width: 110,
      render: (_, record, index) => (
        <EditableHex value={record.size} onChange={(value) => patchMemorySegment(index, { size: value })} />
      ),
    },
    offsetColumn('偏移0', 'offset0'),
    offsetColumn('偏移1', 'offset1'),
    offsetColumn('偏移2', 'offset2'),
    offsetColumn('偏移3', 'offset3'),
    offsetColumn('偏移4', 'offset4'),
    {
      title: '操作',
      width: 110,
      fixed: 'right',
      render: (_, __, index) => (
        <Space size={0}>
          <Button type="link" size="small" onClick={() => beginEditMemorySegment(index)}>
            编辑
          </Button>
          <Button type="link" size="small" danger onClick={() => removeMemorySegment(index)}>
            删除
          </Button>
        </Space>
      ),
    },
  ];

  return (
    <div className="tab-panel">
      <div className="tab-toolbar">
        <span className="tab-toolbar-title">内存段列表</span>
        <Button size="small" type="primary" icon={<PlusOutlined />} onClick={beginAddMemorySegment}>
          添加内存段
        </Button>
      </div>
      <div
        ref={ref}
        className="table-fill"
        style={{ '--table-body-height': `${bodyHeight}px` } as CSSProperties}
      >
        <Table<MemorySegment>
          className="data-table"
          size="small"
          bordered
          rowKey={(record) => `${record.name}-${record.address}-${record.size}`}
          columns={columns}
          dataSource={project.memorySegments}
          pagination={false}
          scroll={{ x: 1600, y: bodyHeight }}
        />
      </div>
    </div>
  );
}
