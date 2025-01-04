#!/bin/env sh

# Check environment 🤠
if [ "$BUILD_ENV" = "PROD" ]; then
    PROFILE="--release"
    EXTRA_MSG=""
else
    PROFILE="--dev"
    EXTRA_MSG="To run as PROD, use 'BUILD_ENV=\"PROD\" sh build.sh'\n"
fi

echo "Building '$PROFILE' profile."
echo -n "$EXTRA_MSG"
echo "---"

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

# Build Wasm binary 🏗
rm -rf ./public && mkdir ./public
$WASM_PACK build $PROFILE --target web

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

    echo "---"
    $PYTHON -m http.server
fi
