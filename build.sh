#!/bin/bash

# 默认使用debug模式
BUILD_MODE="debug"

# 检查是否提供了-r参数
if [ "$1" == "-r" ]; then
  cargo build --release
else
  cargo build
fi

# 检查编译是否成功
if [ $? -ne 0 ]; then
  echo "[jt install] compile failed."
  exit 1
fi

# 复制编译后的二进制文件到/usr/bin/
cp target/$BUILD_MODE/jt ./jt-inner
sudo cp target/$BUILD_MODE/jt /usr/bin/jt-inner

echo "[jt install] compile and install completed."
