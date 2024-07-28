#!/bin/bash

# CSVファイルのパス
csv_file="../webapp/mysql/init/csv/users.csv"

# 一時ファイルのパス
temp_file=$(mktemp)

# 0.pngから9.pngとdefault.pngをそれぞれ0_compressed.jpgから9_compressed.jpgとdefault_compressed.jpgに置換
sed -e 's/0_compressed.png/0.jpg/g' \
    -e 's/1_compressed.png/1.jpg/g' \
    -e 's/2_compressed.png/2.jpg/g' \
    -e 's/3_compressed.png/3.jpg/g' \
    -e 's/4_compressed.png/4.jpg/g' \
    -e 's/5_compressed.png/5.jpg/g' \
    -e 's/6_compressed.png/6.jpg/g' \
    -e 's/7_compressed.png/7.jpg/g' \
    -e 's/8_compressed.png/8.jpg/g' \
    -e 's/9_compressed.png/9.jpg/g' \
    -e 's/default_compressed.png/default.jpg/g' \
    "$csv_file" > "$temp_file"

# 一時ファイルを元のCSVファイルに上書き
mv "$temp_file" "$csv_file"

echo "置換が完了しました: $csv_file"
