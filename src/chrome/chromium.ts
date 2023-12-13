export const OSList = [
  { val: 'Mac', file: 'chrome-mac.zip', diver: 'chromedriver_mac64.zip' },
  { val: 'Mac_Arm', file: 'chrome-mac.zip', diver: 'chromedriver_mac64.zip' },
  { val: 'Win_x64', file: 'chrome-win.zip', diver: 'chromedriver_win32.zip' },
  { val: 'Win', file: 'chrome-win32.zip', diver: 'chromedriver_win32.zip' },
  // { val: 'Linux_x64', file: 'chrome-linux.zip' },
  // { val: 'Linux', file: 'chrome-linux.zip' },
  // { val: 'Android', file: 'chrome-android.zip' },
  // { val: 'Arm', file: 'chrome-linux.zip' },
] as const

export const DownloadUrl = {
  example:
    'https://commondatastorage.googleapis.com/chromium-browser-snapshots/index.html?prefix=Mac/584585/',
  base: 'https://commondatastorage.googleapis.com/chromium-browser-snapshots/index.html?prefix=',
}
