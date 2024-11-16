#!/usr/bin/env bash

# check git is installed
if ! [ -x "$(command -v git)" ]; then
  echo 'Error: git is not installed.' >&2
  exit 1
fi

# check cargo is installed
if ! [ -x "$(command -v cargo)" ]; then
  echo 'Error: cargo is not installed.' >&2
  exit 1
fi

git clone https://github.com/Young-TW/cmake-init.git
cd cmake-init || exit
cargo install --path .
cd .. || exit
