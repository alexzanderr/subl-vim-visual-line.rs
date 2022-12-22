#!/usr/bin/sh

icon_full_path=$(readlink -fn $1)

# echo "'$SUCCESSFUL_COMPILE_WAV;"
wav_full_path=$(readlink -fn $2)

# echo "file: $wav_full_path"
# play -q $wav_full_path

notify-send "❱ cargo check --all-features" "crate: $CRATE_NAME\n<b>successful compilation </b> \!" --icon=$icon_full_path -t 2000