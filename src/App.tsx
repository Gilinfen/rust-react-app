import { useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { Button, Input } from 'antd'
import './App.css'

function App() {
  const [greetMsg, setGreetMsg] = useState('')
  const [name, setName] = useState('')
  const [python_status, setPython] = useState('')

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
          setPython(await invoke('execute_python_script'))
        }}
      >
        测试Python
      </Button>
      <p>{python_status}</p>
    </div>
  )
}

export default App
