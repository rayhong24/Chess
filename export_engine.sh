#! /bin/bash
pyinstaller --onefile -c main_uci.py
mv dist/main_uci.exe ../lichess-bot/engines/main_uci.exe