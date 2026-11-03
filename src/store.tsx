import { create } from 'zustand';
import { message, Modal } from 'antd';
import { api, openFileDialog, saveFileDialog } from './api';
import {
  A2L_FILE_FILTERS,
  OUT_FILE_FILTERS,
  PROJECT_FILE_FILTERS,
  defaultMemorySegment,
} from './constants';
import type {
  A2lProject,
  AddressChange,
  Characteristic,
  DaqListConfig,
  Measurement,
  MemorySegment,
  SymbolNode,
  ValidationResult,
} from './types';

export interface EditorState<T> {
  mode: 'add' | 'edit';
  index: number | null;
  value: T;
  session: number;
}

let editorSession = 0;
const nextEditorSession = () => ++editorSession;

export interface OdtEditorState {
  index: number;
  value: DaqListConfig;
}

interface AppState {
  project: A2lProject | null;
  savedProject: A2lProject | null;
  projectPath: string | null;
  symbols: SymbolNode[];
  searchText: string;
  selectedSymbol: SymbolNode | null;
  status: string;
  validation: ValidationResult | null;
  preview: string;
  loading: boolean;

  measurementEditor: EditorState<Measurement> | null;
  characteristicEditor: EditorState<Characteristic> | null;
  memorySegmentEditor: EditorState<MemorySegment> | null;
  settingsOpen: boolean;
  odtEditor: OdtEditorState | null;

  init: () => Promise<void>;
  setSearchText: (value: string) => void;
  setSelectedSymbol: (node: SymbolNode | null) => void;
  setStatus: (value: string) => void;
  refreshPreview: () => Promise<void>;

  newProject: () => Promise<void>;
  openProject: () => Promise<void>;
  saveProject: () => Promise<void>;
  exportA2l: () => Promise<void>;

  beginAddMeasurement: () => Promise<void>;
  beginEditMeasurement: (index: number) => void;
  commitMeasurement: (value: Measurement) => Promise<void>;
  cancelMeasurement: () => void;
  patchMeasurement: (index: number, patch: Partial<Measurement>) => void;
  removeMeasurement: (index: number) => void;

  beginAddCharacteristic: () => Promise<void>;
  beginEditCharacteristic: (index: number) => void;
  commitCharacteristic: (value: Characteristic) => Promise<void>;
  cancelCharacteristic: () => void;
  patchCharacteristic: (index: number, patch: Partial<Characteristic>) => void;
  removeCharacteristic: (index: number) => void;

  beginAddMemorySegment: () => void;
  beginEditMemorySegment: (index: number) => void;
  commitMemorySegment: (value: MemorySegment) => Promise<void>;
  cancelMemorySegment: () => void;
  patchMemorySegment: (index: number, patch: Partial<MemorySegment>) => void;
  removeMemorySegment: (index: number) => void;

  addDaqList: () => void;
  removeDaqList: (index: number) => void;
  patchDaqList: (index: number, patch: Partial<DaqListConfig>) => void;
  openOdt: (index: number) => void;
  commitOdt: (value: DaqListConfig) => Promise<void>;
  cancelOdt: () => void;

  openSettings: () => void;
  closeSettings: () => void;
  applySettings: (draft: A2lProject) => Promise<void>;
}

const clone = <T,>(value: T): T => structuredClone(value);

const basename = (path: string): string => path.replace(/\\/g, '/').split('/').pop() ?? path;

const normalizePath = (path: string): string => path.trim().toLowerCase();

function confirmDialog(content: string): Promise<boolean> {
  return new Promise((resolve) => {
    Modal.confirm({
      title: '提示',
      content: (
        <div style={{ whiteSpace: 'pre-wrap' }}>
          {content}
        </div>
      ),
      okText: '是',
      cancelText: '否',
      onOk: () => resolve(true),
      onCancel: () => resolve(false),
    });
  });
}

export const useAppStore = create<AppState>((set, get) => {
  const refreshPreview = async () => {
    const project = get().project;
    if (!project) return;
    try {
      const preview = await api.previewA2l(project);
      if (get().project === project) {
        set({ preview });
      }
    } catch {
      // 预览失败不影响编辑流程
    }
  };

  const checkAddressChanges = async (project: A2lProject, symbols: SymbolNode[]) => {
    if (project.measurements.length === 0 && project.characteristics.length === 0) return;

    let changes: AddressChange[] = [];
    try {
      changes = await api.detectAddressChanges(project, symbols);
    } catch {
      return;
    }
    if (changes.length === 0) return;

    const count = changes.length;
    Modal.confirm({
      title: '地址变更',
      content: `检测到 ${count} 个变量地址发生变化，是否更新？`,
      okText: '更新',
      cancelText: '跳过',
      onOk: async () => {
        const current = get().project;
        if (!current) return;
        const updated = await api.applyAddressChanges(current, changes);
        set({ project: updated });
        await get().refreshPreview();
        set({ status: `已更新 ${count} 个变量地址` });
      },
      onCancel: () => set({ status: `检测到 ${count} 个变量地址变化，已跳过更新` }),
    });
  };

  const loadSymbolsInto = async (path: string, updateOutFilePath: boolean): Promise<SymbolNode[]> => {
    const symbols = await api.loadSymbols(path);
    const current = get().project;
    if (!current) return symbols;
    const updated = updateOutFilePath && normalizePath(current.outFilePath) !== normalizePath(path)
      ? { ...current, outFilePath: path }
      : current;
    set({ project: updated, symbols, selectedSymbol: null });
    await get().refreshPreview();
    await checkAddressChanges(updated, symbols);
    return symbols;
  };

  const replaceProject = async (project: A2lProject) => {
    set({ project });
    await get().refreshPreview();
  };

  return {
    project: null,
    savedProject: null,
    projectPath: null,
    symbols: [],
    searchText: '',
    selectedSymbol: null,
    status: '就绪',
    validation: null,
    preview: '',
    loading: true,
    measurementEditor: null,
    characteristicEditor: null,
    memorySegmentEditor: null,
    settingsOpen: false,
    odtEditor: null,

    init: async () => {
      set({ loading: true });
      try {
        const project = await api.newProject();
        set({ project, savedProject: project, status: '就绪', loading: false });
        await get().refreshPreview();
      } catch (error) {
        set({ status: `初始化失败：${error}`, loading: false });
      }
    },

    setSearchText: (value) => set({ searchText: value }),
    setSelectedSymbol: (node) => set({ selectedSymbol: node }),
    setStatus: (value) => set({ status: value }),
    refreshPreview,

    newProject: async () => {
      const project = await api.newProject();
      set({
        project,
        savedProject: project,
        projectPath: null,
        symbols: [],
        selectedSymbol: null,
        validation: null,
        measurementEditor: null,
        characteristicEditor: null,
        memorySegmentEditor: null,
        odtEditor: null,
      });
      await get().refreshPreview();
      set({ status: '已新建项目' });
    },

    openProject: async () => {
      const path = await openFileDialog({ filters: PROJECT_FILE_FILTERS });
      if (!path) return;

      try {
        const project = await api.loadProject(path);
        set({
          project,
          savedProject: project,
          projectPath: path,
          symbols: [],
          selectedSymbol: null,
          validation: null,
          measurementEditor: null,
          characteristicEditor: null,
          memorySegmentEditor: null,
          odtEditor: null,
        });
        await get().refreshPreview();

        if (!project.outFilePath.trim()) {
          set({ status: `项目已打开：${basename(path)}` });
          return;
        }

        if (await api.fileExists(project.outFilePath)) {
          try {
            const symbols = await loadSymbolsInto(project.outFilePath, true);
            set({ status: `项目已打开：${basename(path)}；已自动导入 ${symbols.length} 个符号` });
          } catch (error) {
            message.error(`自动导入 ELF/OUT 失败：${error}`);
            set({ status: `项目已打开：${basename(path)}；自动导入 ELF 失败` });
          }
          return;
        }

        const repick = await confirmDialog(
          `找不到已保存的 ELF/OUT 文件：\n${project.outFilePath}\n\n是否重新选择？`,
        );
        if (repick) {
          const newPath = await openFileDialog({ filters: OUT_FILE_FILTERS });
          if (newPath) {
            try {
              const symbols = await loadSymbolsInto(newPath, true);
              set({ status: `项目已打开：${basename(path)}；已导入 ${symbols.length} 个符号` });
              return;
            } catch (error) {
              message.error(`导入 ELF/OUT 失败：${error}`);
            }
          }
        }

        set({ status: `项目已打开：${basename(path)}；已跳过自动导入 ELF` });
      } catch (error) {
        message.error(String(error));
      }
    },

    saveProject: async () => {
      const project = get().project;
      if (!project) return;
      const path = await saveFileDialog({
        defaultPath: `${project.projectName || 'Project'}.a2lproj`,
        filters: PROJECT_FILE_FILTERS,
      });
      if (!path) return;

      try {
        const saved = await api.saveProject(path, project);
        set({ project: saved, savedProject: saved, projectPath: path });
        await get().refreshPreview();
        set({ status: `项目已保存：${basename(path)}` });
      } catch (error) {
        message.error(`保存工程失败：${error}`);
      }
    },

    exportA2l: async () => {
      const project = get().project;
      if (!project) return;

      let result: ValidationResult;
      try {
        result = await api.validateProject(project);
      } catch (error) {
        message.error(`校验失败：${error}`);
        return;
      }
      set({ validation: result });
      if (!result.canExport) {
        set({ status: '校验失败，不能导出' });
        message.error('校验失败，请先修复错误。');
        return;
      }

      const path = await saveFileDialog({
        defaultPath: `${project.projectName || 'Project'}.a2l`,
        filters: A2L_FILE_FILTERS,
      });
      if (!path) return;

      try {
        await api.exportA2l(path, project);
        const updated = { ...project, lastExportPath: path };
        set({ project: updated, status: `A2L 已导出：${basename(path)}` });
        await get().refreshPreview();
      } catch (error) {
        message.error(`导出 A2L 失败：${error}`);
      }
    },

    beginAddMeasurement: async () => {
      const { project, selectedSymbol } = get();
      if (!project) return;
      if (!selectedSymbol) {
        message.info('请先选择一个符号。');
        return;
      }
      if (!selectedSymbol.isLeaf) {
        message.info('请选择具体变量、数组元素或结构体成员，不能直接添加父节点。');
        return;
      }
      if (project.measurements.some((item) => item.name === selectedSymbol.path)) {
        set({ status: `测量量已存在：${selectedSymbol.path}` });
        return;
      }
      const value = await api.createMeasurement(selectedSymbol, project.xcp.addressMode);
      set({ measurementEditor: { mode: 'add', index: null, value, session: nextEditorSession() } });
    },

    beginEditMeasurement: (index) => {
      const project = get().project;
      const value = project?.measurements[index];
      if (!value) return;
      set({ measurementEditor: { mode: 'edit', index, value: clone(value), session: nextEditorSession() } });
    },

    commitMeasurement: async (value) => {
      const editor = get().measurementEditor;
      const project = get().project;
      if (!editor || !project) return;

      const measurements =
        editor.mode === 'add'
          ? [...project.measurements, value]
          : project.measurements.map((item, index) => (index === editor.index ? value : item));

      set({ project: { ...project, measurements }, measurementEditor: null });
      await get().refreshPreview();
      set({
        status: `${editor.mode === 'add' ? '已添加' : '已更新'}测量量：${value.name}`,
      });
    },

    cancelMeasurement: () => set({ measurementEditor: null }),

    patchMeasurement: (index, patch) => {
      const project = get().project;
      if (!project) return;
      const measurements = project.measurements.map((item, itemIndex) =>
        itemIndex === index ? { ...item, ...patch } : item,
      );
      void replaceProject({ ...project, measurements });
    },

    removeMeasurement: (index) => {
      const project = get().project;
      const measurement = project?.measurements[index];
      if (!project || !measurement) return;
      const measurements = project.measurements.filter((_, itemIndex) => itemIndex !== index);
      void replaceProject({ ...project, measurements });
      set({ status: `已删除测量量：${measurement.name}` });
    },

    beginAddCharacteristic: async () => {
      const { project, selectedSymbol } = get();
      if (!project) return;
      if (!selectedSymbol) {
        message.info('请先选择一个符号。');
        return;
      }
      if (!selectedSymbol.isLeaf) {
        message.info('请选择具体变量、数组元素或结构体成员，不能直接添加父节点。');
        return;
      }
      if (project.characteristics.some((item) => item.name === selectedSymbol.path)) {
        set({ status: `标定量已存在：${selectedSymbol.path}` });
        return;
      }
      const value = await api.createCharacteristic(selectedSymbol, project.xcp.addressMode);
      set({ characteristicEditor: { mode: 'add', index: null, value, session: nextEditorSession() } });
    },

    beginEditCharacteristic: (index) => {
      const project = get().project;
      const value = project?.characteristics[index];
      if (!value) return;
      set({ characteristicEditor: { mode: 'edit', index, value: clone(value), session: nextEditorSession() } });
    },

    commitCharacteristic: async (value) => {
      const editor = get().characteristicEditor;
      const project = get().project;
      if (!editor || !project) return;

      const characteristics =
        editor.mode === 'add'
          ? [...project.characteristics, value]
          : project.characteristics.map((item, index) => (index === editor.index ? value : item));

      set({ project: { ...project, characteristics }, characteristicEditor: null });
      await get().refreshPreview();
      set({
        status: `${editor.mode === 'add' ? '已添加' : '已更新'}标定量：${value.name}`,
      });
    },

    cancelCharacteristic: () => set({ characteristicEditor: null }),

    patchCharacteristic: (index, patch) => {
      const project = get().project;
      if (!project) return;
      const characteristics = project.characteristics.map((item, itemIndex) =>
        itemIndex === index ? { ...item, ...patch } : item,
      );
      void replaceProject({ ...project, characteristics });
    },

    removeCharacteristic: (index) => {
      const project = get().project;
      const characteristic = project?.characteristics[index];
      if (!project || !characteristic) return;
      const characteristics = project.characteristics.filter((_, itemIndex) => itemIndex !== index);
      void replaceProject({ ...project, characteristics });
      set({ status: `已删除标定量：${characteristic.name}` });
    },

    beginAddMemorySegment: () => {
      set({
        memorySegmentEditor: {
          mode: 'add',
          index: null,
          value: defaultMemorySegment(),
          session: nextEditorSession(),
        },
      });
    },

    beginEditMemorySegment: (index) => {
      const project = get().project;
      const value = project?.memorySegments[index];
      if (!value) return;
      set({ memorySegmentEditor: { mode: 'edit', index, value: clone(value), session: nextEditorSession() } });
    },

    commitMemorySegment: async (value) => {
      const editor = get().memorySegmentEditor;
      const project = get().project;
      if (!editor || !project) return;

      const memorySegments =
        editor.mode === 'add'
          ? [...project.memorySegments, value]
          : project.memorySegments.map((item, index) => (index === editor.index ? value : item));

      set({ project: { ...project, memorySegments }, memorySegmentEditor: null });
      await get().refreshPreview();
      const label = value.name.trim() || '(未命名)';
      set({ status: `${editor.mode === 'add' ? '已添加' : '已更新'}内存段：${label}` });
    },

    cancelMemorySegment: () => set({ memorySegmentEditor: null }),

    patchMemorySegment: (index, patch) => {
      const project = get().project;
      if (!project) return;
      const memorySegments = project.memorySegments.map((item, itemIndex) =>
        itemIndex === index ? { ...item, ...patch } : item,
      );
      void replaceProject({ ...project, memorySegments });
    },

    removeMemorySegment: (index) => {
      const project = get().project;
      const segment = project?.memorySegments[index];
      if (!project || !segment) return;
      const memorySegments = project.memorySegments.filter((_, itemIndex) => itemIndex !== index);
      void replaceProject({ ...project, memorySegments });
      set({ status: `已删除内存段：${segment.name.trim() || '(未命名)'}` });
    },

    addDaqList: () => {
      const project = get().project;
      if (!project) return;
      if (project.daq.mode === 'DYNAMIC') {
        message.info('DAQ配置为动态模式，如需添加DAQ列表请先切换至静态模式。');
        return;
      }
      if (project.eventChannels.length === 0) {
        message.info('未添加EVENT事件，请先在项目设置中创建EVENT事件。');
        return;
      }
      const index = project.daqLists.length;
      const daqList: DaqListConfig = {
        number: index,
        type: 'DAQ',
        maxOdt: 1,
        maxOdtEntries: 1,
        firstPid: (0x10 * index) & 0xff,
        eventFixed: 0,
        odts: [],
      };
      void replaceProject({ ...project, daqLists: [...project.daqLists, daqList] });
      set({ status: `已添加 DAQ 列表：${daqList.number}` });
    },

    removeDaqList: (index) => {
      const project = get().project;
      const daqList = project?.daqLists[index];
      if (!project || !daqList) return;
      const daqLists = project.daqLists.filter((_, itemIndex) => itemIndex !== index);
      void replaceProject({ ...project, daqLists });
      set({ status: `已删除 DAQ 列表：${daqList.number}` });
    },

    patchDaqList: (index, patch) => {
      const project = get().project;
      if (!project) return;
      const daqLists = project.daqLists.map((item, itemIndex) =>
        itemIndex === index ? { ...item, ...patch } : item,
      );
      void replaceProject({ ...project, daqLists });
    },

    openOdt: (index) => {
      const project = get().project;
      const daqList = project?.daqLists[index];
      if (!daqList) return;
      set({ odtEditor: { index, value: clone(daqList) } });
    },

    commitOdt: async (value) => {
      const editor = get().odtEditor;
      const project = get().project;
      if (!editor || !project) return;
      const daqLists = project.daqLists.map((item, index) =>
        index === editor.index ? value : item,
      );
      set({ project: { ...project, daqLists }, odtEditor: null });
      await get().refreshPreview();
      set({ status: `已更新 DAQ ${value.number} 的 ODT 配置` });
    },

    cancelOdt: () => set({ odtEditor: null }),

    openSettings: () => set({ settingsOpen: true }),
    closeSettings: () => set({ settingsOpen: false }),

    applySettings: async (draft) => {
      const previous = get().project;
      if (!previous) return;

      const elfChanged = normalizePath(draft.outFilePath) !== normalizePath(previous.outFilePath);
      set({ project: draft, settingsOpen: false });
      await get().refreshPreview();

      if (elfChanged && draft.outFilePath.trim()) {
        try {
          const symbols = await api.loadSymbols(draft.outFilePath);
          set({ symbols, selectedSymbol: null });
          await checkAddressChanges(draft, symbols);
          set({ status: `已导入 ${symbols.length} 个符号` });
        } catch (error) {
          message.error(`导入 ELF/OUT 失败：${error}`);
          set({ status: '项目设置已更新' });
        }
        return;
      }

      await checkAddressChanges(draft, get().symbols);
      set({ status: '项目设置已更新' });
    },
  };
});
