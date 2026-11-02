import { AutoComplete, Col, Form, Input, InputNumber, Modal, Row, Select } from 'antd';
import { useAppStore } from '../../store';
import { A2L_DATA_TYPES, type Measurement } from '../../types';
import { HexInput } from '../Editable';

const DATA_TYPE_OPTIONS = A2L_DATA_TYPES.map((value) => ({ label: value, value }));

export function MeasurementDialog() {
  const editor = useAppStore((state) => state.measurementEditor);
  const project = useAppStore((state) => state.project);
  const commitMeasurement = useAppStore((state) => state.commitMeasurement);
  const cancelMeasurement = useAppStore((state) => state.cancelMeasurement);
  const [form] = Form.useForm();

  const handleOk = async () => {
    if (!editor) return;
    const values = (await form.validateFields()) as Measurement;
    await commitMeasurement({ ...editor.value, ...values });
  };

  return (
    <Modal
      open={editor !== null}
      title={editor?.mode === 'add' ? '添加测量量' : '编辑测量量'}
      width={760}
      okText="确定"
      cancelText="取消"
      onOk={() => void handleOk()}
      onCancel={cancelMeasurement}
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
            <Form.Item label="数据类型" name="dataType">
              <Select options={DATA_TYPE_OPTIONS} />
            </Form.Item>
          </Col>
          <Col span={24}>
            <Form.Item label="描述" name="longIdentifier">
              <Input />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item label="地址" name="ecuAddress">
              <HexInput placeholder="0x..." />
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
          <Col span={12}>
            <Form.Item label="分辨率" name="resolution">
              <InputNumber style={{ width: '100%' }} />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item label="精度" name="accuracy">
              <InputNumber style={{ width: '100%' }} />
            </Form.Item>
          </Col>
        </Row>
      </Form>
    </Modal>
  );
}
