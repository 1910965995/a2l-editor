import { App as AntApp, ConfigProvider, Spin, Splitter, Tabs } from 'antd';
import zhCN from 'antd/locale/zh_CN';
import { useEffect } from 'react';
import './App.css';
import { CharacteristicTable } from './components/CharacteristicTable';
import { DaqListTable } from './components/DaqListTable';
import { MeasurementTable } from './components/MeasurementTable';
import { MemorySegmentTable } from './components/MemorySegmentTable';
import { PreviewTab } from './components/PreviewTab';
import { SymbolTree } from './components/SymbolTree';
import { Toolbar } from './components/Toolbar';
import { ValidationPanel } from './components/ValidationPanel';
import { CharacteristicDialog } from './components/dialogs/CharacteristicDialog';
import { MeasurementDialog } from './components/dialogs/MeasurementDialog';
import { MemorySegmentDialog } from './components/dialogs/MemorySegmentDialog';
import { OdtDialog } from './components/dialogs/OdtDialog';
import { ProjectSettingsDialog } from './components/dialogs/ProjectSettingsDialog';
import { useAppStore } from './store';

function TabLabel({ title, tag, count }: { title: string; tag: string; count: number }) {
  return (
    <span className="tab-label">
      <span>{title}</span>
      <span className="tab-label-tag">{tag}</span>
      <span className="tab-label-count">{count}</span>
    </span>
  );
}

export default function App() {
  const init = useAppStore((state) => state.init);
  const project = useAppStore((state) => state.project);
  const status = useAppStore((state) => state.status);
  const loading = useAppStore((state) => state.loading);

  useEffect(() => {
    void init();
  }, [init]);

  const tabs = project
    ? [
        {
          key: 'measurements',
          label: <TabLabel title="测量量" tag="MEASUREMENT" count={project.measurements.length} />,
          children: <MeasurementTable />,
        },
        {
          key: 'characteristics',
          label: <TabLabel title="标定量" tag="CHARACTERISTIC" count={project.characteristics.length} />,
          children: <CharacteristicTable />,
        },
        {
          key: 'memory',
          label: <TabLabel title="内存段" tag="MEMORY_SEGMENT" count={project.memorySegments.length} />,
          children: <MemorySegmentTable />,
        },
        {
          key: 'daq',
          label: <TabLabel title="DAQ 列表" tag="DAQ_LIST" count={project.daqLists.length} />,
          children: <DaqListTable />,
        },
        {
          key: 'preview',
          label: 'A2L 预览',
          children: <PreviewTab />,
        },
      ]
    : [];

  return (
    <ConfigProvider
      locale={zhCN}
      theme={{
        token: {
          colorPrimary: '#2563eb',
          borderRadius: 6,
          fontFamily: '"Segoe UI", "Microsoft YaHei UI", "Microsoft YaHei", sans-serif',
        },
      }}
    >
      <AntApp>
        <div className="app-shell">
          <Toolbar />

          {loading && (
            <div className="app-loading">
              <Spin size="large" />
            </div>
          )}

          {!loading && !project && (
            <div className="app-loading">加载项目失败，请查看底部状态栏。</div>
          )}

          {!loading && project && (
            <>
              <div className="main-content">
                <Splitter>
                  <Splitter.Panel defaultSize={250} min={250} max={520}>
                    <div className="panel">
                      <div className="panel-header">符号浏览</div>
                      <SymbolTree />
                    </div>
                  </Splitter.Panel>
                  <Splitter.Panel>
                    <div className="panel right-panel">
                      <div className="panel-header">A2L 对象</div>
                      <Splitter layout="vertical" className="right-splitter">
                        <Splitter.Panel defaultSize="79%" min={180}>
                          <Tabs className="object-tabs" items={tabs} destroyOnHidden />
                        </Splitter.Panel>
                        <Splitter.Panel defaultSize="21%" min={80} max="60%">
                          <div className="validation-section">
                            <div className="validation-header">校验信息</div>
                            <div className="validation-panel">
                              <ValidationPanel />
                            </div>
                          </div>
                        </Splitter.Panel>
                      </Splitter>
                    </div>
                  </Splitter.Panel>
                </Splitter>
              </div>

              <div className="status-bar">
                <span className="status-text">{status}</span>
                <span className="status-brand">A2L Tool</span>
              </div>
            </>
          )}

          <MeasurementDialog />
          <CharacteristicDialog />
          <MemorySegmentDialog />
          <ProjectSettingsDialog />
          <OdtDialog />
        </div>
      </AntApp>
    </ConfigProvider>
  );
}
