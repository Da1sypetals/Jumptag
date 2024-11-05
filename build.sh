#!/bin/bash

# 检查是否至少有一个参数
if [ $# -lt 1 ]; then
  echo "Usage: bash build.sh <path_to_rc_file> [-r]"
  exit 1
fi

# 检查第一个参数是否为有效路径
if [ ! -f "$1" ]; then
  echo "Error: The first argument must be a valid path to your existing shell .rc file."
  exit 1
fi

# 默认使用debug模式
BUILD_MODE="debug"

# 检查第二个参数是否为-r
if [ "$2" == "-r" ]; then
  BUILD_MODE="release"
elif [ "$2" != "" ]; then
  echo "Error: The second argument must be '-r' or nothing."
  exit 1
fi

# 编译
if [ "$BUILD_MODE" == "release" ]; then
  cargo build --release
else
  cargo build
fi

# 检查编译是否成功
if [ $? -ne 0 ]; then
  echo "[jt install] compile failed."
  exit 1
fi

# 复制编译后的二进制文件到~/.jumptag/bin/
mkdir -p ~/.jumptag/bin
cp target/$BUILD_MODE/jt ~/.jumptag/bin/jt-inner

# 初始化并加载rc文件
~/.jumptag/bin/jt-inner -init "$1"

if [ $? -ne 0 ]; then
  echo "[jt install] initialization failed."
  exit 1
fi

echo "[jt install] compile and install completed, please restart shell to apply."
