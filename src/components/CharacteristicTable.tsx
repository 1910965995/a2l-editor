import { Button, Checkbox, Space, Table } from 'antd';
import type { TableProps } from 'antd';
import type { CSSProperties } from 'react';
import { useAppStore } from '../store';
import { A2L_DATA_TYPES, CHARACTERISTIC_TYPES, type Characteristic } from '../types';
import {
  EditableHex,
  EditableNumber,
  EditableOptionalNumber,
  EditableSelect,
  EditableText,
} from './Editable';
import { useFillTable } from './useFillTable';

const CHARACTERISTIC_TYPE_OPTIONS = CHARACTERISTIC_TYPES.map((value) => ({ label: value, value }));
const DEPOSIT_OPTIONS = A2L_DATA_TYPES.map((value) => ({ label: `__${value}`, value: `__${value}` }));

export function CharacteristicTable() {
  const project = useAppStore((state) => state.project);
  const patchCharacteristic = useAppStore((state) => state.patchCharacteristic);
  const removeCharacteristic = useAppStore((state) => state.removeCharacteristic);
  const beginEditCharacteristic = useAppStore((state) => state.beginEditCharacteristic);
  const { ref, bodyHeight } = useFillTable();

  if (!project) return null;

  const columns: TableProps<Characteristic>['columns'] = [
    {
      title: '启用',
      width: 55,
      align: 'center',
      render: (_, record, index) => (
        <Checkbox
          checked={record.isEnabled}
          onChange={(event) => patchCharacteristic(index, { isEnabled: event.target.checked })}
        />
      ),
    },
    {
      title: '名称',
      width: 170,
      render: (_, record, index) => (
        <EditableText value={record.name} onChange={(value) => patchCharacteristic(index, { name: value })} />
      ),
    },
    {
      title: '特征类型',
      width: 120,
      render: (_, record, index) => (
        <EditableSelect
          value={record.characteristicType}
          options={CHARACTERISTIC_TYPE_OPTIONS}
          onChange={(value) => patchCharacteristic(index, { characteristicType: value })}
        />
      ),
    },
    {
      title: '存储布局',
      width: 130,
      render: (_, record, index) => (
        <EditableSelect
          value={record.deposit}
          options={DEPOSIT_OPTIONS}
          onChange={(value) => patchCharacteristic(index, { deposit: value })}
        />
      ),
    },
    {
      title: '最大步进',
      width: 90,
      render: (_, record, index) => (
        <EditableNumber value={record.maxDiff} onChange={(value) => patchCharacteristic(index, { maxDiff: value })} />
      ),
    },
    {
      title: '地址',
      width: 110,
      render: (_, record, index) => (
        <EditableHex
          value={record.ecuAddress}
          onChange={(value) => patchCharacteristic(index, { ecuAddress: value })}
        />
      ),
    },
    {
      title: '单位',
      width: 80,
      render: (_, record, index) => (
        <EditableText value={record.physUnit} onChange={(value) => patchCharacteristic(index, { physUnit: value })} />
      ),
    },
    {
      title: '转换方式',
      width: 120,
      render: (_, record, index) => (
        <EditableText
          value={record.conversion}
          onChange={(value) => patchCharacteristic(index, { conversion: value })}
        />
      ),
    },
    {
      title: '最小值',
      width: 95,
      render: (_, record, index) => (
        <EditableNumber
          value={record.lowerLimit}
          onChange={(value) => patchCharacteristic(index, { lowerLimit: value })}
        />
      ),
    },
    {
      title: '最大值',
      width: 95,
      render: (_, record, index) => (
        <EditableNumber
          value={record.upperLimit}
          onChange={(value) => patchCharacteristic(index, { upperLimit: value })}
        />
      ),
    },
    {
      title: '扩展最小值',
      width: 105,
      render: (_, record, index) => (
        <EditableOptionalNumber
          value={record.extendedLowerLimit}
          emptyValue={-Number.MAX_VALUE}
          onChange={(value) => patchCharacteristic(index, { extendedLowerLimit: value })}
        />
      ),
    },
    {
      title: '扩展最大值',
      width: 105,
      render: (_, record, index) => (
        <EditableOptionalNumber
          value={record.extendedUpperLimit}
          emptyValue={Number.MAX_VALUE}
          onChange={(value) => patchCharacteristic(index, { extendedUpperLimit: value })}
        />
      ),
    },
    {
      title: '显示格式',
      width: 95,
      render: (_, record, index) => (
        <EditableText value={record.format} onChange={(value) => patchCharacteristic(index, { format: value })} />
      ),
    },
    {
      title: '描述',
      width: 180,
      render: (_, record, index) => (
        <EditableText
          value={record.longIdentifier}
          onChange={(value) => patchCharacteristic(index, { longIdentifier: value })}
        />
      ),
    },
    {
      title: '操作',
      width: 110,
      fixed: 'right',
      render: (_, __, index) => (
        <Space size={0}>
          <Button type="link" size="small" onClick={() => beginEditCharacteristic(index)}>
            编辑
          </Button>
          <Button type="link" size="small" danger onClick={() => removeCharacteristic(index)}>
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
      <Table<Characteristic>
        className="data-table"
        size="small"
        bordered
        rowKey={(record) => record.name}
        columns={columns}
        dataSource={project.characteristics}
        pagination={false}
        scroll={{ x: 1750, y: bodyHeight }}
      />
    </div>
  );
}
