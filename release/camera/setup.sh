#!/bin/bash

# clean up previous venv
rm -rf .venv

# create new venv
python3 -m venv .venv

# enter the virtual environment
. .venv/bin/activate

pip install -r requirements.txt

deactivate
