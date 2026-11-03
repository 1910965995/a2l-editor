import {
  ExportOutlined,
  FileAddOutlined,
  FileTextOutlined,
  FolderOpenOutlined,
  SaveOutlined,
  SettingOutlined,
} from '@ant-design/icons';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { Button, Space } from 'antd';
import { useEffect } from 'react';
import { isTauri } from '../api';
import { APP_TITLE } from '../constants';
import { useAppStore } from '../store';

const basename = (path: string): string => path.replace(/\\/g, '/').split('/').pop() ?? path;

export function Toolbar() {
  const project = useAppStore((state) => state.project);
  const projectPath = useAppStore((state) => state.projectPath);
  const dirty = useAppStore((state) => state.project !== null && state.project !== state.savedProject);
  const newProject = useAppStore((state) => state.newProject);
  const openProject = useAppStore((state) => state.openProject);
  const saveProject = useAppStore((state) => state.saveProject);
  const openSettings = useAppStore((state) => state.openSettings);
  const exportA2l = useAppStore((state) => state.exportA2l);

  const fileName = project ? (projectPath ? basename(projectPath) : '未命名工程') : null;

  useEffect(() => {
    const title = fileName ? `${dirty ? '*' : ''}${fileName} - ${APP_TITLE}` : APP_TITLE;
    document.title = title;
    if (isTauri) {
      void getCurrentWindow().setTitle(title).catch(() => undefined);
    }
  }, [fileName, dirty]);

  return (
    <div className="toolbar">
      <div className="project-summary">
        <div className="project-icon">
          <FileTextOutlined />
        </div>
        <div className="project-meta">
          <div className="project-title-row">
            <span className="project-file" title={projectPath ?? '尚未保存到文件'}>
              {fileName ?? '正在加载…'}
            </span>
            {dirty && <span className="dirty-dot" title="有未保存的修改" />}
            {project && <span className="platform-tag">{project.targetPlatform}</span>}
          </div>
          <div className="project-subtitle">
            {project
              ? `${project.projectName}${project.moduleName ? ` · ${project.moduleName}` : ''}`
              : '正在加载项目…'}
          </div>
        </div>
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
