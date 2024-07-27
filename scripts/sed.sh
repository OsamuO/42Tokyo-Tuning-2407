#!/bin/bash

# CSVファイルのパス
csv_file="../../42Tokyo-Tuning-2407/webapp/mysql/init/csv/users.csv"

# 一時ファイルのパス
temp_file=$(mktemp)

# 0.pngから9.pngとdefault.pngをそれぞれ0_compressed.jpgから9_compressed.jpgとdefault_compressed.jpgに置換
sed -e 's/0_compressed.jpg/0_compressed.png/g' \
    -e 's/1_compressed.jpg/0_compressed.png/g' \
    -e 's/2_compressed.jpg/0_compressed.png/g' \
    -e 's/3_compressed.jpg/0_compressed.png/g' \
    -e 's/4_compressed.jpg/0_compressed.png/g' \
    -e 's/5_compressed.jpg/0_compressed.png/g' \
    -e 's/6_compressed.jpg/0_compressed.png/g' \
    -e 's/7_compressed.jpg/0_compressed.png/g' \
    -e 's/8_compressed.jpg/0_compressed.png/g' \
    -e 's/9_compressed.jpg/0_compressed.png/g' \
    -e 's/default_compressed.jpg/default_compressed.png/g' \
    "$csv_file" > "$temp_file"

# 一時ファイルを元のCSVファイルに上書き
mv "$temp_file" "$csv_file"

echo "置換が完了しました: $csv_file"
