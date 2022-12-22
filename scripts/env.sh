#!/usr/bin/sh

# these can be included in makefile

export CRATE_NAME="4soul-motivation"
export FERRIS_COMPILE_ERROR="static/icons/ferris/ferris-compile-error.png"
export FERRIS_SUCCESSFUL_COMPILE="static/icons/ferris/ferris-successful-compile.png"
export CARGO_CHECK_ALL="cargo check --quiet --workspace --all-features"
# export COMPILE_ERROR_WAV=$(readlink -fn "./static/audio/error.wav")
# export SUCCESSFUL_COMPILE_WAV=$(readlink -fn "./static/audio/great.wav")
export COMPILE_ERROR_WAV="static/audio/error.wav"
export SUCCESSFUL_COMPILE_WAV="static/audio/great.wav"
