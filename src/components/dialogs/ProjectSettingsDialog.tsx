import { PlusOutlined } from '@ant-design/icons';
import {
  Button,
  Checkbox,
  Col,
  Input,
  InputNumber,
  Modal,
  Row,
  Select,
  Table,
  Tabs,
  Tag,
} from 'antd';
import type { TableProps } from 'antd';
import { useState } from 'react';
import { openFileDialog } from '../../api';
import {
  ADDRESS_EXTENSION_OPTIONS,
  ADDRESS_GRANULARITY_OPTIONS,
  ADDRESS_MODE_OPTIONS,
  BYTE_ORDERS,
  DAQ_GRANULARITY_OPTIONS,
  DAQ_MODE_OPTIONS,
  ID_FIELD_OPTIONS,
  OPTIMISATION_OPTIONS,
  OUT_FILE_FILTERS,
  TIME_UNIT_LABELS,
  TIMESTAMP_SIZE_OPTIONS,
  TIMESTAMP_UNIT_OPTIONS,
  OVERLOAD_OPTIONS,
} from '../../constants';
import { useAppStore } from '../../store';
import { EVENT_TYPES, type A2lProject, type EventChannel } from '../../types';
import { EditableText, HexInput } from '../Editable';

const YES_NO_OPTIONS = [
  { label: '是', value: true },
  { label: '否', value: false },
];

const EVENT_TYPE_OPTIONS = EVENT_TYPES.map((value) => ({ label: value, value }));
const TIME_UNIT_OPTIONS = TIME_UNIT_LABELS.map((label, index) => ({ label, value: index }));

export function ProjectSettingsDialog() {
  const settingsOpen = useAppStore((state) => state.settingsOpen);
  const project = useAppStore((state) => state.project);

  if (!settingsOpen || !project) return null;
  return <ProjectSettingsEditor project={project} />;
}

function ProjectSettingsEditor({ project }: { project: A2lProject }) {
  const closeSettings = useAppStore((state) => state.closeSettings);
  const applySettings = useAppStore((state) => state.applySettings);
  const [draft, setDraft] = useState<A2lProject>(() => structuredClone(project));

  const patchDraft = (updater: (current: A2lProject) => A2lProject) => {
    setDraft(updater);
  };

  const patchXcp = (patch: Partial<A2lProject['xcp']>) =>
    patchDraft((current) => ({ ...current, xcp: { ...current.xcp, ...patch } }));

  const patchDaq = (patch: Partial<A2lProject['daq']>) =>
    patchDraft((current) => ({ ...current, daq: { ...current.daq, ...patch } }));

  const patchEvent = (index: number, patch: Partial<EventChannel>) =>
    patchDraft((current) => {
      const previous = current.eventChannels[index];
      const eventChannels = current.eventChannels.map((item, itemIndex) =>
        itemIndex === index ? { ...item, ...patch } : item,
      );
      let daqLists = current.daqLists;
      if (previous && patch.type && patch.type !== previous.type) {
        daqLists = current.daqLists.map((list) =>
          list.eventFixed === previous.channelNumber ? { ...list, type: patch.type! } : list,
        );
      }
      return { ...current, eventChannels, daqLists };
    });

  const addEvent = () =>
    patchDraft((current) => {
      const index = current.eventChannels.length;
      const event: EventChannel = {
        name: `NewEvent${index + 1}`,
        shortName: `NewEvent${index + 1}`,
        channelNumber: index,
        type: 'DAQ',
        maxDaqList: 1,
        timeCycle: 10,
        timeUnit: 6,
        priority: index,
      };
      return { ...current, eventChannels: [...current.eventChannels, event] };
    });

  const removeEvent = (index: number) =>
    patchDraft((current) => ({
      ...current,
      eventChannels: current.eventChannels.filter((_, itemIndex) => itemIndex !== index),
    }));

  const browseOutFile = async () => {
    const path = await openFileDialog({ filters: OUT_FILE_FILTERS });
    if (path) patchDraft((current) => ({ ...current, outFilePath: path }));
  };

  const eventColumns: TableProps<EventChannel>['columns'] = [
    {
      title: '通道号',
      width: 80,
      render: (_, record, index) => (
        <InputNumber
          size="small"
          min={0}
          max={65535}
          value={record.channelNumber}
          onChange={(value) => patchEvent(index, { channelNumber: value ?? 0 })}
          style={{ width: '100%' }}
        />
      ),
    },
    {
      title: '名称',
      width: 160,
      render: (_, record, index) => (
        <EditableText value={record.name} onChange={(value) => patchEvent(index, { name: value })} />
      ),
    },
    {
      title: '类型',
      width: 120,
      render: (_, record, index) => (
        <Select
          size="small"
          value={record.type}
          options={EVENT_TYPE_OPTIONS}
          onChange={(value) => patchEvent(index, { type: value })}
          style={{ width: '100%' }}
        />
      ),
    },
    {
      title: '优先级',
      width: 80,
      render: (_, record, index) => (
        <InputNumber
          size="small"
          min={0}
          max={255}
          value={record.priority}
          onChange={(value) => patchEvent(index, { priority: value ?? 0 })}
          style={{ width: '100%' }}
        />
      ),
    },
    {
      title: '周期',
      width: 80,
      render: (_, record, index) => (
        <InputNumber
          size="small"
          min={0}
          max={255}
          value={record.timeCycle}
          onChange={(value) => patchEvent(index, { timeCycle: value ?? 0 })}
          style={{ width: '100%' }}
        />
      ),
    },
    {
      title: '时间单位',
      width: 110,
      render: (_, record, index) => (
        <Select
          size="small"
          value={record.timeUnit}
          options={TIME_UNIT_OPTIONS}
          onChange={(value) => patchEvent(index, { timeUnit: value })}
          style={{ width: '100%' }}
        />
      ),
    },
    {
      title: '最大DAQ列表数',
      width: 120,
      render: (_, record, index) => (
        <InputNumber
          size="small"
          min={0}
          max={255}
          value={record.maxDaqList}
          onChange={(value) => patchEvent(index, { maxDaqList: value ?? 0 })}
          style={{ width: '100%' }}
        />
      ),
    },
    {
      title: '操作',
      width: 80,
      render: (_, __, index) => (
        <Button type="link" size="small" danger onClick={() => removeEvent(index)}>
          删除
        </Button>
      ),
    },
  ];

  const basicTab = (
    <div className="settings-section">
      <Row gutter={16}>
        <Col span={12}>
          <div className="field-label">项目名称</div>
          <Input
            value={draft.projectName}
            onChange={(event) => patchDraft((current) => ({ ...current, projectName: event.target.value }))}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">模块名</div>
          <Input
            value={draft.moduleName}
            onChange={(event) => patchDraft((current) => ({ ...current, moduleName: event.target.value }))}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">CRO ID</div>
          <HexInput value={draft.xcp.croCanId} onChange={(value) => patchXcp({ croCanId: value })} />
        </Col>
        <Col span={12}>
          <div className="field-label">DTO ID</div>
          <HexInput value={draft.xcp.dtoCanId} onChange={(value) => patchXcp({ dtoCanId: value })} />
        </Col>
        <Col span={12}>
          <div className="field-label">MAX_CTO</div>
          <InputNumber
            value={draft.xcp.maxCto}
            onChange={(value) => patchXcp({ maxCto: value ?? 0 })}
            style={{ width: '100%' }}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">MAX_DTO</div>
          <InputNumber
            value={draft.xcp.maxDto}
            onChange={(value) => patchXcp({ maxDto: value ?? 0 })}
            style={{ width: '100%' }}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">字节序</div>
          <Select
            value={draft.xcp.byteOrder}
            options={BYTE_ORDERS.map((value) => ({ label: value, value }))}
            onChange={(value) => patchXcp({ byteOrder: value })}
            style={{ width: '100%' }}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">地址粒度</div>
          <Select
            value={draft.xcp.addressGranularity}
            options={ADDRESS_GRANULARITY_OPTIONS.map((item) => ({ ...item }))}
            onChange={(value) => patchXcp({ addressGranularity: value })}
            style={{ width: '100%' }}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">ELF地址类型</div>
          <Select
            value={draft.xcp.addressMode}
            options={ADDRESS_MODE_OPTIONS}
            onChange={(value) => patchXcp({ addressMode: value })}
            style={{ width: '100%' }}
          />
        </Col>
      </Row>
    </div>
  );

  const elfTab = (
    <div className="settings-section">
      <div className="field-label">当前文件</div>
      <div style={{ display: 'flex', gap: 8 }}>
        <Input readOnly value={draft.outFilePath} placeholder="尚未选择 ELF/OUT 文件" />
        <Button onClick={() => void browseOutFile()}>浏览…</Button>
      </div>
      <p className="hint-text">选择文件后，需点击「确定」才会加载符号；「取消」不应用本次更改。</p>
    </div>
  );

  const daqTab = (
    <div className="settings-section">
      <Row gutter={16}>
        <Col span={12}>
          <div className="field-label">配置模式</div>
          <Select
            value={draft.daq.mode}
            options={DAQ_MODE_OPTIONS.map((item) => ({ ...item }))}
            onChange={(value) => patchDaq({ mode: value })}
            style={{ width: '100%' }}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">最大DAQ编号</div>
          <InputNumber
            value={draft.daq.maxDaq}
            onChange={(value) => patchDaq({ maxDaq: value ?? 0 })}
            style={{ width: '100%' }}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">最大事件通道号</div>
          <InputNumber
            value={draft.daq.maxEventChannel}
            onChange={(value) => patchDaq({ maxEventChannel: value ?? 0 })}
            style={{ width: '100%' }}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">最小DAQ编号</div>
          <InputNumber
            min={0}
            max={255}
            value={draft.daq.minDaq}
            onChange={(value) => patchDaq({ minDaq: value ?? 0 })}
            style={{ width: '100%' }}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">过载指示</div>
          <Select
            value={draft.daq.overloadIndication}
            options={OVERLOAD_OPTIONS.map((item) => ({ ...item }))}
            onChange={(value) => patchDaq({ overloadIndication: value })}
            style={{ width: '100%' }}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">PID标识类型</div>
          <Select
            value={draft.daq.identificationFieldType}
            options={ID_FIELD_OPTIONS.map((item) => ({ ...item }))}
            onChange={(value) => patchDaq({ identificationFieldType: value })}
            style={{ width: '100%' }}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">扩展地址</div>
          <Select
            value={draft.daq.addressExtension}
            options={ADDRESS_EXTENSION_OPTIONS.map((item) => ({ ...item }))}
            onChange={(value) => patchDaq({ addressExtension: value })}
            style={{ width: '100%' }}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">优化类型</div>
          <Select
            value={draft.daq.optimisationType}
            options={OPTIMISATION_OPTIONS.map((item) => ({ ...item }))}
            onChange={(value) => patchDaq({ optimisationType: value })}
            style={{ width: '100%' }}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">ODT条目粒度</div>
          <Select
            value={draft.daq.granularity}
            options={DAQ_GRANULARITY_OPTIONS.map((item) => ({ ...item }))}
            onChange={(value) => patchDaq({ granularity: value })}
            style={{ width: '100%' }}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">ODT条目最大长度</div>
          <InputNumber
            min={0}
            max={255}
            value={draft.daq.maxOdtEntrySizeDaq}
            onChange={(value) => patchDaq({ maxOdtEntrySizeDaq: value ?? 0 })}
            style={{ width: '100%' }}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">支持预分频</div>
          <Select
            value={draft.daq.prescalerSupported}
            options={YES_NO_OPTIONS}
            onChange={(value) => patchDaq({ prescalerSupported: value })}
            style={{ width: '100%' }}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">支持恢复</div>
          <Select
            value={draft.daq.resumeSupported}
            options={YES_NO_OPTIONS}
            onChange={(value) => patchDaq({ resumeSupported: value })}
            style={{ width: '100%' }}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">支持关闭PID</div>
          <Select
            value={draft.daq.pidOffSupported}
            options={YES_NO_OPTIONS}
            onChange={(value) => patchDaq({ pidOffSupported: value })}
            style={{ width: '100%' }}
          />
        </Col>
      </Row>
    </div>
  );

  const timestampTab = (
    <div className="settings-section">
      <Checkbox
        checked={draft.daq.timestampSupported}
        onChange={(event) => patchDaq({ timestampSupported: event.target.checked })}
      >
        启用时间戳支持
      </Checkbox>
      <Row gutter={16} style={{ marginTop: 16 }}>
        <Col span={12}>
          <div className="field-label">滴答数 (Ticks)</div>
          <InputNumber
            disabled={!draft.daq.timestampSupported}
            value={draft.daq.timestampTicks}
            onChange={(value) => patchDaq({ timestampTicks: value ?? 0 })}
            style={{ width: '100%' }}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">时间戳大小</div>
          <Select
            disabled={!draft.daq.timestampSupported}
            value={draft.daq.timestampSize}
            options={TIMESTAMP_SIZE_OPTIONS.map((value) => ({ label: value, value }))}
            onChange={(value) => patchDaq({ timestampSize: value })}
            style={{ width: '100%' }}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">时间单位</div>
          <Select
            disabled={!draft.daq.timestampSupported}
            value={draft.daq.timestampUnit}
            options={TIMESTAMP_UNIT_OPTIONS.map((value) => ({ label: value, value }))}
            onChange={(value) => patchDaq({ timestampUnit: value })}
            style={{ width: '100%' }}
          />
        </Col>
        <Col span={12}>
          <div className="field-label">从站时间戳固定模式</div>
          <Select
            disabled={!draft.daq.timestampSupported}
            value={draft.daq.timestampFixed}
            options={YES_NO_OPTIONS}
            onChange={(value) => patchDaq({ timestampFixed: value })}
            style={{ width: '100%' }}
          />
        </Col>
      </Row>
    </div>
  );

  const eventTab = (
    <div className="settings-section">
      <div className="tab-toolbar">
        <span className="tab-toolbar-title">EVENT 事件通道</span>
        <Button size="small" type="primary" icon={<PlusOutlined />} onClick={addEvent}>
          添加
        </Button>
      </div>
      <Table<EventChannel>
        size="small"
        bordered
        rowKey={(record) => `${record.channelNumber}-${record.name}`}
        columns={eventColumns}
        dataSource={draft.eventChannels}
        pagination={false}
        scroll={{ x: 830, y: 300 }}
      />
      <p className="hint-text">
        当前事件数量：{draft.eventChannels.length}，MAX_EVENT_CHANNEL：{draft.daq.maxEventChannel}
        {draft.eventChannels.length > draft.daq.maxEventChannel ? (
          <Tag color="warning" style={{ marginLeft: 8 }}>
            超出上限
          </Tag>
        ) : null}
      </p>
    </div>
  );

  return (
    <Modal
      open
      title="项目设置"
      width={920}
      okText="确定"
      cancelText="取消"
      onOk={() => void applySettings(draft)}
      onCancel={closeSettings}
      destroyOnHidden
      styles={{ body: { maxHeight: '68vh', overflowY: 'auto', paddingRight: 8 } }}
    >
      <Tabs
        items={[
          { key: 'basic', label: '基本设置', children: basicTab },
          { key: 'elf', label: '加载 ELF/DWARF', children: elfTab },
          { key: 'daq', label: 'DAQ 配置', children: daqTab },
          { key: 'timestamp', label: '时间戳配置', children: timestampTab },
          { key: 'event', label: 'EVENT 事件', children: eventTab },
        ]}
      />
    </Modal>
  );
}
