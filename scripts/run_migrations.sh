#!/bin/bash

# This script runs database migrations for the Quantum Chess application

# Ensure the script fails on any error
set -e

# Load environment variables
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
else
    echo "Error: .env file not found"
    exit 1
fi

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "Error: DATABASE_URL is not set. Please check your .env file."
    exit 1
fi

echo "Running migrations on $DATABASE_URL"

# Check if diesel CLI is installed
if ! command -v diesel &> /dev/null; then
    echo "Diesel CLI not found. Installing..."
    cargo install diesel_cli --no-default-features --features postgres
fi

# Run the migrations
diesel migration run

echo "Migrations completed successfully!"
