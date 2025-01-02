#!/bin/env sh

# Check for necessary binaries 🕵
WASM_PACK=$(which wasm-pack)
if [ -z "$WASM_PACK" ]; then
    echo "'wasm-pack' binary not found in PATH."
    exit 1
fi

PYTHON=$(which python3 || which python)
if [ "$1" = "run" ] && [ -z "$PYTHON" ]; then
    echo "'python3' or 'python' binary not found in PATH."
    exit 1
fi

# Check for necessary files ⚖
if [ ! -d ./html ]; then
    echo "'./html' folder not found."
    exit 1
fi

# Build WASM binary 🏗
rm -rf ./public && mkdir ./public
$WASM_PACK build --target web

# Prepare app 👩‍🍳
if [ ! -d ./pkg ]; then
    echo "'./pkg' folder not found."
    exit 1
fi

mv ./pkg ./public
cp -a ./html/. ./public

# Run app locally (If needed) 🚀
if [ "$1" = "run" ]; then
    cd ./public
    $PYTHON -m http.server
fi
