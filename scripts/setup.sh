#!/bin/bash

# Add the submodule if it doesn't exist
if [ ! -d "saasy-proto" ]; then
    git submodule add https://github.com/saasybyte/saasy-proto.git saasy-proto
fi

# Update and initialize
git submodule update --init --recursive
git config submodule.saasy-proto.ignore all