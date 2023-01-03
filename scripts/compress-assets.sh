#!/usr/bin/env bash

OS="$1"
TARGET="$2"
RELEASE_VERSION="$3"

if [ "$OS" = "windows-2022" ]; then
  7z a -tzip "bilal-$RELEASE_VERSION-$TARGET.zip" bilal-"$RELEASE_VERSION"/
else
  tar -czvf bilal-"$RELEASE_VERSION"-"$TARGET".tar.gz bilal-"$RELEASE_VERSION"/
  shasum -a 512 bilal-"$RELEASE_VERSION"-"$TARGET".tar.gz >bilal-"$RELEASE_VERSION"-"$TARGET".tar.gz.sha512
fi
