#!/usr/bin/env bash

set -e
shopt -s extglob nullglob
directory=$1
backup_dir="$directory/original"
extensions="${@:2}"
extensions="${extensions:-m4a aac}"
echo $extensions

if [ ! -d "$backup_dir" ];
then
    echo "Creating $backup_dir directory."
    mkdir "$backup_dir"
fi

for ext in $extensions; do
    for audio in "$directory"/*.$ext; do
        noext=$(basename "$audio")
        noext="${noext%.$ext}"
        echo $noext
        ffmpeg -i "$audio" -f flac "converted.flac"
        mv "$audio" "$backup_dir"
        mv "converted.flac" "$directory/${noext// /_}.flac"
    done
done
