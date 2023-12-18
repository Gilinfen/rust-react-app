import os
import shutil

def move_py_files(src_directory, dst_directory, ignore_dirs):
    # 检查并删除现有的目标目录
    if os.path.exists(dst_directory):
        shutil.rmtree(dst_directory)

    for root, dirs, files in os.walk(src_directory, topdown=True):
        # 修改 dirs，排除忽略的文件夹
        dirs[:] = [d for d in dirs if os.path.join(root, d) not in ignore_dirs]

        for file in files:
            if file.endswith('.py'):
                src_file_path = os.path.join(root, file)
                dst_file_path = os.path.join(dst_directory, os.path.relpath(root, src_directory), file)

                # 确保目标目录存在
                os.makedirs(os.path.dirname(dst_file_path), exist_ok=True)

                # 复制文件
                shutil.copy2(src_file_path, dst_file_path)
    
    # 复制 requirements.txt 文件到目标目录
    src_requirements_path = os.path.join(src_directory, 'requirements.txt')
    dst_requirements_path = os.path.join(dst_directory, 'requirements.txt')
    if os.path.exists(src_requirements_path):
        shutil.copy2(src_requirements_path, dst_requirements_path)

# 源目录和目标目录
src_directory = './python'
dst_directory = './pythonrc'

# 定义要忽略的目录名称
ignore_dir_names = ['.idea', 'build', 'dist', 'venv']

# 使用列表推导创建忽略目录的完整路径集合
ignore_dirs = {os.path.join(src_directory, d) for d in ignore_dir_names}

move_py_files(src_directory, dst_directory, ignore_dirs)
