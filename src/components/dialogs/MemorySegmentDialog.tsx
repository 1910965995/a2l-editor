import { Col, Form, Input, InputNumber, Modal, Row, Select } from 'antd';
import { useLayoutEffect } from 'react';
import { useAppStore } from '../../store';
import {
  MEM_ATTRIBUTES,
  MEM_MEMORY_TYPES,
  PRG_TYPES,
  type MemorySegment,
} from '../../types';
import { HexInput } from '../Editable';

const PRG_TYPE_OPTIONS = PRG_TYPES.map((value) => ({ label: value, value }));
const MEMORY_TYPE_OPTIONS = MEM_MEMORY_TYPES.map((value) => ({ label: value, value }));
const ATTRIBUTE_OPTIONS = MEM_ATTRIBUTES.map((value) => ({ label: value, value }));

export function MemorySegmentDialog() {
  const editor = useAppStore((state) => state.memorySegmentEditor);
  const commitMemorySegment = useAppStore((state) => state.commitMemorySegment);
  const cancelMemorySegment = useAppStore((state) => state.cancelMemorySegment);
  const [form] = Form.useForm();

  // 表单实例跨会话复用，rc-form 重挂载时旧 store 值会覆盖新的 initialValues，
  // 必须在每次打开时强制写入当前会话的值，否则显示上一次的内容。
  useLayoutEffect(() => {
    if (editor) {
      form.setFieldsValue(editor.value);
    }
  }, [editor, form]);

  const handleOk = async () => {
    if (!editor) return;
    const values = (await form.validateFields()) as MemorySegment;
    await commitMemorySegment({ ...editor.value, ...values });
  };

  return (
    <Modal
      open={editor !== null}
      title={editor?.mode === 'add' ? '添加内存段' : '编辑内存段'}
      width={760}
      okText="确定"
      cancelText="取消"
      onOk={() => void handleOk()}
      onCancel={cancelMemorySegment}
      destroyOnHidden
    >
      <Form
        key={editor?.session ?? 'empty'}
        form={form}
        layout="vertical"
        preserve={false}
        initialValues={editor?.value}
      >
        <Row gutter={16}>
          <Col span={12}>
            <Form.Item label="名称" name="name">
              <Input />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item label="描述" name="longIdentifier">
              <Input />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item label="程序类型" name="prgType">
              <Select options={PRG_TYPE_OPTIONS} />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item label="内存类型" name="memoryType">
              <Select options={MEMORY_TYPE_OPTIONS} />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item label="属性" name="attribute">
              <Select options={ATTRIBUTE_OPTIONS} />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item label="起始地址" name="address">
              <HexInput placeholder="0x..." />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item label="大小" name="size">
              <HexInput placeholder="0x..." />
            </Form.Item>
          </Col>
          {([0, 1, 2, 3, 4] as const).map((index) => (
            <Col span={4} key={index}>
              <Form.Item label={`偏移${index}`} name={`offset${index}`}>
                <InputNumber style={{ width: '100%' }} precision={0} />
              </Form.Item>
            </Col>
          ))}
        </Row>
      </Form>
    </Modal>
  );
}
