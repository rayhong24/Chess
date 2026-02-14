#!/usr/bin/env bash

set -e  # Exit immediately if a command fails

OS="$(uname -s)"

echo "Detected OS: $OS"

cd rust_chess
maturin develop
cd ..

if [[ "$OS" == "Linux"* ]]; then
    echo "Building for Linux..."
    pyinstaller --onefile -c main_uci.py
    mv dist/main_uci ../lichess-bot/engines/main_uci

elif [[ "$OS" == "Darwin"* ]]; then
    echo "Building for macOS..."
    pyinstaller --onefile -c --paths=D:.venv/lib/python3.11/site-packages main_uci.py
    mv dist/main_uci ../lichess-bot/engines/main_uci

elif [[ "$OS" == "MINGW"* || "$OS" == "MSYS"* || "$OS" == "CYGWIN"* ]]; then
    echo "Building for Windows..."
    pyinstaller --onefile -c \
        --paths=D:.venv/lib/python3.11/site-packages \
        main_uci.py

    mv dist/main_uci.exe ../lichess-bot/engines/main_uci.exe

else
    echo "Unsupported OS: $OS"
    exit 1
fi
