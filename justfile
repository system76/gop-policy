# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2024 System76, Inc.

# List available recipes
@help:
    just --list

# Build the project
@build *args='':
    cargo build {{args}}

# Generate API documentation
@doc *args='':
    cargo doc --workspace --document-private-items --no-deps {{args}}

# Run clippy
@clippy *args='':
    cargo clippy --all-features {{args}}

# Run rustfmt
@fmt *args='':
    cargo fmt --all {{args}}

# Remove build artifacts
@clean:
    cargo clean
