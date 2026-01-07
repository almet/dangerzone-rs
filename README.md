# dangerzone.rs

[![CI](https://github.com/almet/dangerzone-rs/workflows/CI/badge.svg)](https://github.com/almet/dangerzone-rs/actions)

A minimal Rust implementation of [Dangerzone](https://dangerzone.rocks) that
converts potentially dangerous documents into safe PDFs.

## What does it do?

This tool takes untrusted documents (PDFs, Office files, images, etc.) and
converts them into safe PDFs by:

1. **Converting to pixels**: Documents are rendered to raw RGB pixel data in a
   sandboxed container
3. **Generating clean PDF**: A new PDF is created from scratch, embedding only
   the pixel data

This process removes any malicious code, macros, or exploits that may be hidden
in the original document.

## Why is this a good thing?

The main goal is to have both a library and a small binary (1.2MB) that runs on top of
podman to do conversions. Additionally, unlike other implementations, this this
doesn't rely on external PDF libraries like `muPDF`. Instead, it manually
constructs PDFs following the PDF 1.4 specification.

## Usage

### Command-line Interface

Basic conversion:
```bash
dangerzone-rs --input unsafe.pdf --output safe.pdf
```

With OCR:
```bash
dangerzone-rs --input unsafe.pdf --output safe.pdf --ocr
```

**Note on OCR**:

- On **macOS**, the tool uses PDFKit's built-in `saveTextFromOCROption` for
  OCR, which is faster and doesn't require additional dependencies.
- On **other platforms**, OCR can be enabled by installing `ocrmypdf`:
  ```bash
  pip install ocrmypdf
  ```

### Python Library

Use dangerzone-rs as a Python library to programmatically convert documents.

#### Installation

As this is not published to PyPI, here is how to run it locally:

```bash
cargo build --release
uv tool install maturin
uv venv
uv pip install -e . # this will install 
pip install dangerzone-rs
```

#### Basic Usage

Run the demos like this:

```bash
source .venv/bin/activate
python demo/demo.py
```

#### Requirements

- **Podman**: The container runtime (required for document conversion)
- **Dangerzone container image**:
  ```bash
  podman pull ghcr.io/freedomofpress/dangerzone/v1
  ```
- **ocrmypdf** (optional): For OCR on non-macOS platforms:
  ```bash
  pip install ocrmypdf
  ```

## Prerequisites (CLI)

- Rust (for building the binary from source)
- Podman
- Dangerzone container image:
  ```bash
  podman pull ghcr.io/freedomofpress/dangerzone/v1
  ```

## Installation

### CLI Binary

#### Download pre-built binaries

Download the latest release for your platform from the [Releases
page](https://github.com/almet/dangerzone-rs/releases).

Available platforms:

- Linux (x86_64, ARM64)
- macOS (Intel x86_64, Apple Silicon ARM64)
- Windows (x86_64)

#### Build from Source

```bash
cargo build --release
./target/release/dangerzone-rs --input unsafe.pdf --output safe.pdf
```

## How it works

The container converts documents to a binary stream:

- Page count (2 bytes, big-endian)
- For each page: width (2 bytes), height (2 bytes), RGB pixels (3 bytes per pixel)

The Rust code parses this stream and generates a minimal PDF that contains only
the pixel data as uncompressed RGB images. No external PDF library needed.
