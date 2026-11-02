import { AutoComplete, Col, Form, Input, InputNumber, Modal, Row, Select } from 'antd';
import { useAppStore } from '../../store';
import { A2L_DATA_TYPES, CHARACTERISTIC_TYPES, type Characteristic } from '../../types';
import { HexInput } from '../Editable';

const CHARACTERISTIC_TYPE_OPTIONS = CHARACTERISTIC_TYPES.map((value) => ({ label: value, value }));
const DEPOSIT_OPTIONS = A2L_DATA_TYPES.map((value) => ({ label: `__${value}`, value: `__${value}` }));

export function CharacteristicDialog() {
  const editor = useAppStore((state) => state.characteristicEditor);
  const project = useAppStore((state) => state.project);
  const commitCharacteristic = useAppStore((state) => state.commitCharacteristic);
  const cancelCharacteristic = useAppStore((state) => state.cancelCharacteristic);
  const [form] = Form.useForm();

  const initialValues = editor
    ? {
        ...editor.value,
        extendedLowerLimit:
          editor.value.extendedLowerLimit <= -Number.MAX_VALUE ? undefined : editor.value.extendedLowerLimit,
        extendedUpperLimit:
          editor.value.extendedUpperLimit >= Number.MAX_VALUE ? undefined : editor.value.extendedUpperLimit,
      }
    : undefined;

  const handleOk = async () => {
    if (!editor) return;
    const values = (await form.validateFields()) as Characteristic & {
      extendedLowerLimit?: number | null;
      extendedUpperLimit?: number | null;
    };
    await commitCharacteristic({
      ...editor.value,
      ...values,
      extendedLowerLimit:
        values.extendedLowerLimit === undefined || values.extendedLowerLimit === null
          ? -Number.MAX_VALUE
          : values.extendedLowerLimit,
      extendedUpperLimit:
        values.extendedUpperLimit === undefined || values.extendedUpperLimit === null
          ? Number.MAX_VALUE
          : values.extendedUpperLimit,
    });
  };

  return (
    <Modal
      open={editor !== null}
      title={editor?.mode === 'add' ? '添加标定量' : '编辑标定量'}
      width={820}
      okText="确定"
      cancelText="取消"
      onOk={() => void handleOk()}
      onCancel={cancelCharacteristic}
      destroyOnHidden
    >
      <Form
        key={editor?.session ?? 'empty'}
        form={form}
        layout="vertical"
        preserve={false}
        initialValues={initialValues}
      >
        <Row gutter={16}>
          <Col span={12}>
            <Form.Item label="名称" name="name">
              <Input />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item label="特征类型" name="characteristicType">
              <Select options={CHARACTERISTIC_TYPE_OPTIONS} />
            </Form.Item>
          </Col>
          <Col span={24}>
            <Form.Item label="描述" name="longIdentifier">
              <Input />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item label="存储布局 (Deposit)" name="deposit">
              <Select options={DEPOSIT_OPTIONS} showSearch optionFilterProp="label" placeholder="__UWORD" />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item label="地址" name="ecuAddress">
              <HexInput placeholder="0x..." />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item label="最大步进" name="maxDiff">
              <InputNumber style={{ width: '100%' }} />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item label="转换方式" name="conversion">
              <AutoComplete
                options={(project?.compuMethods ?? []).map((item) => ({ value: item.name }))}
                placeholder="IDENTICAL"
              />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item label="单位" name="physUnit">
              <Input />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item label="显示格式" name="format">
              <Input placeholder="%6.2" />
            </Form.Item>
          </Col>
          <Col span={6}>
            <Form.Item label="最小值" name="lowerLimit">
              <InputNumber style={{ width: '100%' }} />
            </Form.Item>
          </Col>
          <Col span={6}>
            <Form.Item label="最大值" name="upperLimit">
              <InputNumber style={{ width: '100%' }} />
            </Form.Item>
          </Col>
          <Col span={6}>
            <Form.Item label="扩展最小值" name="extendedLowerLimit">
              <InputNumber style={{ width: '100%' }} placeholder="默认" />
            </Form.Item>
          </Col>
          <Col span={6}>
            <Form.Item label="扩展最大值" name="extendedUpperLimit">
              <InputNumber style={{ width: '100%' }} placeholder="默认" />
            </Form.Item>
          </Col>
        </Row>
      </Form>
    </Modal>
  );
}
