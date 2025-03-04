#!/bin/bash
set -e

echo "Running post-create setup script..."

# Make the script executable
chmod +x ${0}

# Setup Backend
echo "Setting up Rust backend..."
cargo check

# Setup Frontend
echo "Setting up SvelteKit frontend..."
cd ui
npm install

echo "Setup complete! You can now run:"
echo "- For backend: cargo run"
echo "- For frontend: cd ui && npm run dev"