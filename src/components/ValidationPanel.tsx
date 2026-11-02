import { CheckCircleOutlined, CloseCircleOutlined, ExclamationCircleOutlined } from '@ant-design/icons';
import { useAppStore } from '../store';

export function ValidationPanel() {
  const validation = useAppStore((state) => state.validation);

  if (!validation) {
    return <div className="validation-empty">尚未执行校验</div>;
  }

  const messages = [
    ...validation.errors.map((text) => ({ kind: 'error' as const, text: `错误: ${text}` })),
    ...validation.warnings.map((text) => ({ kind: 'warning' as const, text: `警告: ${text}` })),
  ];

  if (messages.length === 0) {
    return (
      <div className="validation-empty validation-ok">
        <CheckCircleOutlined /> 校验通过，可以导出
      </div>
    );
  }

  return (
    <div className="validation-list">
      {messages.map((message, index) => (
        <div key={index} className={`validation-item ${message.kind}`}>
          {message.kind === 'error' ? <CloseCircleOutlined /> : <ExclamationCircleOutlined />}
          <span>{message.text}</span>
        </div>
      ))}
    </div>
  );
}
