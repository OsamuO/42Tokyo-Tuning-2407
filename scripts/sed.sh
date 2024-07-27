#!/bin/bash

# CSVファイルのパス
csv_file="パスを入力してください/42Tokyo-Tuning-2407/webapp/mysql/init/csv/users.csv"

# 一時ファイルのパス
temp_file=$(mktemp)

# 0.pngから9.pngとdefault.pngをそれぞれ0_compressed.jpgから9_compressed.jpgとdefault_compressed.jpgに置換
sed -e 's/0.png/0_compressed.jpg/g' \
    -e 's/1.png/1_compressed.jpg/g' \
    -e 's/2.png/2_compressed.jpg/g' \
    -e 's/3.png/3_compressed.jpg/g' \
    -e 's/4.png/4_compressed.jpg/g' \
    -e 's/5.png/5_compressed.jpg/g' \
    -e 's/6.png/6_compressed.jpg/g' \
    -e 's/7.png/7_compressed.jpg/g' \
    -e 's/8.png/8_compressed.jpg/g' \
    -e 's/9.png/9_compressed.jpg/g' \
    -e 's/default.png/default_compressed.jpg/g' \
    "$csv_file" > "$temp_file"

# 一時ファイルを元のCSVファイルに上書き
mv "$temp_file" "$csv_file"

echo "置換が完了しました: $csv_file"
