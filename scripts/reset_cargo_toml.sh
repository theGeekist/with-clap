#!/bin/bash

# Step 1: Extract metadata from Cargo.toml
PACKAGE_NAME=$(grep -oP '(?<=^name = ").*(?="$)' Cargo.toml)
PACKAGE_VERSION=$(grep -oP '(?<=^version = ").*(?="$)' Cargo.toml)
PACKAGE_AUTHORS=$(grep -oP '(?<=^authors = \[").*(?="\])' Cargo.toml)

# Step 2: Backup and remove Cargo.toml
mv Cargo.toml Cargo.toml.bak

# Step 3: Create a new Cargo.toml with package metadata
cat <<EOF > Cargo.toml
[package]
name = "$PACKAGE_NAME"
version = "$PACKAGE_VERSION"
authors = ["$PACKAGE_AUTHORS"]
edition = "2021"
EOF

# Step 4: Add dependencies with locked versions
cargo add clap log pretty_env_logger serde --features derive serde_json
cargo add --dev cargo-watch cargo-tarpaulin cargo-audit clippy
