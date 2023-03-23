#!/bin/bash

WHISPER_MODEL_DOWNLOAD_LINK="https://huggingface.co/datasets/ggerganov/whisper.cpp/resolve/main/ggml-small.bin"
WHISPER_MODEL_PATH="whisper-models/ggml-small.bin"
YT_DLP_MAC_DOWNLOAD_LINK="https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos"
YT_DLP_LINUX_DOWNLOAD_LINK="https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp"
YT_DLP_PATH="/usr/local/bin/yt-dlp"

# check OS
if [ "$(uname)" == 'Darwin' ]; then
  OS='Mac'
elif [ "$(expr substr $(uname -s) 1 5)" == 'Linux' ]; then
  OS='Linux'
else
  echo "Your platform ($(uname -a)) is not supported."
  exit 1
fi

# download whisper model
if [ -e $WHISPER_MODEL_PATH ]; then
  echo "$WHISPER_MODEL_PATH already exists."
else
  echo "Downloading $WHISPER_MODEL_PATH..."
  curl -L -o $WHISPER_MODEL_PATH $WHISPER_MODEL_DOWNLOAD_LINK
fi
[ -e $WHISPER_MODEL_PATH ] || { echo "$WHISPER_MODEL_PATH not found."; exit 1; }

# download yt-dlp
if [ -e $YT_DLP_PATH ]; then
  echo "$YT_DLP_PATH already exists."
else
  echo "Downloading $YT_DLP_PATH..."
  if [ $OS == 'Mac' ]; then
    sudo curl -L -o $YT_DLP_PATH $YT_DLP_MAC_DOWNLOAD_LINK
  elif [ $OS == 'Linux' ]; then
    sudo curl -L -o $YT_DLP_PATH $YT_DLP_LINUX_DOWNLOAD_LINK
  fi
  sudo chmod +x $YT_DLP_PATH
fi

# create tmp directory
mkdir -p tmp
