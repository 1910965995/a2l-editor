import { PlusOutlined } from '@ant-design/icons';
import { Button, Col, Modal, Row, Space, Table, message } from 'antd';
import type { TableProps } from 'antd';
import { useState } from 'react';
import { DATA_TYPE_SIZE, characteristicSize } from '../../constants';
import { useAppStore, type OdtEditorState } from '../../store';
import type { DaqListConfig, OdtConfig, OdtEntry } from '../../types';
import { EditableHex, EditableNumber } from '../Editable';

interface SourceItem {
  key: string;
  kind: string;
  name: string;
  address: number;
  size: number;
}

export function OdtDialog() {
  const editor = useAppStore((state) => state.odtEditor);

  if (!editor) return null;
  return <OdtEditor editor={editor} />;
}

function OdtEditor({ editor }: { editor: OdtEditorState }) {
  const project = useAppStore((state) => state.project);
  const commitOdt = useAppStore((state) => state.commitOdt);
  const cancelOdt = useAppStore((state) => state.cancelOdt);

  const [draft, setDraft] = useState<DaqListConfig>(() => structuredClone(editor.value));
  const [selectedOdt, setSelectedOdt] = useState(0);
  const [sourceOpen, setSourceOpen] = useState(false);
  const [sourceSelection, setSourceSelection] = useState<string | null>(null);

  const patchDraft = (updater: (current: DaqListConfig) => DaqListConfig) => setDraft(updater);

  const patchOdt = (index: number, updater: (odt: OdtConfig) => OdtConfig) =>
    patchDraft((current) => ({
      ...current,
      odts: current.odts.map((odt, odtIndex) => (odtIndex === index ? updater(odt) : odt)),
    }));

  const currentOdt = draft.odts[selectedOdt];

  const addOdt = () => {
    const nextIndex = draft.odts.length;
    patchDraft((current) => ({
      ...current,
      odts: [...current.odts, { number: nextIndex, entries: [] }],
    }));
    setSelectedOdt(nextIndex);
  };

  const removeOdt = (index: number) => {
    patchDraft((current) => ({
      ...current,
      odts: current.odts.filter((_, odtIndex) => odtIndex !== index),
    }));
    setSelectedOdt((current) => Math.max(0, Math.min(current, draft.odts.length - 2)));
  };

  const addEntry = () => {
    if (!currentOdt) return;
    patchOdt(selectedOdt, (odt) => ({
      ...odt,
      entries: [
        ...odt.entries,
        {
          number: odt.entries.length,
          address: 0,
          addressExtension: 0,
          size: 2,
          bitOffset: 0,
        },
      ],
    }));
  };

  const removeEntry = (entryIndex: number) => {
    patchOdt(selectedOdt, (odt) => ({
      ...odt,
      entries: odt.entries.filter((_, index) => index !== entryIndex),
    }));
  };

  const sourceItems: SourceItem[] = project
    ? [
        ...project.measurements.map((measurement, index) => ({
          key: `m-${index}`,
          kind: '测量量',
          name: measurement.name,
          address: measurement.ecuAddress,
          size: DATA_TYPE_SIZE[measurement.dataType] ?? 2,
        })),
        ...project.characteristics.map((characteristic, index) => ({
          key: `c-${index}`,
          kind: '标定量',
          name: characteristic.name,
          address: characteristic.ecuAddress,
          size: characteristicSize(characteristic.characteristicType, characteristic.deposit),
        })),
      ]
    : [];

  const openSource = () => {
    if (sourceItems.length === 0) {
      message.info('项目中没有已配置的测量量或标定量。');
      return;
    }
    setSourceSelection(sourceItems[0].key);
    setSourceOpen(true);
  };

  const addFromSource = () => {
    const item = sourceItems.find((source) => source.key === sourceSelection);
    if (!item || !currentOdt) return;
    patchOdt(selectedOdt, (odt) => ({
      ...odt,
      entries: [
        ...odt.entries,
        {
          number: odt.entries.length,
          address: item.address,
          addressExtension: 0,
          size: item.size,
          bitOffset: 0,
        },
      ],
    }));
    setSourceOpen(false);
  };

  const odtColumns: TableProps<OdtConfig>['columns'] = [
    {
      title: '编号',
      width: 75,
      render: (_, record, index) => (
        <EditableNumber
          integer
          min={0}
          max={255}
          value={record.number}
          onChange={(value) => patchOdt(index, (odt) => ({ ...odt, number: value }))}
        />
      ),
    },
    {
      title: '条目数',
      width: 70,
      align: 'center',
      render: (_, record) => record.entries.length,
    },
    {
      title: '',
      width: 60,
      render: (_, __, index) => (
        <Button type="link" size="small" danger onClick={() => removeOdt(index)}>
          删除
        </Button>
      ),
    },
  ];

  const entryColumns: TableProps<OdtEntry>['columns'] = [
    {
      title: '编号',
      width: 75,
      render: (_, record, index) => (
        <EditableNumber
          integer
          min={0}
          max={255}
          value={record.number}
          onChange={(value) =>
            patchOdt(selectedOdt, (odt) => ({
              ...odt,
              entries: odt.entries.map((entry, entryIndex) =>
                entryIndex === index ? { ...entry, number: value } : entry,
              ),
            }))
          }
        />
      ),
    },
    {
      title: '地址',
      width: 130,
      render: (_, record, index) => (
        <EditableHex
          value={record.address}
          onChange={(value) =>
            patchOdt(selectedOdt, (odt) => ({
              ...odt,
              entries: odt.entries.map((entry, entryIndex) =>
                entryIndex === index ? { ...entry, address: value } : entry,
              ),
            }))
          }
        />
      ),
    },
    {
      title: '地址扩展',
      width: 95,
      render: (_, record, index) => (
        <EditableNumber
          integer
          min={0}
          max={255}
          value={record.addressExtension}
          onChange={(value) =>
            patchOdt(selectedOdt, (odt) => ({
              ...odt,
              entries: odt.entries.map((entry, entryIndex) =>
                entryIndex === index ? { ...entry, addressExtension: value } : entry,
              ),
            }))
          }
        />
      ),
    },
    {
      title: '大小',
      width: 80,
      render: (_, record, index) => (
        <EditableNumber
          integer
          min={0}
          max={255}
          value={record.size}
          onChange={(value) =>
            patchOdt(selectedOdt, (odt) => ({
              ...odt,
              entries: odt.entries.map((entry, entryIndex) =>
                entryIndex === index ? { ...entry, size: value } : entry,
              ),
            }))
          }
        />
      ),
    },
    {
      title: '位偏移',
      width: 85,
      render: (_, record, index) => (
        <EditableNumber
          integer
          min={0}
          max={255}
          value={record.bitOffset}
          onChange={(value) =>
            patchOdt(selectedOdt, (odt) => ({
              ...odt,
              entries: odt.entries.map((entry, entryIndex) =>
                entryIndex === index ? { ...entry, bitOffset: value } : entry,
              ),
            }))
          }
        />
      ),
    },
    {
      title: '',
      width: 60,
      render: (_, __, index) => (
        <Button type="link" size="small" danger onClick={() => removeEntry(index)}>
          删除
        </Button>
      ),
    },
  ];

  const sourceColumns: TableProps<SourceItem>['columns'] = [
    { title: '类型', dataIndex: 'kind', width: 90 },
    { title: '名称', dataIndex: 'name' },
    {
      title: '地址',
      dataIndex: 'address',
      width: 110,
      render: (value: number) => `0x${value.toString(16).toUpperCase()}`,
    },
    { title: '大小', dataIndex: 'size', width: 70 },
  ];

  return (
    <>
      <Modal
        open
        title={`DAQ ${draft.number} 的 ODT 配置`}
        width={1000}
        okText="确定"
        cancelText="取消"
        onOk={() => void commitOdt(draft)}
        onCancel={cancelOdt}
        destroyOnHidden
        styles={{ body: { maxHeight: '70vh', overflowY: 'auto' } }}
      >
        <Row gutter={12}>
          <Col span={7}>
            <div className="tab-toolbar">
              <span className="tab-toolbar-title">ODT 列表</span>
              <Button size="small" type="primary" icon={<PlusOutlined />} onClick={addOdt}>
                添加
              </Button>
            </div>
            <Table<OdtConfig>
              size="small"
              bordered
              rowKey={(record) => String(record.number)}
              columns={odtColumns}
              dataSource={draft.odts}
              pagination={false}
              scroll={{ y: 360 }}
              rowSelection={{
                type: 'radio',
                selectedRowKeys: [String(draft.odts[selectedOdt]?.number ?? -1)],
                onChange: (_, selectedRows) => {
                  const index = draft.odts.indexOf(selectedRows[0]);
                  setSelectedOdt(index >= 0 ? index : 0);
                },
                hideSelectAll: true,
              }}
              onRow={(_, index) => ({
                onClick: () => setSelectedOdt(index ?? 0),
              })}
            />
          </Col>
          <Col span={17}>
            <div className="tab-toolbar">
              <span className="tab-toolbar-title">ODT 条目</span>
              <Space size={6}>
                <Button size="small" icon={<PlusOutlined />} disabled={!currentOdt} onClick={addEntry}>
                  添加条目
                </Button>
                <Button size="small" disabled={!currentOdt} onClick={openSource}>
                  从对象添加
                </Button>
              </Space>
            </div>
            <Table<OdtEntry>
              size="small"
              bordered
              rowKey={(record) => `${record.number}-${record.address}-${record.bitOffset}`}
              columns={entryColumns}
              dataSource={currentOdt?.entries ?? []}
              pagination={false}
              scroll={{ y: 360 }}
              locale={{ emptyText: currentOdt ? '暂无条目' : '请先添加 ODT' }}
            />
          </Col>
        </Row>
      </Modal>

      <Modal
        open={sourceOpen}
        title="从已配置对象添加 ODT 条目"
        width={640}
        okText="添加"
        cancelText="取消"
        okButtonProps={{ disabled: sourceSelection === null }}
        onOk={addFromSource}
        onCancel={() => setSourceOpen(false)}
      >
        <Table<SourceItem>
          size="small"
          rowKey="key"
          columns={sourceColumns}
          dataSource={sourceItems}
          pagination={false}
          scroll={{ y: 320 }}
          rowSelection={{
            type: 'radio',
            selectedRowKeys: sourceSelection ? [sourceSelection] : [],
            onChange: (keys) => setSourceSelection(String(keys[0] ?? '')),
            hideSelectAll: true,
          }}
          onRow={(record) => ({
            onClick: () => setSourceSelection(record.key),
          })}
        />
      </Modal>
    </>
  );
}
