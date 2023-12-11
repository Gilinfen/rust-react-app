import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { Button, Input } from 'antd'
import './App.css'
import LogViewer from './log'

function App() {
  const [python_status, setPython] = useState('')
  const [pythonPath, setPythonPath] = useState<string>()
  const [times, setTimes] = useState(0)
  const [chormeV, setChormeV] = useState('')

  const init_fun = async () => {
    setChormeV(await invoke('get_chrome_version_command'))
  }

  useEffect(() => {
    init_fun()
  }, [])

  return (
    <div className="container">
      <h1>{chormeV}</h1>
      <h2>Python：3.11.5</h2>
      <Input
        id="python"
        value={pythonPath}
        onChange={(e) => setPythonPath(e.currentTarget.value.trim())}
        placeholder="手动输入python地址"
      />
      <Button
        onClick={async () => {
          if (!pythonPath) return
          await invoke('python_path', {
            path: pythonPath,
          })
        }}
      >
        手动设置 python
      </Button>
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
