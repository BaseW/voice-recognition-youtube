#!/bin/bash

VOSK_MODEL_DOWNLOAD_LINK="https://alphacephei.com/vosk/models/vosk-model-ja-0.22.zip"
VOSK_MODEL_ZIP="vosk-model-ja-0.22.zip"
VOSK_MODEL_DIR="vosk-model-ja-0.22"
VOSK_MAC_DLL_DOWNLOAD_LINK="https://github.com/alphacep/vosk-api/releases/download/v0.3.42/vosk-osx-0.3.42.zip"
VOSK_LINUX_DLL_DOWNLOAD_LINK="https://github.com/alphacep/vosk-api/releases/download/v0.3.42/vosk-linux-x86_64-0.3.42.zip"
VOSK_MAC_DLL_ZIP="vosk-osx-0.3.42.zip"
VOSK_LINUX_DLL_ZIP="vosk-linux-x86_64-0.3.42.zip"
VOSK_MAC_DLL_DIR="vosk-osx-0.3.42"
VOSK_LINUX_DLL_DIR="vosk-linux-x86_64-0.3.42"

# check OS
if [ "$(uname)" == 'Darwin' ]; then
  OS='Mac'
elif [ "$(expr substr $(uname -s) 1 5)" == 'Linux' ]; then
  OS='Linux'
else
  echo "Your platform ($(uname -a)) is not supported."
  exit 1
fi

# download vosk model zip if not exists
if [ -d $VOSK_MODEL_ZIP ]; then
  echo "$VOSK_MODEL_ZIP already exists."
else
  echo "Downloading $VOSK_MODEL_ZIP..."
  curl -L -o $VOSK_MODEL_ZIP $VOSK_MODEL_DOWNLOAD_LINK
fi
# unzip vosk model zip if not exists
if [ -d $VOSK_MODEL_DIR ]; then
  echo "$VOSK_MODEL_DIR already exists."
else
  echo "Unzipping $VOSK_MODEL_ZIP..."
  unzip $VOSK_MODEL_ZIP
fi

# download dynamic library zip
if [ $OS == 'Mac' ]; then
  if [ -d $VOSK_MAC_DLL_ZIP ]; then
    echo "$VOSK_MAC_DLL_ZIP already exists."
  else
    echo "Downloading $VOSK_MAC_DLL_ZIP..."
    curl -L -o $VOSK_MAC_DLL_ZIP $VOSK_MAC_DLL_DOWNLOAD_LINK
  fi
elif [ $OS == 'Linux' ]; then
  if [ -d $VOSK_LINUX_DLL_ZIP ]; then
    echo "$VOSK_LINUX_DLL_ZIP already exists."
  else
    echo "Downloading $VOSK_LINUX_DLL_ZIP..."
    curl -L -o $VOSK_LINUX_DLL_ZIP $VOSK_LINUX_DLL_DOWNLOAD_LINK
  fi
fi

# unzip dynamic library zip
if [ $OS == 'Mac' ]; then
  if [ -d $VOSK_MAC_DLL_DIR ]; then
    echo "$VOSK_MAC_DLL_DIR already exists."
  else
    echo "Unzipping $VOSK_MAC_DLL_ZIP..."
    unzip $VOSK_MAC_DLL_ZIP
  fi
elif [ $OS == 'Linux' ]; then
  if [ -d $VOSK_LINUX_DLL_DIR ]; then
    echo "$VOSK_LINUX_DLL_DIR already exists."
  else
    echo "Unzipping $VOSK_LINUX_DLL_ZIP..."
    unzip $VOSK_LINUX_DLL_ZIP
  fi
fi

# create tmp directory
mkdir -p tmp
