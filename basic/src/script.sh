#!/bin/bash

# 引数が渡されているかを確認する
if [ -z "$1" ]
  then
    echo "引数がありません。"
    exit 1
fi

# 引数を取得する
n=$1

# indexN.rsにリネームする
mv main.rs "index$n.rs"

# 新しいmain.rsを作成する
echo "fn main() {}" > main.rs