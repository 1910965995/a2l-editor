import { Button, Checkbox, Space, Table } from 'antd';
import type { TableProps } from 'antd';
import type { CSSProperties } from 'react';
import { useAppStore } from '../store';
import { A2L_DATA_TYPES, type Measurement } from '../types';
import { EditableHex, EditableNumber, EditableSelect, EditableText } from './Editable';
import { useFillTable } from './useFillTable';

const DATA_TYPE_OPTIONS = A2L_DATA_TYPES.map((value) => ({ label: value, value }));

export function MeasurementTable() {
  const project = useAppStore((state) => state.project);
  const patchMeasurement = useAppStore((state) => state.patchMeasurement);
  const removeMeasurement = useAppStore((state) => state.removeMeasurement);
  const beginEditMeasurement = useAppStore((state) => state.beginEditMeasurement);
  const { ref, bodyHeight } = useFillTable();

  if (!project) return null;

  const columns: TableProps<Measurement>['columns'] = [
    {
      title: '启用',
      width: 55,
      align: 'center',
      render: (_, record, index) => (
        <Checkbox
          checked={record.isEnabled}
          onChange={(event) => patchMeasurement(index, { isEnabled: event.target.checked })}
        />
      ),
    },
    {
      title: '名称',
      width: 170,
      render: (_, record, index) => (
        <EditableText value={record.name} onChange={(value) => patchMeasurement(index, { name: value })} />
      ),
    },
    {
      title: '类型',
      width: 120,
      render: (_, record, index) => (
        <EditableSelect
          value={record.dataType}
          options={DATA_TYPE_OPTIONS}
          onChange={(value) => patchMeasurement(index, { dataType: value })}
        />
      ),
    },
    {
      title: '地址',
      width: 110,
      render: (_, record, index) => (
        <EditableHex value={record.ecuAddress} onChange={(value) => patchMeasurement(index, { ecuAddress: value })} />
      ),
    },
    {
      title: '单位',
      width: 80,
      render: (_, record, index) => (
        <EditableText value={record.physUnit} onChange={(value) => patchMeasurement(index, { physUnit: value })} />
      ),
    },
    {
      title: '转换方式',
      width: 130,
      render: (_, record, index) => (
        <EditableText value={record.conversion} onChange={(value) => patchMeasurement(index, { conversion: value })} />
      ),
    },
    {
      title: '最小值',
      width: 95,
      render: (_, record, index) => (
        <EditableNumber value={record.lowerLimit} onChange={(value) => patchMeasurement(index, { lowerLimit: value })} />
      ),
    },
    {
      title: '最大值',
      width: 95,
      render: (_, record, index) => (
        <EditableNumber value={record.upperLimit} onChange={(value) => patchMeasurement(index, { upperLimit: value })} />
      ),
    },
    {
      title: '分辨率',
      width: 80,
      render: (_, record, index) => (
        <EditableNumber value={record.resolution} onChange={(value) => patchMeasurement(index, { resolution: value })} />
      ),
    },
    {
      title: '精度',
      width: 75,
      render: (_, record, index) => (
        <EditableNumber value={record.accuracy} onChange={(value) => patchMeasurement(index, { accuracy: value })} />
      ),
    },
    {
      title: '显示格式',
      width: 95,
      render: (_, record, index) => (
        <EditableText value={record.format} onChange={(value) => patchMeasurement(index, { format: value })} />
      ),
    },
    {
      title: '描述',
      width: 200,
      render: (_, record, index) => (
        <EditableText
          value={record.longIdentifier}
          onChange={(value) => patchMeasurement(index, { longIdentifier: value })}
        />
      ),
    },
    {
      title: '操作',
      width: 110,
      fixed: 'right',
      render: (_, __, index) => (
        <Space size={0}>
          <Button type="link" size="small" onClick={() => beginEditMeasurement(index)}>
            编辑
          </Button>
          <Button type="link" size="small" danger onClick={() => removeMeasurement(index)}>
            删除
          </Button>
        </Space>
      ),
    },
  ];

  return (
    <div
      ref={ref}
      className="table-fill"
      style={{ '--table-body-height': `${bodyHeight}px` } as CSSProperties}
    >
      <Table<Measurement>
        className="data-table"
        size="small"
        bordered
        rowKey={(record) => record.name}
        columns={columns}
        dataSource={project.measurements}
        pagination={false}
        scroll={{ x: 1500, y: bodyHeight }}
      />
    </div>
  );
}
