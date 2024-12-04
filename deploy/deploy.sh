#!/bin/bash

set -e

echo "Building contract 1..."
cargo build --release --package contract1

echo "Building contract 2..."
cargo build --release --package contract2

echo "Deploying contracts..."
# Add your deployment commands here, such as uploading to a blockchain
