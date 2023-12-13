import { Button, Select } from 'antd'
import { useEffect, useState } from 'react'
import { OSList } from './chromium'
import { invoke } from '@tauri-apps/api/tauri'

type ValUnion = (typeof OSList)[number]['val']

export default function Chrome() {
  const [os, setOs] = useState<ValUnion>()
  const handleChange = (value: ValUnion) => {
    setOs(value)
  }

  useEffect(() => {
    invoke('get_os_info').then((res: any) => setOs(res))
  }, [])

  const downloads = async () => {
    await invoke('download_chromedriver', {
      val: 'Win',
      position: '1226644',
      files: 'chromedriver_win32.zip',
    })
  }

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
      <Button onClick={downloads}>下载 chromedriver</Button>
    </div>
  )
}
