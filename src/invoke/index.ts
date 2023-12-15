import { invoke } from '@tauri-apps/api'
import { InvokeArgs } from '@tauri-apps/api/tauri'

export type InvokeFn = {
  execute_python_script: string
  get_chrome_version_command: string
  download_chromedriver: string
  update_json_command: string
  read_json_command: string
  get_os_info: string
  app_ready: string
  delayed_restart: string
}

export const tyInvoke = async <T>(
  cmd: keyof InvokeFn,
  args?: InvokeArgs
): Promise<T> => invoke<T>(cmd, args)
