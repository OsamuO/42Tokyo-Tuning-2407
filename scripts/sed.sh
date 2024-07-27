#!/bin/bash

# CSVファイルのパス
csv_file="../webapp/mysql/init/csv/users.csv"

# 一時ファイルのパス
temp_file=$(mktemp)

# 0.pngから9.pngとdefault.pngをそれぞれ0_compressed.jpgから9_compressed.jpgとdefault_compressed.jpgに置換
sed -e 's/0.jpg/0_compressed.png/g' \
    -e 's/1.jpg/1_compressed.png/g' \
    -e 's/2.jpg/2_compressed.png/g' \
    -e 's/3.jpg/3_compressed.png/g' \
    -e 's/4.jpg/4_compressed.png/g' \
    -e 's/5.jpg/5_compressed.png/g' \
    -e 's/6.jpg/6_compressed.png/g' \
    -e 's/7.jpg/7_compressed.png/g' \
    -e 's/8.jpg/8_compressed.png/g' \
    -e 's/9.jpg/9_compressed.png/g' \
    -e 's/default.jpg/default_compressed.png/g' \
    "$csv_file" > "$temp_file"

# 一時ファイルを元のCSVファイルに上書き
mv "$temp_file" "$csv_file"

echo "置換が完了しました: $csv_file"
