#!/usr/bin/env bash

set -e
shopt -s extglob nullglob
directory=$1
backup_dir="$directory/original"
extensions="${@:2}"
extensions="${extensions:-mp4 MP4}"
echo $extensions

if [ ! -d "$backup_dir" ];
then
    echo "Creating $backup_dir directory."
    mkdir "$backup_dir"
fi

for ext in $extensions; do
    for video in "$directory"/*.$ext; do
        noext=$(basename "$video")
        noext="${noext%.$ext}"
        echo $noext
        ffmpeg -i "$video" -acodec pcm_s16le -vcodec copy "converted.mov"
        mv "$video" "$backup_dir"
        mv "converted.mov" "$directory/${noext// /_}.mov"
    done
done