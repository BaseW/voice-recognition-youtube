#!/bin/bash

VOSK_MODEL_DOWNLOAD_LINK="https://alphacephei.com/vosk/models/vosk-model-ja-0.22.zip"
VOSK_MODEL_ZIP="vosk-model-ja-0.22.zip"
VOSK_MODEL_DIR="vosk-model-ja-0.22"
VOSK_MAC_DLL_DOWNLOAD_LINK="https://github.com/alphacep/vosk-api/releases/download/v0.3.42/vosk-osx-0.3.42.zip"
VOSK_LINUX_DLL_DOWNLOAD_LINK="https://github.com/alphacep/vosk-api/releases/download/v0.3.42/vosk-linux-x86_64-0.3.42.zip"
VOSK_MAC_DLL_ZIP="vosk-osx-0.3.42.zip"
VOSK_LINUX_DLL_ZIP="vosk-linux-x86_64-0.3.42.zip"
VOSK_MAC_DLL_PATH="libvosk.dylib"
VOSK_LINUX_DLL_PATH="libvosk.so"

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
if [ -e $VOSK_MODEL_ZIP ]; then
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
  if [ -e $VOSK_MAC_DLL_ZIP ]; then
    echo "$VOSK_MAC_DLL_ZIP already exists."
  else
    echo "Downloading $VOSK_MAC_DLL_ZIP..."
    curl -L -o $VOSK_MAC_DLL_ZIP $VOSK_MAC_DLL_DOWNLOAD_LINK
  fi
elif [ $OS == 'Linux' ]; then
  if [ -e $VOSK_LINUX_DLL_ZIP ]; then
    echo "$VOSK_LINUX_DLL_ZIP already exists."
  else
    echo "Downloading $VOSK_LINUX_DLL_ZIP..."
    curl -L -o $VOSK_LINUX_DLL_ZIP $VOSK_LINUX_DLL_DOWNLOAD_LINK
  fi
fi

# unzip dynamic library zip
if [ $OS == 'Mac' ]; then
  if [ -e $VOSK_MAC_DLL_PATH ]; then
    echo "$VOSK_MAC_DLL_PATH already exists."
  else
    echo "Unzipping $VOSK_MAC_DLL_ZIP..."
    # unzip files to current directory
    unzip $VOSK_MAC_DLL_ZIP -d .
    # move libvosk.dylib to current directory
    mv ./vosk-osx-0.3.42/libvosk.dylib .
    # move vosk.h to current directory
    mv ./vosk-osx-0.3.42/vosk_api.h .
    # remove directory
    rm -rf ./vosk-osx-0.3.42
  fi
elif [ $OS == 'Linux' ]; then
  if [ -e $VOSK_LINUX_DLL_PATH ]; then
    echo "$VOSK_LINUX_DLL_PATH already exists."
  else
    echo "Unzipping $VOSK_LINUX_DLL_ZIP..."
    unzip $VOSK_LINUX_DLL_ZIP -d .
    mv ./vosk-linux-x86_64-0.3.42/libvosk.so .
    mv ./vosk-linux-x86_64-0.3.42/vosk_api.h .
    rm -rf ./vosk-linux-x86_64-0.3.42
  fi
fi

# create tmp directory
mkdir -p tmp
