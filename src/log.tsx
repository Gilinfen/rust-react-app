import { useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'

const LogViewer = () => {
  const [logs, setLogs] = useState<any[]>([])

  useEffect(() => {
    const unlisten = listen('log-message', (event) => {
      setLogs((currentLogs) => [...currentLogs, event.payload])
    })

    return () => {
      unlisten.then((f) => f())
    }
  }, [])

  return (
    <div>
      {logs.map((log, index) => (
        <p key={index}>{log}</p>
      ))}
    </div>
  )
}

export default LogViewer
