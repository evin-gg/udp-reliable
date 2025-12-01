#!/usr/bin/env bash

set -e

mkdir ../loggers
mkdir ../loggers/logs
# Name of the venv folder
VENV_DIR="../loggers"

echo "Creating virtual environment..."
python3 -m venv "$VENV_DIR"

echo "Activating virtual environment..."
# shellcheck disable=SC1091
source "$VENV_DIR/bin/activate"

echo "Upgrading pip..."
pip install --upgrade pip

if [ -f "requirements.txt" ]; then
    echo "Installing requirements..."
    pip install -r requirements.txt
else
    echo "requirements.txt not found"
    exit 1
fi

echo "Creating logs directory and log files..."
mkdir -p logs
touch "$VENV_DIR/logs/client.log" 
touch "$VENV_DIR/logs/proxy.log" 
touch "$VENV_DIR/logs/server.log"

echo "Copying python scripts to directory..."
cp client_graph.py proxy_graph.py server_graph.py ../loggers/

echo "Setup complete!"
