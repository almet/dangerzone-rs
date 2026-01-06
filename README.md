# dangerzone.rs

A command-line implementation of Dangerzone in Rust.

## Overview

This is a simple Rust implementation of Dangerzone that converts potentially dangerous documents (PDF, Office documents, etc.) into safe PDFs by rendering them to pixels and reconstructing a clean PDF.

## Features

- Uses the official Dangerzone Docker images from `ghcr.io/freedomofpress/dangerzone/v1`
- Supports both podman and docker runtimes
- Streams documents through the conversion process
- Two-phase conversion: document → pixels → safe PDF

## Prerequisites

- Rust (for building)
- Podman or Docker installed and running
- The Dangerzone container image pulled:
  ```bash
  podman pull ghcr.io/freedomofpress/dangerzone/v1
  # or
  docker pull ghcr.io/freedomofpress/dangerzone/v1
  ```

## Building

```bash
cargo build --release
```

## Usage

Basic usage with podman (default):
```bash
dangerzone --input unsafe.pdf --output safe.pdf
```

Using docker instead of podman:
```bash
dangerzone --input unsafe.pdf --output safe.pdf --use-docker
```

Or using cargo run:
```bash
cargo run -- --input unsafe.pdf --output safe.pdf
```

## How it works

1. **Document to Pixels**: The input document is streamed to a sandboxed container that converts it to pixel data
2. **Pixels to PDF**: The pixel data is streamed to another sandboxed container that reconstructs a safe PDF

Both conversions happen in isolated containers with strict security settings following the Dangerzone security model.

## Security Features

The implementation uses the same security flags as the official Dangerzone:
- `--security-opt no-new-privileges`: Prevents privilege escalation
- `--cap-drop all --cap-add SYS_CHROOT`: Minimal capabilities
- `--network=none`: No network access
- `-u dangerzone`: Run as unprivileged user
- `--rm`: Automatically remove containers after use

## References

- [Dangerzone Project](https://github.com/freedomofpress/dangerzone)
- [Container Security Flags](https://github.com/freedomofpress/dangerzone/blob/main/dangerzone/isolation_provider/container.py)