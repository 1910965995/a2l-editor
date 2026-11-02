import { PlusOutlined } from '@ant-design/icons';
import { Button, Space, Table, Tag } from 'antd';
import type { TableProps } from 'antd';
import type { CSSProperties } from 'react';
import { useAppStore } from '../store';
import type { DaqListConfig } from '../types';
import { EditableNumber, EditableSelect } from './Editable';
import { useFillTable } from './useFillTable';

export function DaqListTable() {
  const project = useAppStore((state) => state.project);
  const patchDaqList = useAppStore((state) => state.patchDaqList);
  const removeDaqList = useAppStore((state) => state.removeDaqList);
  const addDaqList = useAppStore((state) => state.addDaqList);
  const openOdt = useAppStore((state) => state.openOdt);
  const { ref, bodyHeight } = useFillTable();

  if (!project) return null;

  const eventOptions = project.eventChannels.map((event) => ({
    label: `${event.channelNumber} - ${event.name}`,
    value: event.channelNumber,
  }));

  const columns: TableProps<DaqListConfig>['columns'] = [
    {
      title: '编号',
      width: 70,
      render: (_, record, index) => (
        <EditableNumber
          integer
          value={record.number}
          min={0}
          max={65535}
          onChange={(value) => patchDaqList(index, { number: value })}
        />
      ),
    },
    {
      title: '类型',
      width: 110,
      render: (_, record) => <Tag color={record.type === 'DAQ' ? 'blue' : 'purple'}>{record.type}</Tag>,
    },
    {
      title: '绑定事件',
      width: 210,
      render: (_, record, index) => (
        <EditableSelect
          value={record.eventFixed}
          options={eventOptions}
          onChange={(value) => {
            const event = project.eventChannels.find((item) => item.channelNumber === value);
            patchDaqList(index, { eventFixed: value, ...(event ? { type: event.type } : {}) });
          }}
        />
      ),
    },
    {
      title: '起始PID',
      width: 100,
      render: (_, record, index) => (
        <EditableNumber
          integer
          value={record.firstPid}
          min={0}
          max={255}
          onChange={(value) => patchDaqList(index, { firstPid: value })}
        />
      ),
    },
    {
      title: '最大ODT数',
      width: 110,
      render: (_, record, index) => (
        <EditableNumber
          integer
          value={record.maxOdt}
          min={0}
          max={255}
          onChange={(value) => patchDaqList(index, { maxOdt: value })}
        />
      ),
    },
    {
      title: 'ODT最大条目数',
      width: 130,
      render: (_, record, index) => (
        <EditableNumber
          integer
          value={record.maxOdtEntries}
          min={0}
          max={255}
          onChange={(value) => patchDaqList(index, { maxOdtEntries: value })}
        />
      ),
    },
    {
      title: 'ODT 数量',
      width: 90,
      align: 'center',
      render: (_, record) => <span>{record.odts.length}</span>,
    },
    {
      title: '操作',
      width: 150,
      fixed: 'right',
      render: (_, __, index) => (
        <Space size={0}>
          <Button type="link" size="small" onClick={() => openOdt(index)}>
            配置ODT
          </Button>
          <Button type="link" size="small" danger onClick={() => removeDaqList(index)}>
            删除
          </Button>
        </Space>
      ),
    },
  ];

  return (
    <div className="tab-panel">
      <div className="tab-toolbar">
        <Space size={8}>
          <span className="tab-toolbar-title">DAQ_LIST 列表</span>
          <Tag color={project.daq.mode === 'STATIC' ? 'green' : 'blue'}>{project.daq.mode}</Tag>
        </Space>
        <Button size="small" type="primary" icon={<PlusOutlined />} onClick={addDaqList}>
          添加DAQ
        </Button>
      </div>
      <div
        ref={ref}
        className="table-fill"
        style={{ '--table-body-height': `${bodyHeight}px` } as CSSProperties}
      >
        <Table<DaqListConfig>
          className="data-table"
          size="small"
          bordered
          rowKey={(record) => String(record.number)}
          columns={columns}
          dataSource={project.daqLists}
          pagination={false}
          scroll={{ x: 1000, y: bodyHeight }}
        />
      </div>
    </div>
  );
}
