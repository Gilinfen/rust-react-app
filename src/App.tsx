import { useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { Button, Input } from 'antd'
import './App.css'
import LogViewer from './log'

function App() {
  const [greetMsg, setGreetMsg] = useState('')
  const [name, setName] = useState('')
  const [python_status, setPython] = useState('')
  const [pythonPath, setPythonPath] = useState<string>()
  const [times, setTimes] = useState(0)

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke('greet', { name }))
  }

  return (
    <div className="container">
      <Input
        id="greet-input"
        onChange={(e) => setName(e.currentTarget.value)}
        placeholder="Enter a name..."
      />
      <Button
        onClick={async () => {
          await greet()
        }}
        type="primary"
      >
        Greet
      </Button>
      <p>{greetMsg}</p>
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
