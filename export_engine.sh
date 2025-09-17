#! /bin/bash
cd rust_chess
maturin develop

cd ..

pyinstaller --onefile -c --paths=D:.venv/lib/python3.11/site-packages main_uci.py

mv dist/main_uci.exe ../lichess-bot/engines/main_uci.exe