import {
  ExportOutlined,
  FileAddOutlined,
  FolderOpenOutlined,
  SaveOutlined,
  SettingOutlined,
} from '@ant-design/icons';
import { Button, Space } from 'antd';
import { APP_SUBTITLE, APP_TITLE } from '../constants';
import { useAppStore } from '../store';

export function Toolbar() {
  const newProject = useAppStore((state) => state.newProject);
  const openProject = useAppStore((state) => state.openProject);
  const saveProject = useAppStore((state) => state.saveProject);
  const openSettings = useAppStore((state) => state.openSettings);
  const exportA2l = useAppStore((state) => state.exportA2l);

  return (
    <div className="toolbar">
      <div className="brand">
        <div className="brand-title">
          {APP_TITLE} <span className="brand-version">v1.0</span>
        </div>
        <div className="brand-subtitle">{APP_SUBTITLE}</div>
      </div>

      <Space size={8} wrap>
        <Button icon={<FileAddOutlined />} onClick={() => void newProject()}>
          新建项目
        </Button>
        <Button icon={<FolderOpenOutlined />} onClick={() => void openProject()}>
          打开项目
        </Button>
        <Button icon={<SaveOutlined />} onClick={() => void saveProject()}>
          保存项目
        </Button>
        <Button icon={<SettingOutlined />} onClick={openSettings}>
          项目设置
        </Button>
        <Button type="primary" icon={<ExportOutlined />} onClick={() => void exportA2l()}>
          导出 A2L
        </Button>
      </Space>
    </div>
  );
}
