import React from 'react'
import { Button, Form, Input, message } from 'antd'
import { invoke } from '@tauri-apps/api'

export const getU8Arr = (data?: string) =>
  Array.from(new TextEncoder().encode(data))

type FieldType = {
  uuid?: string
  signature?: string
}

const onFinish = (values: FieldType) => {
  invoke('use_verify_signature', {
    data: values.uuid,
    signature: getU8Arr(values.signature),
  }).then((res: any) => {
    if (res) {
      message.success('验证成功')
    } else {
      message.error('验证失败')
    }
  })
}

const Verify: React.FC = () => (
  <Form name="basic" onFinish={onFinish} autoComplete="off">
    <Form.Item<FieldType>
      label="uuid"
      name="uuid"
      rules={[{ required: true, message: 'Please input your username!' }]}
    >
      <Input />
    </Form.Item>
    <Form.Item<FieldType>
      label="signature"
      name="signature"
      rules={[{ required: true, message: 'Please input your username!' }]}
    >
      <Input />
    </Form.Item>

    <Form.Item>
      <Button type="primary" htmlType="submit">
        Submit
      </Button>
    </Form.Item>
  </Form>
)

export default Verify
