#!/bin/bash

# 画像が保存されているディレクトリのパス
input_dir="../webapp/backend/images/user_profile"

# 圧縮品質を指定 (ここでは例として20を使用)
quality=20
# ディレクトリ内のすべての .png ファイルをループ処理
for input_file in "$input_dir"/*.png
do
  # ファイル名と拡張子を分離
  filename=$(basename "$input_file" .png)

  # 出力ファイルのパスを生成
  output_file="$input_dir/${filename}_compressed.png"

  # ffmpeg を使用して画像を圧縮
  ffmpeg -i "$input_file" -vf "scale=500:500:force_original_aspect_ratio=decrease,pad=500:500:(ow-iw)/2:(oh-ih)/2" -q:v "$quality" "$output_file"

  echo "圧縮が完了しました: $output_file"
done
