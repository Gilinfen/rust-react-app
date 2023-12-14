# 这是一个示例 Python 脚本。

# 按 ⌃R 执行或将其替换为您的代码。
# 按 双击 ⇧ 在所有地方搜索类、文件、工具窗口、操作和设置。
import json
import os

# 获取当前工作目录
current_directory = os.getcwd()

# 打印当前工作目录
print(f"当前所在目录：{current_directory}")

# 打开并读取JSON文件
with open('./settings.json', 'r') as file:
    data = json.load(file)

# 打印读取的数据
print(f"JSON 对象：{data}")




def print_hi(name):
    # 在下面的代码行中使用断点来调试脚本。
    print(f'Hi, {name}')  # 按 ⌘F8 切换断点。


# 按间距中的绿色按钮以运行脚本。
if __name__ == '__main__':
    print_hi('PyCharm')

# 访问 https://www.jetbrains.com/help/pycharm/ 获取 PyCharm 帮助
