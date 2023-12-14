import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { Button, Input } from 'antd'
import './App.css'
import LogViewer from './log'
import Chrome from './chrome'

function App() {
  const [python_status, setPython] = useState('')
  const [times, setTimes] = useState(0)
  const [chormeV, setChormeV] = useState('')
  const [settings, setSettings] = useState<any>()
  const [python_path, setpython_path] = useState('')

  const init_fun = async () => {
    await invoke('get_chrome_version_command')
      .then((res: any) => setChormeV(res))
      .catch(() => setChormeV(''))

    await invoke('read_json_command')
      .then((res: any) => setSettings(res))
      .catch(() => setSettings(''))
  }

  useEffect(() => {
    init_fun()
  }, [])
  useEffect(() => {
    // 页面加载完成后通知 Tauri 显示窗口
    invoke('app_ready')
  }, [])

  const updateSe = async () => {
    await invoke('update_json_command', {
      data: {
        python_path,
      },
    })
    await init_fun()
  }

  return (
    <div className="container">
      <h1>Settings</h1>
      <p>{settings?.python_path}</p>
      <p>{settings?.chromedriver}</p>
      <Input
        value={python_path}
        onChange={(e) => setpython_path(e.target.value)}
      />
      <Button onClick={updateSe}>修改 Settings</Button>
      <h1>{chormeV}</h1>
      <Chrome />
      <h2>Python：3.11.5</h2>
      <Button
        onClick={async () => {
          const time1 = +new Date()
          setPython(await invoke('execute_python_script'))
          const time2 = +new Date()
          setTimes(time2 - time1)
        }}
      >
        测试Python
      </Button>
      <p>{python_status}</p>
      <p>耗时：{times / 1000} s</p>
      <LogViewer />
    </div>
  )
}

export default App
