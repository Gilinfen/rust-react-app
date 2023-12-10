import { useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { Button, Input } from 'antd'
import './App.css'

function App() {
  const [greetMsg, setGreetMsg] = useState('')
  const [name, setName] = useState('')
  const [python_status, setPython] = useState('')
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
    </div>
  )
}

export default App
