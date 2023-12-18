import { Button, Select, message } from 'antd'
import { useEffect, useState } from 'react'
import { OSList } from './chromium'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'

type ValUnion = (typeof OSList)[number]['val']

export default function Chrome() {
  const [os, setOs] = useState<ValUnion>()
  const [dwF, setDwF] = useState(false)
  const handleChange = (value: ValUnion) => {
    setOs(value)
  }

  useEffect(() => {
    invoke('get_os_info').then((res: any) => setOs(res))
    listen('message-download-chromedriver', (event: any) => {
      const data = JSON.parse(event.payload)
      if (data.data === 200) {
        message.success(data.message)
      } else {
        message.error(data.message)
      }
    })
  }, [])

  const downloads = async () => {
    setDwF(true)
    await invoke('download_chromedriver', {
      params: {
        os: 'Mac_Arm',
        position: '1213882',
        files: 'chromedriver_mac64.zip',
      },
    }).catch((err) => {
      console.log(err)
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
      <Button onClick={downloads} disabled={dwF}>
        下载 chromedriver
      </Button>
    </div>
  )
}
