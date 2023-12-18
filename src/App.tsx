import { useEffect, useState } from 'react'
import { Button, Input } from 'antd'
import './App.css'
import LogViewer from './log'
import Chrome from './chrome'
import { tyInvoke } from './invoke'
import Verify from './verify'

function App() {
  const [times, setTimes] = useState(0)
  const [chormeV, setChormeV] = useState('')
  const [settings, setSettings] = useState<any>()
  const [python_path, setpython_path] = useState('')

  const init_fun = async () => {
    await tyInvoke('get_chrome_version_command')
      .then((res: any) => setChormeV(res))
      .catch(() => setChormeV(''))

    await tyInvoke('read_json_command')
      .then((res: any) => setSettings(res))
      .catch(() => setSettings(''))
  }

  useEffect(() => {
    init_fun()
  }, [])
  useEffect(() => {
    // 页面加载完成后通知 Tauri 显示窗口
    tyInvoke('app_ready')
  }, [])

  const updateSe = async () => {
    await tyInvoke('update_json_command', {
      data: {
        ...settings,
        python_path,
      },
    })
    await init_fun()
  }

  return (
    <div className="container">
      <Verify />
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
          await tyInvoke('init_python_path')
          const time2 = +new Date()
          setTimes(time2 - time1)
        }}
      >
        设置 Python 环境
      </Button>
      <Button
        onClick={async () => {
          const time1 = +new Date()
          await tyInvoke('execute_python_script', {
            cmdType: 'Python',
            pyFile: 'sele.pyc',
          })
          const time2 = +new Date()
          setTimes(time2 - time1)
        }}
      >
        测试Python
      </Button>
      <Button
        onClick={async () => {
          const time1 = +new Date()
          await tyInvoke('execute_python_script', {
            cmdType: 'Pip',
            pyFile: '',
          })
          const time2 = +new Date()
          setTimes(time2 - time1)
        }}
      >
        安装依赖
      </Button>
      <p>耗时：{times / 1000} s</p>
      <LogViewer />
    </div>
  )
}

export default App
