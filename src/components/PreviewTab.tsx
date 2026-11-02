import { useAppStore } from '../store';

export function PreviewTab() {
  const preview = useAppStore((state) => state.preview);

  return (
    <div className="preview-panel">
      <pre className="a2l-preview">{preview || '// 暂无预览内容'}</pre>
    </div>
  );
}
