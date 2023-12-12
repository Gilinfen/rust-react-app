import { Select } from 'antd'
import { useState } from 'react'
import { OSList, getfile } from './chromium'

type ValUnion = (typeof OSList)[number]['val']

export default function Chrome() {
  const [os, setOs] = useState<ValUnion>('Mac_Arm')
  const handleChange = (value: ValUnion) => {
    setOs(value)
  }

  console.log(getfile('Win', '1226644', 'chromedriver_win32.zip'))

  return (
    <div>
      <Select
        style={{ width: 120 }}
        value={os}
        onChange={handleChange}
        options={OSList.map((item) => ({
          value: item.val,
          label: item.val,
        }))}
      />
    </div>
  )
}
