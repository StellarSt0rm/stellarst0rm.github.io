#!/bin/env sh

echo "Preprocessing website..."
echo "To run afterwards, use \`run\` argument."
echo "---"

# Check for necssary packages
CARGO=$(which cargo)
if [ -z "$CARGO" ]; then
	echo "You need \`cargo\` to build a Rust program, idiot."
fi

PYTHON=$(which python3 || which python)
if [ "$1" = "run" ] && [ -z "$PYTHON" ]; then
    echo "\`python3\` or \`python\` binary not found in PATH. (Needed to use \`run\` argument.)"
    exit 1
fi

# Preprocess
if [ ! -d ./templates ]; then
	echo "\`templates\` folder not found, are you cd'd into the project's folder?"
	exit 1
fi

if [ -d ./processed.html ]; then
	rm ./processed.html
fi

$CARGO run || exit 1

# Prepare for gh pages
if [ -d ./public ]; then
	rm -r ./public
fi

mkdir ./public
mv ./processed.html ./public/index.html
cp -a ./templates/resources/. ./public

# Run app locally when the argument `run` is passed
if [ "$1" = "run" ]; then
    cd ./public

    echo "---"
    $PYTHON -m http.server
fi
