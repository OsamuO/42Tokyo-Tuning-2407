#!/bin/bash

# 画像が保存されているディレクトリのパス
input_dir="../webapp/backend/images/user_profile"

# 出力ディレクトリのパス（必要に応じて変更してください）
output_dir="../webapp/backend/images/user_profile"

# 出力ディレクトリが存在しない場合は作成
mkdir -p "$output_dir"

# ディレクトリ内のすべての .png ファイルをループ処理
for input_file in "$input_dir"/*.png
do
    # ファイル名のみを取得
    filename=$(basename "$input_file" .png)

    # 出力ファイルパス
    output_file="$output_dir/$filename.jpg"

    # ffmpegを使って変換と圧縮
    ffmpeg -i "$input_file" -qscale:v 2 "$output_file"
done

echo "すべての .png ファイルを圧縮された .jpg に変換しました。"
