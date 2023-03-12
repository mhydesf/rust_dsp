#!/usr/bin/bash

# Get Current Diretory Info
DIR=$(pwd)
BASE="$(basename $DIR)"

if [ "${BASE}" != "rust_dsp" ]
then
    echo "Stopping Script! Please run this script from the rust_dsp directory"
    exit 1
fi

# Make Virtualenvironment
mkdir venv
cd venv
virtualenv -p /usr/bin/python3.10 DSP
source DSP/bin/activate
cd ../
pip install -r requirements.txt
deactivate

# Run Python Initialization Script
python3 tools/initialize.py
