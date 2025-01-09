#!/bin/env sh

if [ "$BUILD_ENV" = "PROD" ]; then
    PROFILE="--release"
    EXTRA_MSG="To run as DEV, unset the 'BUILD_ENV' environment variable.\n"
else
    PROFILE="--dev"
    EXTRA_MSG="To run as PROD, use 'BUILD_ENV=\"PROD\" sh build.sh'\n"
fi

echo "Building '$PROFILE' profile."
echo -n "$EXTRA_MSG"

if [ "$1" != "run" ]; then
    echo "To run the app afterwards, use 'run' argument."
fi

echo "---"

# Check for necessary binaries
WASM_PACK=$(which wasm-pack)
if [ -z "$WASM_PACK" ]; then
    echo "'wasm-pack' binary not found in PATH."
    exit 1
fi

PYTHON=$(which python3 || which python)
if [ "$1" = "run" ] && [ -z "$PYTHON" ]; then
    echo "'python3' or 'python' binary not found in PATH. Needed to use 'run' argument."
    exit 1
fi

# Build Wasm pkg
$WASM_PACK build $PROFILE --target web|| exit 1

# Prepare app
if [ ! -d ./html ]; then
    echo "'./html' folder not found."
    exit 1
fi

if [ ! -d ./pkg ]; then
    echo "'./pkg' folder not found."
    exit 1
fi

rm -rf ./public && mkdir ./public
mv ./pkg ./public
cp -a ./html/. ./public

# Run app locally (If needed)
if [ "$1" = "run" ]; then
    cd ./public

    echo "---"
    $PYTHON -m http.server
fi

