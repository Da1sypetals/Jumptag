#!/bin/bash

# 默认使用debug模式
BUILD_MODE=""

# 检查是否提供了-r参数
if [ "$1" == "-r" ]; then
  BUILD_MODE="release"
fi

# 使用cargo进行编译
cargo build --$BUILD_MODE

# 检查编译是否成功
if [ $? -ne 0 ]; then
  echo "[jt install] compile failed."
  exit 1
fi

# 复制编译后的二进制文件到当前目录
cp target/$BUILD_MODE/jt-inner ./

# 复制编译后的二进制文件到/usr/bin/
sudo cp target/$BUILD_MODE/jt-inner /usr/bin/

echo "[jt install] compile and install completed."
